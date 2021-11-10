use crate::Render::Renderer;
use anyhow::Result;
use std::time::Duration;
use winit::event::*;

/// Represents a application with reactive state.
pub trait State: Sized + 'static {
    fn Init(renderer: &Renderer) -> Result<Self>;
    fn Input(&mut self, renderer: &Renderer, event: &WindowEvent) -> bool;
    fn Update(&mut self, renderer: &Renderer, delta: Duration);
    fn Resize(&mut self, renderer: &Renderer);
    fn Draw(&mut self, renderer: &mut Renderer) -> Result<(), wgpu::SurfaceError>;
}
