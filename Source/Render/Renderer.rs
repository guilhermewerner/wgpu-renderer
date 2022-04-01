use super::{IndexBuffer, UniformBuffer, VertexBuffer};
use crate::Shader::Shader;
use anyhow::Result;
use wgpu::util::DeviceExt;
use winit::window::Window;

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub window: Window,
    pub config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Renderer {
    pub async fn New(window: Window) -> Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::DX12);
        let surface = unsafe { instance.create_surface(&window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            window,
            config,
            device,
            queue,
        })
    }

    pub fn SubmitShader(&self, shader: &Shader) -> wgpu::ShaderModule {
        let src = shader.source.WgslToString().unwrap();

        self.device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some(&shader.label),
                source: wgpu::ShaderSource::Wgsl(src.as_str().into()),
            })
    }

    pub fn SubmitVertexBuffer(&self, vertex_buffer: &VertexBuffer) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&vertex_buffer.label),
                contents: vertex_buffer.content.as_ref(),
                usage: wgpu::BufferUsages::VERTEX,
            })
    }

    pub fn SubmitIndexBuffer(&self, index_buffer: &IndexBuffer) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&index_buffer.label),
                contents: index_buffer.content.as_ref(),
                usage: wgpu::BufferUsages::INDEX,
            })
    }

    pub fn SubmitUniformBuffer(&self, uniform_buffer: &UniformBuffer) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&uniform_buffer.label),
                contents: uniform_buffer.content.as_ref(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }

    pub fn Draw(
        &self,
        pipeline: &wgpu::RenderPipeline,
        vertex_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        num_indices: u32,
    ) -> Result<()> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("RENDER_ENCODER"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("RENDER_PASS"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn Resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }
}
