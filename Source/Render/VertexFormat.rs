use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum VertexFormat {
    /// Two unsigned bytes (u8). `uvec2` in shaders.
    UInt8x2 = 0,

    /// Four unsigned bytes (u8). `uvec4` in shaders.
    UInt8x4 = 1,

    /// Two signed bytes (i8). `ivec2` in shaders.
    Int8x2 = 2,

    /// Four signed bytes (i8). `ivec4` in shaders.
    Int8x4 = 3,

    /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2` in shaders.
    UNorm8x2 = 4,

    /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4` in shaders.
    UNorm8x4 = 5,

    /// Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2` in shaders.
    Norm8x2 = 6,

    /// Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4` in shaders.
    Norm8x4 = 7,

    /// Two unsigned shorts (u16). `uvec2` in shaders.
    UInt16x2 = 8,

    /// Four unsigned shorts (u16). `uvec4` in shaders.
    UInt16x4 = 9,

    /// Two signed shorts (i16). `ivec2` in shaders.
    Int16x2 = 10,

    /// Four signed shorts (i16). `ivec4` in shaders.
    Int16x4 = 11,

    /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2` in shaders.
    UNorm16x2 = 12,

    /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4` in shaders.
    UNorm16x4 = 13,

    /// Two signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec2` in shaders.
    Norm16x2 = 14,

    /// Four signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec4` in shaders.
    Norm16x4 = 15,

    /// Two half-precision floats (no Rust equiv). `vec2` in shaders.
    Float16x2 = 16,

    /// Four half-precision floats (no Rust equiv). `vec4` in shaders.
    Float16x4 = 17,

    /// One single-precision float (f32). `float` in shaders.
    Float32 = 18,

    /// Two single-precision floats (f32). `vec2` in shaders.
    Float32x2 = 19,

    /// Three single-precision floats (f32). `vec3` in shaders.
    Float32x3 = 20,

    /// Four single-precision floats (f32). `vec4` in shaders.
    Float32x4 = 21,

    /// One unsigned int (u32). `uint` in shaders.
    UInt32 = 22,

    /// Two unsigned ints (u32). `uvec2` in shaders.
    UInt32x2 = 23,

    /// Three unsigned ints (u32). `uvec3` in shaders.
    UInt32x3 = 24,

    /// Four unsigned ints (u32). `uvec4` in shaders.
    UInt32x4 = 25,

    /// One signed int (i32). `int` in shaders.
    Int32 = 26,

    /// Two signed ints (i32). `ivec2` in shaders.
    Int32x2 = 27,

    /// Three signed ints (i32). `ivec3` in shaders.
    Int32x3 = 28,

    /// Four signed ints (i32). `ivec4` in shaders.
    Int32x4 = 29,

    /// One double-precision float (f64). `double` in shaders.
    Float64 = 30,

    /// Two double-precision floats (f64). `dvec2` in shaders.
    Float64x2 = 31,

    /// Three double-precision floats (f64). `dvec3` in shaders.
    Float64x3 = 32,

    /// Four double-precision floats (f64). `dvec4` in shaders.
    Float64x4 = 33,
}

impl VertexFormat {
    /// Returns the size in bytes of this format.
    pub const fn GetSize(&self) -> u64 {
        match *self {
            Self::UInt8x2 => 2,
            Self::UInt8x4 => 4,
            Self::Int8x2 => 2,
            Self::Int8x4 => 4,
            Self::UNorm8x2 => 2,
            Self::UNorm8x4 => 4,
            Self::Norm8x2 => 2,
            Self::Norm8x4 => 4,
            Self::UInt16x2 => 2 * 2,
            Self::UInt16x4 => 4 * 4,
            Self::Int16x2 => 2 * 2,
            Self::Int16x4 => 2 * 4,
            Self::UNorm16x2 => 2 * 2,
            Self::UNorm16x4 => 2 * 4,
            Self::Norm16x2 => 2 * 2,
            Self::Norm16x4 => 2 * 4,
            Self::Float16x2 => 2 * 2,
            Self::Float16x4 => 2 * 4,
            Self::Float32 => 4,
            Self::Float32x2 => 4 * 2,
            Self::Float32x3 => 4 * 3,
            Self::Float32x4 => 4 * 4,
            Self::UInt32 => 4,
            Self::UInt32x2 => 4 * 2,
            Self::UInt32x3 => 4 * 3,
            Self::UInt32x4 => 4 * 4,
            Self::Int32 => 4,
            Self::Int32x2 => 4 * 2,
            Self::Int32x3 => 4 * 3,
            Self::Int32x4 => 4 * 4,
            Self::Float64 => 8,
            Self::Float64x2 => 8 * 2,
            Self::Float64x3 => 8 * 3,
            Self::Float64x4 => 8 * 4,
        }
    }
}

