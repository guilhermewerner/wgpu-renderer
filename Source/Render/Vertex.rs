use super::VertexBufferLayout;

pub trait Vertex {
    fn GetLayout() -> VertexBufferLayout;
}
