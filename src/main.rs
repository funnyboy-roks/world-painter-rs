#[macro_use]
extern crate carlog;
use std::collections::BTreeMap;

use anyhow::Context;
use carlog::prelude::*;

mod cli;
mod util;
use cli::Cli;
use color_art::Color;
use image::RgbImage;
use mca_parser::RegionPosition;
use progress_bar as bar;
use util::MinMax;

use crate::util::map_num;

fn main() {
    match run() {
        Ok(_) => {}
        Err(err) => {
            carlog_error!(format!("{:?}", err));
        }
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse()?;

    // Get the actual height from worldborders
    let radius = cli
        .files
        .iter()
        .map(|f| f.worldborder)
        .max()
        .context("Couldn't get a max worldborder")?;

    // Get height in terms of regions
    let radius = (radius as f64 / (32.0 * 16.0)).ceil() as u32;
    //let radius = radius * 32;

    // height is 2 * radius
    let height = radius * 2;

    // Convert height to use chunks and thus the actual height of the image
    let height = height * 32;

    const PADDING: u32 = 5 * 32;
    // Add the padding around the image
    let height = height + PADDING * 2;

    let mut img = RgbImage::new(height * cli.files.len() as u32, height);

    for (i, file) in cli.files.iter().enumerate() {
        let mut regions = mca_parser::from_directory(file.path.clone())?;
        let x_offset = i as u32 * height;

        // Radius from the centre (0, 0) which to render (rectangular radius)
        let radius = radius as i32;

        for x in 0..img.height() {
            for y in 0..PADDING {
                img.put_pixel(x_offset + x, y, util::color_to_rgb(file.color));
                img.put_pixel(x_offset + x, height - y - 1, util::color_to_rgb(file.color));
                img.put_pixel(x_offset + y, x, util::color_to_rgb(file.color));
                img.put_pixel(x_offset + height - y - 1, x, util::color_to_rgb(file.color));
            }
        }

        carlog_info!(
            "Reading",
            format!(
                "Parsing and drawing region files from {}",
                file.path.display()
            )
        );
        bar::init_progress_bar(regions.regions.len());
        bar::set_progress_bar_action("Parsing", bar::Color::Cyan, bar::Style::Bold);
        let rgs: BTreeMap<RegionPosition, Vec<_>> = regions
            .regions
            .iter_mut()
            .filter_map(|(k, v)| {
                bar::inc_progress_bar();
                let rg = v.parse_without_data().ok()?;
                let c = rg
                    .chunks
                    .iter()
                    .map(|c| c.clone().map(|c| c.payload.length))
                    .collect();
                Some((k.clone(), c))
            })
            .collect();
        bar::finalize_progress_bar();

        let mut chunks_minmax = rgs
            .values()
            .map(|c| c.iter().filter_map(|c| *c))
            .flatten()
            .min_max();

        if chunks_minmax.1 > 10000 {
            chunks_minmax.1 = 10000;
        }

        //let bar = Bar::new(radius as u64 * radius as u64 * 4, Config::cargo());
        bar::init_progress_bar((radius * radius) as usize * 4);
        bar::set_progress_bar_action("Drawing", bar::Color::Cyan, bar::Style::Bold);
        let wb = file.worldborder as i32;
        let wb = (wb as f64 / (32.0 * 16.0)).ceil() as i32;
        for x in -radius..radius {
            for y in -radius..radius {
                bar::inc_progress_bar();
                if x >= -wb && x <= wb && y >= -wb && y <= wb {
                    if let Some(chunks) = rgs.get(&RegionPosition::new(x, y)) {
                        let x = x + radius;
                        let y = y + radius;
                        assert!(x >= 0);
                        assert!(y >= 0);

                        let x = x * 32 + PADDING as i32;
                        let y = y * 32 + PADDING as i32;
                        for chunk_x in 0..32 {
                            for chunk_z in 0..32 {
                                let len = chunks[((chunk_x & 31) + (chunk_z & 31) * 32) as usize];
                                if let Some(len) = len {
                                    let c = map_num(
                                        len.min(chunks_minmax.1) as f64,
                                        chunks_minmax.0 as f64,
                                        chunks_minmax.1 as f64,
                                        360.0 * 0.25,
                                        0.0,
                                    );
                                    let c = if c.is_nan() { 0.0 } else { c };
                                    let c = Color::from_hsv(c, 1.0, 1.0)?;
                                    img.put_pixel(
                                        x_offset + x as u32 + chunk_x,
                                        y as u32 + chunk_z,
                                        util::color_to_rgb(c),
                                    );
                                } else {
                                    img.put_pixel(
                                        x_offset + x as u32 + chunk_x,
                                        y as u32 + chunk_z,
                                        util::color_to_rgb(file.color),
                                    );
                                }
                            }
                        }
                    } else {
                        let x = x + radius;
                        let y = y + radius;
                        assert!(x >= 0);
                        assert!(y >= 0);

                        let x = x as u32 * 32 + PADDING;
                        let y = y as u32 * 32 + PADDING;

                        for x_off in 0..32 {
                            for y_off in 0..32 {
                                img.put_pixel(
                                    x_offset + x as u32 + x_off,
                                    y as u32 + y_off,
                                    util::color_to_rgb(file.color),
                                );
                            }
                        }
                    }
                } else {
                    let x = x + radius;
                    let y = y + radius;
                    assert!(x >= 0);
                    assert!(y >= 0);

                    let x = x as u32 * 32 + PADDING;
                    let y = y as u32 * 32 + PADDING;

                    for x_off in 0..32 {
                        for y_off in 0..32 {
                            img.put_pixel(
                                x_offset + x as u32 + x_off,
                                y as u32 + y_off,
                                util::color_to_rgb(file.color),
                            );
                        }
                    }
                }
            }
        }

        bar::finalize_progress_bar();
    }
    img.save("out.png")?;
    carlog_ok!("Saving", "Image saved");

    Ok(())
}