impl From<VertexFormat> for wgpu::VertexFormat {
    fn from(format: VertexFormat) -> Self {
        match format {
            VertexFormat::UInt8x2 => wgpu::VertexFormat::Uint8x2,
            VertexFormat::UInt8x4 => wgpu::VertexFormat::Uint8x4,
            VertexFormat::Int8x2 => wgpu::VertexFormat::Sint8x2,
            VertexFormat::Int8x4 => wgpu::VertexFormat::Sint8x4,
            VertexFormat::UNorm8x2 => wgpu::VertexFormat::Unorm8x2,
            VertexFormat::UNorm8x4 => wgpu::VertexFormat::Unorm8x4,
            VertexFormat::Norm8x2 => wgpu::VertexFormat::Snorm8x2,
            VertexFormat::Norm8x4 => wgpu::VertexFormat::Snorm8x4,
            VertexFormat::UInt16x2 => wgpu::VertexFormat::Uint16x2,
            VertexFormat::UInt16x4 => wgpu::VertexFormat::Uint16x4,
            VertexFormat::Int16x2 => wgpu::VertexFormat::Sint16x2,
            VertexFormat::Int16x4 => wgpu::VertexFormat::Sint16x4,
            VertexFormat::UNorm16x2 => wgpu::VertexFormat::Unorm16x2,
            VertexFormat::UNorm16x4 => wgpu::VertexFormat::Unorm16x4,
            VertexFormat::Norm16x2 => wgpu::VertexFormat::Snorm16x2,
            VertexFormat::Norm16x4 => wgpu::VertexFormat::Snorm16x4,
            VertexFormat::Float16x2 => wgpu::VertexFormat::Float16x2,
            VertexFormat::Float16x4 => wgpu::VertexFormat::Float16x4,
            VertexFormat::Float32 => wgpu::VertexFormat::Float32,
            VertexFormat::Float32x2 => wgpu::VertexFormat::Float32x2,
            VertexFormat::Float32x3 => wgpu::VertexFormat::Float32x3,
            VertexFormat::Float32x4 => wgpu::VertexFormat::Float32x4,
            VertexFormat::UInt32 => wgpu::VertexFormat::Uint32,
            VertexFormat::UInt32x2 => wgpu::VertexFormat::Uint32x2,
            VertexFormat::UInt32x3 => wgpu::VertexFormat::Uint32x3,
            VertexFormat::UInt32x4 => wgpu::VertexFormat::Uint32x4,
            VertexFormat::Int32 => wgpu::VertexFormat::Sint32,
            VertexFormat::Int32x2 => wgpu::VertexFormat::Sint32x2,
            VertexFormat::Int32x3 => wgpu::VertexFormat::Sint32x3,
            VertexFormat::Int32x4 => wgpu::VertexFormat::Sint32x4,
            VertexFormat::Float64 => wgpu::VertexFormat::Float64,
            VertexFormat::Float64x2 => wgpu::VertexFormat::Float64x2,
            VertexFormat::Float64x3 => wgpu::VertexFormat::Float64x3,
            VertexFormat::Float64x4 => wgpu::VertexFormat::Float64x4,
        }
    }
}
