#![allow(non_snake_case)]
#![allow(unused_variables)]

use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use std::time::Duration;
use winit::event::*;
use Graphics::Render::*;
use Graphics::Shader::*;
use Graphics::{Runtime, State};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct TriangleVertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex for TriangleVertex {
    fn GetLayout() -> VertexBufferLayout {
        VertexBufferLayout {
            label: "Triangle".into(),
            stride: std::mem::size_of::<TriangleVertex>(),
            step_mode: StepMode::Vertex,
            attributes: vec![
                VertexAttribute {
                    label: "Position".into(),
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    label: "Color".into(),
                    format: VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 3]>(),
                    shader_location: 1,
                },
            ],
        }
    }
}

#[rustfmt::skip]
const VERTICES: &[TriangleVertex] = &[
    TriangleVertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    TriangleVertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    TriangleVertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 2,
];

struct Triangle {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl State for Triangle {
    fn Init(renderer: &Renderer) -> Result<Self> {
        // Shader

        let shader = Shader::FromWgsl(include_str!("../Shaders/Triangle.wgsl"));
        let shader_module = renderer.SubmitShader(&shader);

        // Pipeline

        let render_pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("RENDER_PIPELINE_LAYOUT"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let render_pipeline = {
            let mut layout = TriangleVertex::GetLayout();

            let attributes = layout
                .attributes
                .drain(..)
                .map(|x| x.into())
                .collect::<Vec<_>>();

            let wgpu_layout = wgpu::VertexBufferLayout {
                array_stride: layout.stride as wgpu::BufferAddress,
                step_mode: layout.step_mode.into(),
                attributes: attributes.as_ref(),
            };

            renderer
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("RENDER_PIPELINE"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader_module,
                        entry_point: "main",
                        buffers: &[wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<TriangleVertex>()
                                as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[
                                wgpu::VertexAttribute {
                                    offset: 0,
                                    shader_location: 0,
                                    format: wgpu::VertexFormat::Float32x3,
                                },
                                wgpu::VertexAttribute {
                                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                    shader_location: 1,
                                    format: wgpu::VertexFormat::Float32x3,
                                },
                            ],
                        }],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader_module,
                        entry_point: "main",
                        targets: &[wgpu::ColorTargetState {
                            format: renderer.config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        }],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        clamp_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                })
        };

        let vertex_buffer = renderer.SubmitVertexBuffer(&VertexBuffer {
            label: "Vertex Buffer".into(),
            content: bytemuck::cast_slice(VERTICES).to_vec(),
        });

        let index_buffer = renderer.SubmitIndexBuffer(&IndexBuffer {
            label: "Index Buffer".into(),
            content: bytemuck::cast_slice(INDICES).to_vec(),
        });

        let num_indices = INDICES.len() as u32;

        Ok(Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        })
    }

    fn Input(&mut self, renderer: &Renderer, event: &WindowEvent) -> bool {
        false
    }

    fn Update(&mut self, renderer: &Renderer, delta: Duration) {}

    fn Resize(&mut self, renderer: &Renderer) {}

    fn Draw(&mut self, renderer: &mut Renderer) -> Result<(), wgpu::SurfaceError> {
        renderer
            .Draw(
                &self.render_pipeline,
                &self.vertex_buffer,
                &self.index_buffer,
                self.num_indices,
            )
            .unwrap();

        Ok(())
    }
}

fn main() -> Result<()> {
    Runtime::Execute::<Triangle>()
}
