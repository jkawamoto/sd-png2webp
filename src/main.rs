// main.rs
//
// Copyright (c) 2023 Junpei Kawamoto
//
// This software is released under the MIT License.
//
// http://opensource.org/licenses/mit-license.php

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::Result;
use color_print::cprintln;
use crossbeam::sync::WaitGroup;
use scheduled_thread_pool::ScheduledThreadPool;

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
fn convert_all(p: PathBuf, pool: &ScheduledThreadPool, wg: &WaitGroup) {
    if p.is_dir() {
        match p.read_dir() {
            Ok(v) => {
                for r in v {
                    match r {
                        Ok(e) => convert_all(e.path(), pool, wg),
                        Err(e) => cprintln!("<y>failed to read a directory: {}</y>", e),
                    }
                }
            }
            Err(e) => cprintln!("<y>failed to read {}: {}</y>", p.to_string_lossy(), e),
        }
        return;
    }

    if p.extension().unwrap_or("".as_ref()) != "png" {
        return;
    }

    let wg = wg.clone();
    _ = pool.execute(move || {
        if let Err(e) = convert(&p) {
            cprintln!("<y>failed to convert {}: {}</y>", p.to_string_lossy(), e);
        }
        drop(wg);
    })
}

fn main() {
    let wg = WaitGroup::new();
    let pool = ScheduledThreadPool::new(num_cpus::get() / 2 + 1);
    for p in env::args().skip(1) {
        convert_all(PathBuf::from(p), &pool, &wg);
    }
    wg.wait();
}
