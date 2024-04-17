
use glam::{Mat2, Mat3, Mat4, Vec2, Vec3, Vec4};

use crate::buffers::uniform::UniformBufferData;

// implement uniform buffer data trait, to ease use
// with uniform buffers
impl UniformBufferData for Vec2 {}
impl UniformBufferData for Vec3 {}
impl UniformBufferData for Vec4 {}

impl UniformBufferData for Mat2 {}
impl UniformBufferData for Mat3 {}
impl UniformBufferData for Mat4 {}


// #[rustfmt::skip]
// pub const OPENGL_TO_WGPU_MATRIX: = ::new(
//     1.0, 0.0, 0.0, 0.0,
//     0.0, 1.0, 0.0, 0.0,
//     0.0, 0.0, 0.5, 0.5,
//     0.0, 0.0, 0.0, 1.0,
// );
