use std::io::{BufRead, Cursor, Read, Seek, SeekFrom, Write};

use anyhow::{bail, Context, Result};
use byteorder::ByteOrder;
use png::Decoder;
use riff::{Chunk, ChunkContents, ChunkId};

use crate::exif::exif;

/// Reads the given PNG image and returns the generation parameters if the image contains them.
fn parameters<R: Read>(r: &mut R) -> Result<Option<String>> {
    let reader = Decoder::new(r).read_info()?;
    for c in &reader.info().uncompressed_latin1_text {
        if c.keyword == "parameters" {
            return Ok(Some(c.text.to_string()));
        }
    }
    Ok(None)
}

/// Creates a chunk ID representing the given str.
fn chunk_id(s: &str) -> Result<ChunkId> {
    ChunkId::new(s).or_else(|e| bail!("failed to create a chunk ID: {}", e))
}

/// Creates a VP8X chunk with the given width and height.
fn vp8x_chunk(w: u32, h: u32) -> Result<ChunkContents> {
    let mut data = vec![8, 0, 0, 0];

    let mut buf = [0; 4];
    byteorder::LittleEndian::write_u32(&mut buf, w - 1);
    data.extend(&buf[..3]);

    byteorder::LittleEndian::write_u32(&mut buf, h - 1);
    data.extend(&buf[..3]);

    Ok(ChunkContents::Data(chunk_id("VP8X")?, data))
}

/// Creates an EXIF chunk consisting of the given comment.
fn exif_chunk(comment: &str) -> Result<ChunkContents> {
   Ok(ChunkContents::Data(chunk_id("EXIF")?, exif(comment)?))
}

/// Converts the given image with r and writes the result via w.
pub fn convert<R, W>(mut r: R, mut w: W) -> Result<()>
    where R: BufRead + Seek, W: Write + Seek
{
    let img = image::io::Reader::new(&mut r).with_guessed_format()?.decode()?;
    let webp = match webp::Encoder::from_image(&img) {
        Ok(v) => v,
        Err(e) => bail!("failed to create webp encoder: {}", e)
    }.encode_lossless();

    r.seek(SeekFrom::Start(0))?;
    match parameters(&mut r)? {
        None => w.write_all(&webp).context("failed to write an image"),
        Some(p) => {
            let mut stream = Cursor::new(webp.as_ref());
            let root = Chunk::read(&mut stream, 0)?;

            let bitstream = root.iter(&mut stream).find(|c| {
                let id = c.id();
                let ids = id.as_str();
                ids == "VP8 " || ids == "VP8L"
            }).context("no bitstreams are found")?;
            ChunkContents::Children(
                ChunkId { value: [0x52, 0x49, 0x46, 0x46] },
                ChunkId { value: [0x57, 0x45, 0x42, 0x50] },
                vec![
                    vp8x_chunk(img.width(), img.height())?,
                    ChunkContents::Data(
                        bitstream.id(),
                        bitstream.read_contents(&mut stream)?,
                    ),
                    exif_chunk(&p)?,
                ],
            ).write(&mut w)?;
            Ok(())
        }
    }
}
