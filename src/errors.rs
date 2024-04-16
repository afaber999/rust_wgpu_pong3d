use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum WvtError {

    #[error("IO error")]
    IoError(#[from] io::Error),

    #[error("wgpu error")]
    WgpuError(#[from] wgpu::Error),


    
    #[error("Image loading/handling error")]
    ImageError(#[from] image::error::ImageError),


    #[error("Unknown wvt error")]
    Unknown,
}