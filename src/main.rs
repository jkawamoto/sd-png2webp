use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;

mod exif;
mod image;

/// Converts the given file in WebP image and saves with .webp extension.
fn convert(p: &Path) -> Result<()> {
    println!("converting {}...", p.to_string_lossy());
    image::convert(
        BufReader::new(File::open(p)?),
        File::create(p.with_extension("webp"))?,
    )
}

/// Converts the given file in WebP image. If the given path represents a directory,
/// this function walks into the directory and converts all images.
fn convert_all<T: AsRef<Path>>(p: T) {
    let p = p.as_ref();
    if p.is_dir() {
        match p.read_dir() {
            Ok(v) => {
                for r in v {
                    match r {
                        Ok(e) => convert_all(e.path()),
                        Err(e) => println!("failed to read a directory: {}", e)
                    }
                }
            }
            Err(e) => println!("failed to read {}: {}", p.to_string_lossy(), e)
        }
        return;
    }

    if p.extension().unwrap_or("".as_ref()) != "png" {
        return;
    }
    if let Err(e) = convert(p) {
        println!("failed to convert {}: {}", p.to_string_lossy(), e);
    }
}


fn main() {
    for p in env::args().skip(1) {
        convert_all(p);
    }
}


