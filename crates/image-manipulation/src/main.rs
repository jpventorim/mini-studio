use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use image_manipulation::{
    BaseShape, ManipulationOptions, Position, TrimMode, TrimOptions, prepare,
};
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Background {
    Auto,
    Alpha,
    White,
    None,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Shape {
    Rectangle,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Placement {
    Grounded,
    Hovering,
    Flying,
}

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Trim and resize an image into a mirrored paper standee"
)]
struct Args {
    /// Source PNG or JPEG image.
    #[arg(short, long)]
    input: PathBuf,
    /// Destination RGBA PNG, including physical-resolution metadata.
    #[arg(short, long)]
    output: PathBuf,
    /// Base width as a multiplier of the grid size.
    #[arg(long, default_value_t = 1.0)]
    size: f64,
    /// Grid size in millimetres.
    #[arg(long, default_value_t = 25.0)]
    grid_size_mm: f64,
    /// Override total clearance (auto: 1 mm below size 1, 2 mm for sizes 1 and up).
    #[arg(long)]
    base_clearance_mm: Option<f64>,
    /// Target raster resolution.
    #[arg(long, default_value_t = 300.0)]
    dpi: f64,
    /// Base shape added below the artwork, then mirrored with it.
    #[arg(long, value_enum, default_value = "rectangle")]
    base_shape: Shape,
    /// Vertical artwork position within the fixed panel.
    #[arg(long, value_enum, default_value = "grounded")]
    position: Placement,
    /// Fraction of the base width occupied by the artwork.
    #[arg(long, default_value_t = 0.9)]
    artwork_width_ratio: f64,
    /// Available paper height; a fixed standee exceeding it returns an error.
    #[arg(long, default_value_t = ManipulationOptions::default().max_height_mm)]
    max_height_mm: f64,
    /// Empty-margin detection strategy.
    #[arg(long, value_enum, default_value = "auto")]
    trim_background: Background,
    /// Custom solid background color to trim, as RRGGBB or #RRGGBB.
    #[arg(long, value_parser = parse_color, conflicts_with = "trim_background")]
    background_color: Option<[u8; 3]>,
    /// Pixels at or below this alpha value count as empty.
    #[arg(long, default_value_t = 0)]
    alpha_threshold: u8,
    /// Per-channel tolerance when trimming a solid background.
    #[arg(long, default_value_t = 8)]
    color_tolerance: u8,
}

fn parse_color(value: &str) -> Result<[u8; 3], String> {
    let value = value.strip_prefix('#').unwrap_or(value);
    if value.len() != 6 || !value.is_ascii() {
        return Err("use a six-digit RGB color such as FFFFFF".into());
    }
    let mut result = [0; 3];
    for (index, channel) in result.iter_mut().enumerate() {
        *channel = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16)
            .map_err(|_| "use a six-digit RGB color such as FFFFFF".to_owned())?;
    }
    Ok(result)
}

fn run(args: Args) -> Result<()> {
    let source = image::ImageReader::open(&args.input)
        .with_context(|| format!("cannot open {}", args.input.display()))?
        .with_guessed_format()
        .context("cannot identify image format")?
        .decode()
        .with_context(|| format!("cannot decode {}", args.input.display()))?;
    let mode = args
        .background_color
        .map(TrimMode::Color)
        .unwrap_or(match args.trim_background {
            Background::Auto => TrimMode::Auto,
            Background::Alpha => TrimMode::Alpha,
            Background::White => TrimMode::Color([255, 255, 255]),
            Background::None => TrimMode::None,
        });
    let result = prepare(
        &source,
        &ManipulationOptions {
            size: args.size,
            grid_size_mm: args.grid_size_mm,
            base_clearance_mm: args.base_clearance_mm,
            dpi: args.dpi,
            base_shape: match args.base_shape {
                Shape::Rectangle => BaseShape::Rectangle,
            },
            position: match args.position {
                Placement::Grounded => Position::Grounded,
                Placement::Hovering => Position::Hovering,
                Placement::Flying => Position::Flying,
            },
            artwork_width_ratio: args.artwork_width_ratio,
            max_height_mm: args.max_height_mm,
            trim: TrimOptions {
                mode,
                alpha_threshold: args.alpha_threshold,
                color_tolerance: args.color_tolerance,
            },
        },
    )?;
    let pixels_per_meter = (f64::from(result.image.width()) * 1000.0 / result.width_mm).round();
    if !(1.0..=f64::from(u32::MAX)).contains(&pixels_per_meter) {
        anyhow::bail!("resolution cannot be represented in PNG metadata");
    }
    let mut bytes = Vec::new();
    {
        let mut encoder =
            png::Encoder::new(&mut bytes, result.image.width(), result.image.height());
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_pixel_dims(Some(png::PixelDimensions {
            xppu: pixels_per_meter as u32,
            yppu: pixels_per_meter as u32,
            unit: png::Unit::Meter,
        }));
        let mut writer = encoder.write_header().context("cannot encode PNG header")?;
        writer
            .write_image_data(result.image.as_raw())
            .context("cannot encode PNG pixels")?;
        writer.finish().context("cannot finalize PNG")?;
    }
    std::fs::write(&args.output, bytes)
        .with_context(|| format!("cannot write {}", args.output.display()))?;
    eprintln!(
        "Saved {:.2} x {:.2} mm ({} x {} px) to {}",
        result.width_mm,
        result.height_mm,
        result.image.width(),
        result.image.height(),
        args.output.display()
    );
    Ok(())
}

fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
    }
}
