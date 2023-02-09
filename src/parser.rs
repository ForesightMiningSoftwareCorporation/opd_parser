use glam::Vec3;
use nom::{
    bytes::complete::tag,
    multi::{count, length_data, many1},
    number::complete::{be_f32, be_u16, be_u32, be_u64, be_u8},
    IResult,
};

use crate::{Centroid, Frame, Frames, OpdFile, OpdHeader};

pub fn parse(input: &[u8]) -> IResult<&[u8], OpdFile> {
    let (input, _) = tag(b".opd".as_slice())(input)?;

    let (input, json_header) = length_data(be_u32)(input)?;
    let header: crate::OpdHeader = serde_json::from_slice(json_header).unwrap();

    println!("{:?}", header);
    let (mut input, centroids) = count(parse_centroid, header.directive.num_centroids)(input)?;

    let _base_offset = header.directive.num_centroids * 4 * 4;
    let _frame_data_len = header.directive.precision * 3;

    let frames = match header.directive.precision {
        1 => {
            let (next_input, frames) = parse_frame(input, &header, be_u8)?;
            input = next_input;
            Frames::U8(frames)
        }
        2 => {
            let (next_input, frames) = parse_frame(input, &header, be_u16)?;
            input = next_input;
            Frames::U16(frames)
        }
        4 => {
            let (next_input, frames) = parse_frame(input, &header, be_u32)?;
            input = next_input;
            Frames::U32(frames)
        }
        8 => {
            let (next_input, frames) = parse_frame(input, &header, be_u64)?;
            input = next_input;
            Frames::U64(frames)
        }
        _ => {
            unimplemented!()
        }
    };

    Ok((
        input,
        OpdFile {
            header,
            centroids,
            frames,
        },
    ))
}

type NumberParser<'a, NUM> = fn(input: &'a [u8]) -> IResult<&'a [u8], NUM>;

pub fn parse_frame<'a, T>(
    mut input: &'a [u8],
    header: &OpdHeader,
    number_parser: NumberParser<'a, T>,
) -> IResult<&'a [u8], Vec<Frame<T>>> {
    assert_eq!(header.directive.precision, std::mem::size_of::<T>());
    let base_offset = header.directive.num_centroids * 4 * 4;

    let mut frames = Vec::with_capacity(header.directive.frames.len());
    for frame in header.directive.frames.windows(2) {
        let start = (frame[0].offset - base_offset) / header.directive.precision;
        let end = (frame[1].offset - base_offset) / header.directive.precision;
        let len = end - start;

        let (new_input, data) = count(number_parser, len as usize)(input)?;
        input = new_input;
        frames.push(Frame {
            time: frame[0].time,
            data,
        });
    }
    if let Some(last_frame) = header.directive.frames.last() {
        let (rest, data) = many1(number_parser)(input)?;
        frames.push(Frame {
            time: last_frame.time,
            data,
        });
        Ok((rest, frames))
    } else {
        Ok((input, frames))
    }
}

pub fn parse_centroid(input: &[u8]) -> IResult<&[u8], Centroid> {
    let (input, parent_id) = be_u32(input)?;
    let (input, x) = be_f32(input)?;
    let (input, y) = be_f32(input)?;
    let (input, z) = be_f32(input)?;
    Ok((
        input,
        Centroid {
            parent_id,
            offset: Vec3::new(x, y, z),
        },
    ))
}
