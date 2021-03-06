//
// This file is a self contained render pipeline responsible for showing the loading screen before other assets are loaded.
// This is separate so it can be ran before the other services are setup
//

use wgpu::{SwapChain, Device, RenderPipeline, Queue, ShaderModule};
use winit::dpi::PhysicalSize;

pub struct LoadingScreen {
    pipeline: RenderPipeline
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LoadingVertices {
    pub position: [f32; 2]
}

impl LoadingVertices {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<LoadingVertices>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float2,
                }
            ]
        }
    }
}

impl LoadingScreen {
    pub fn new(device: &Device, size: &PhysicalSize<u32>) -> LoadingScreen {
        LoadingScreen { pipeline: LoadingScreen::generate_loading_render_pipeline(device, size) }
    }

    pub fn render(&mut self, swapchain: &mut SwapChain, device: &Device, queue: &mut Queue, percentage: u8) {

        let x = ((percentage as f32 / 100.0) * 1.4) - 0.7;

        let top_left = LoadingVertices {
            position: [-0.7, 0.4],
        };
        let top_right = LoadingVertices {
            position: [x, 0.4],
        };
        let bottom_left = LoadingVertices {
            position: [-0.7, 0.7],
        };
        let bottom_right = LoadingVertices {
            position: [x, 0.7],
        };

        // Create loading
        let mut vertices = vec![top_left, bottom_right, bottom_left, top_left, top_right, bottom_right];

        let vertices_buffer = device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(vertices.as_mut_slice());

        let frame = swapchain.get_next_texture();

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            todo: 0,
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        },
                    }
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffers(0, &[(&vertices_buffer, 0)]);
            render_pass.draw(0..vertices.len() as u32, 0..1)
        }

        queue.submit(&[
            encoder.finish()
        ]);
    }

    fn generate_loading_render_pipeline(device: &Device, size: &PhysicalSize<u32>) -> RenderPipeline {

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Vsync,
        };

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[],
        });

        let (vs_module, fs_module) = load_shaders(&device);

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[
                LoadingVertices::desc(),
            ],
            layout: &render_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            color_states: &[
                wgpu::ColorStateDescriptor {
                    format: sc_desc.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                },
            ],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: None,
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false
        })
    }
}

pub fn load_shaders(device: &Device) -> (ShaderModule, ShaderModule){
    let vs_src = include_str!("../render/shaders/loading.vert");
    let fs_src = include_str!("../render/shaders/loading.frag");

    let vs_spirv = glsl_to_spirv::compile(vs_src, glsl_to_spirv::ShaderType::Vertex).unwrap();
    let fs_spirv = glsl_to_spirv::compile(fs_src, glsl_to_spirv::ShaderType::Fragment).unwrap();

    let vs_data = wgpu::read_spirv(vs_spirv).unwrap();
    let fs_data = wgpu::read_spirv(fs_spirv).unwrap();

    let vs_module = device.create_shader_module(&vs_data);
    let fs_module = device.create_shader_module(&fs_data);

    (vs_module, fs_module)
}

// pub fn load_font<'a>(device: &Device, path: &str) -> GlyphBrush<'a, (), DefaultSectionHasher> {
//     let mut buf = Vec::new();
//     let mut file = File::open(format!("{}{}", path, "/resources/default.ttf")).expect(&format!("Font not found! {}{}", path, "/default.ttf"));
//     file.read_to_end(&mut buf);
//
//     GlyphBrushBuilder::using_font_bytes(buf).expect("Error in font")
//         .texture_filter_method(wgpu::FilterMode::Nearest)
//         .build(device, wgpu::TextureFormat::Bgra8UnormSrgb)
// }