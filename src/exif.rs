// exif.rs
//
// Copyright (c) 2023 Junpei Kawamoto
//
// This software is released under the MIT License.
//
// http://opensource.org/licenses/mit-license.php

use anyhow::{Context, Result};
use byteorder::{BigEndian, ByteOrder};
use rexif::{ExifData, ExifEntry, ExifTag, IfdEntry, IfdFormat, IfdKind, Namespace, TagValue};

/// Creates exif data consists of the given text as a user command and returns them as Vec<u8>.
pub fn exif(value: &str) -> Result<Vec<u8>> {
    let mut v = vec![85, 78, 73, 67, 79, 68, 69, 0];
    v.append(&mut encode_utf16(value));

    ExifData {
        mime: "image/tiff",
        entries: vec![
            ExifEntry {
                namespace: Namespace::Standard,
                ifd: IfdEntry {
                    namespace: Namespace::Standard,
                    tag: ExifTag::ExifOffset as u16,
                    format: IfdFormat::U32,
                    count: 1,
                    data: vec![0, 0, 0, 26],
                    // ifd_data: vec![0, 0, 0, 26],
                    ifd_data: Default::default(),
                    ext_data: vec![],
                    le: false,
                },
                tag: ExifTag::ExifOffset,
                value: TagValue::U32(vec![26]),
                // unit: Cow::Borrowed("byte offset"),
                // value_more_readable: Cow::Borrowed("26"),
                unit: Default::default(),
                value_more_readable: Default::default(),
                kind: IfdKind::Ifd0,
            },
            ExifEntry {
                namespace: Namespace::Standard,
                ifd: IfdEntry {
                    namespace: Namespace::Standard,
                    tag: ExifTag::UserComment as u16,
                    format: IfdFormat::Undefined,
                    count: v.len() as u32,
                    data: v.clone(),
                    // ifd_data: vec![0, 0, 0, 40],
                    // ext_data: v.clone(),
                    ifd_data: Default::default(),
                    ext_data: Default::default(),
                    le: false,
                },
                tag: ExifTag::UserComment,
                // value: TagValue::Undefined(v.clone(), false),
                value: TagValue::Undefined(Vec::new(), false),
                // unit: Cow::Borrowed("none"),
                // value_more_readable: Cow::Owned(value.clone()),
                unit: Default::default(),
                value_more_readable: Default::default(),
                kind: IfdKind::Exif,
            },
        ],
        le: false,
    }.serialize().context("failed to serialize exif data")
}


/// Encode the given text in UTF16 and returns it as Vec<u8>.
fn encode_utf16(v: &str) -> Vec<u8> {
    let src: Vec<u16> = v.encode_utf16().collect();
    let mut dest = vec![0; src.len() * 2];
    BigEndian::write_u16_into(src.as_slice(), &mut dest);
    dest
}