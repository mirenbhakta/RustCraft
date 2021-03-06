use crate::render::RenderState;
use std::time::{Instant};
use crate::services::ui_service::UIService;

pub mod uniforms;

impl RenderState {
    pub fn render(&mut self) {

        self.update();

        let mut swapchain = self.swap_chain.take().unwrap();
        let mut services = self.services.take().unwrap();

        let frame = swapchain.get_next_texture();

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            todo: 0,
        });

        {
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[
                        wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            },
                        }
                    ],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                        attachment: &self.depth_texture.1,
                        depth_load_op: wgpu::LoadOp::Clear,
                        depth_store_op: wgpu::StoreOp::Store,
                        clear_depth: 1.0,
                        stencil_load_op: wgpu::LoadOp::Load,
                        stencil_store_op: wgpu::StoreOp::Store,
                        clear_stencil: 0,
                    }),
                });

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &services.asset.atlas_bind_group.as_ref().unwrap(), &[]);
                render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);

                for chunk in &services.chunk.chunks {
                    let indices_buffer = chunk.1.indices_buffer.as_ref().unwrap();
                    let vertices_buffer = chunk.1.vertices_buffer.as_ref().unwrap();
                    let model_bind_group = chunk.1.model_bind_group.as_ref().unwrap();

                    render_pass.set_bind_group(2, model_bind_group, &[0]);
                    render_pass.set_vertex_buffers(0, &[(vertices_buffer, 0)]);
                    render_pass.set_index_buffer(indices_buffer, 0);
                    render_pass.draw_indexed(0..chunk.1.indices_buffer_len, 0, 0..1);
                }
            }

            // Debug information

            UIService::render(&frame, &mut encoder, &self.device, &mut services);

            self.queue.submit(&[
                encoder.finish()
            ]);
        }

        std::mem::drop(frame);

        self.swap_chain = Some(swapchain);
        self.services = Some(services);
    }

    pub fn update(&mut self) {
        // Update fps
        if Instant::now().duration_since(self.fps_counter).as_secs_f32() >= 1.0 {
            self.fps = self.frames;
            self.frames = 0;
            self.fps_counter = Instant::now();
        }
        self.frames += 1;
    }
}