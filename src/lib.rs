mod parser;

#[macro_use]
extern crate serde;

use glam::Vec3;
pub use parser::parse;
use serde::Deserialize;

pub struct OpdFile {
    pub header: OpdHeader,
    pub centroids: Vec<Centroid>,
    pub frames: Frames,
}

#[derive(Deserialize, Debug)]
pub struct OpdHeader {
    pub version: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub directive: OpdHeaderDirective,
}

#[derive(Clone)]
pub struct Frame<T> {
    pub time: f32,
    pub data: Vec<T>,
}

#[derive(Clone)]
pub enum Frames {
    I8(Vec<Frame<i8>>),
    I16(Vec<Frame<i16>>),
    I32(Vec<Frame<i32>>),
    I64(Vec<Frame<i64>>),
}

#[derive(Deserialize, Debug)]
pub struct FrameMeta {
    pub time: f32,
    pub offset: usize,
}

#[derive(Deserialize, Debug)]
pub struct OpdHeaderDirective {
    pub version: String,
    pub meta: OpdHeaderDirectiveMeta,

    #[serde(rename = "numCentroids")]
    pub num_centroids: usize,

    pub origin: OpdHeaderDirectiveOrigin,

    pub precision: usize,
    pub scale: Vec3,
    pub frames: Vec<FrameMeta>,
}

#[derive(Deserialize, Debug)]
pub struct OpdHeaderDirectiveOrigin {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Deserialize, Debug)]
pub struct OpdHeaderDirectiveMeta {
    #[serde(rename = "projectId")]
    pub project_id: String,

    #[serde(rename = "projectName")]
    pub project_name: String,
}

pub struct Centroid {
    pub parent_id: u32,

    /// Relative to origin defined in header
    pub offset: Vec3,
}
