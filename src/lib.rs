mod parser;

pub use glam::Vec3;
pub use parser::parse;
use serde::{Deserialize, Serialize};

pub struct OpdFile {
    pub header: OpdHeader,
    pub centroids: Vec<Centroid>,
    pub frames: Frames,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct OpdHeader {
    pub version: String,
    pub compressed: Option<String>,
    #[serde(rename = "type")]
    pub ty: String,
    pub directive: OpdHeaderDirective,
}

#[derive(Clone)]
pub struct Frame<T> {
    pub time: f32,
    pub data: Vec<T>,
}

impl<T: Copy> Frame<T> {
    pub fn frame_as_vec3a(&self) -> impl Iterator<Item = Vec3> + '_
    where
        T: Into<f32>,
    {
        let iter = self.data.iter();
        FrameIterator { iter }
    }
}
pub struct FrameIterator<'a, T> {
    iter: std::slice::Iter<'a, T>,
}
impl<'a, T: Copy> Iterator for FrameIterator<'a, T>
where
    T: Into<f32>,
{
    type Item = Vec3;
    fn next(&mut self) -> Option<Self::Item> {
        let max_value: usize = (1 << (std::mem::size_of::<T>() * 8 - 1)) - 1;
        let max_value = max_value as f32;
        let arr: [f32; 3] = [
            match self.iter.next() {
                Some(a) => (*a).into() / max_value,
                None => return None,
            },
            match self.iter.next() {
                Some(a) => (*a).into() / max_value,
                None => return None,
            },
            match self.iter.next() {
                Some(a) => (*a).into() / max_value,
                None => return None,
            },
        ];
        Some(arr.into())
    }
}

#[derive(Clone)]
pub enum Frames {
    I8(Vec<Frame<i8>>),
    I16(Vec<Frame<i16>>),
    I32(Vec<Frame<i32>>),
    I64(Vec<Frame<i64>>),
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct FrameMeta {
    pub time: f32,
    pub offset: usize,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct OpdHeaderDirective {
    pub version: String,
    pub meta: OpdHeaderDirectiveMeta,

    #[serde(rename = "numCentroids")]
    pub num_centroids: Option<usize>,

    #[serde(rename = "numPoints")]
    pub num_points: Option<usize>,

    pub origin: OpdHeaderDirectiveOrigin,

    pub precision: usize,
    pub scale: Vec3,
    pub frames: Vec<FrameMeta>,

    pub index: Option<bool>,
    #[serde(rename = "subCentroids")]
    pub sub_centroids: Option<bool>,

    #[serde(rename = "lastFrameCorrected")]
    pub last_frame_corrected: Option<bool>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct OpdHeaderDirectiveOrigin {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl From<OpdHeaderDirectiveOrigin> for Vec3 {
    fn from(value: OpdHeaderDirectiveOrigin) -> Self {
        Vec3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
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
