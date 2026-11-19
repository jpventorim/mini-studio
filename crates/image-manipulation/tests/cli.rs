#![cfg(feature = "cli")]

use image::{Rgb, RgbImage, Rgba, RgbaImage};
use std::process::Command;

#[test]
fn cli_writes_numbered_copies_and_restarts_for_each_input() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("source.png");
    RgbaImage::from_pixel(10, 20, Rgba([80, 20, 150, 255]))
        .save(&input)
        .unwrap();
    for (name, quantity) in [("first", 4), ("second", 3)] {
        let output = dir.path().join(format!("{name}.png"));
        let result = Command::new(env!("CARGO_BIN_EXE_image-manipulation"))
            .arg("--input")
            .arg(&input)
            .arg("--output")
            .arg(&output)
            .args(["--copies", &quantity.to_string(), "--number-copies"])
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(!output.exists());
        for index in 1..=quantity {
            let png = image::open(dir.path().join(format!("{name}-{index}.png")))
                .unwrap()
                .to_rgba8();
            let source = image::open(&input).unwrap();
            let options = image_manipulation::ManipulationOptions {
                number_copies: true,
                ..Default::default()
            };
            let expected = image_manipulation::prepare_copies(&source, quantity, &options)
                .unwrap()
                .nth(index - 1)
                .unwrap()
                .unwrap();
            assert_eq!(png, expected.image);
        }
    }
    assert_eq!(
        std::fs::read(dir.path().join("first-1.png")).unwrap(),
        std::fs::read(dir.path().join("second-1.png")).unwrap()
    );
    let result = Command::new(env!("CARGO_BIN_EXE_image-manipulation"))
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(dir.path().join("invalid.png"))
        .args(["--copies", "0", "--number-copies"])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(!dir.path().join("invalid.png").exists());
}

#[test]
fn cli_writes_a_symmetric_png_with_physical_size_metadata() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("source image.png");
    let output = dir.path().join("standee.png");
    let mut source = RgbaImage::new(14, 18);
    for y in 3..15 {
        for x in 3..11 {
            source.put_pixel(x, y, Rgba([80, 20, 150, 255]));
        }
    }
    source.save(&input).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_image-manipulation"))
        .arg("--input")
        .arg(input)
        .arg("--output")
        .arg(&output)
        .args(["--size", "2"])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let image = image::open(&output).unwrap().to_rgba8();
    assert_eq!(image.width(), 567);
    for y in 0..image.height() {
        for x in 0..image.width() {
            assert_eq!(
                image.get_pixel(x, y),
                image.get_pixel(x, image.height() - 1 - y)
            );
        }
    }
    let decoder = png::Decoder::new(std::io::BufReader::new(
        std::fs::File::open(output).unwrap(),
    ));
    let reader = decoder.read_info().unwrap();
    let resolution = reader.info().pixel_dims.unwrap();
    assert_eq!(resolution.unit, png::Unit::Meter);
    let width_mm = f64::from(image.width()) / f64::from(resolution.xppu) * 1000.0;
    assert!((width_mm - 48.0).abs() < 0.01);
}

#[test]
fn cli_accepts_opaque_backgrounds_and_rejects_invalid_sizes_without_writing() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("opaque.png");
    let output = dir.path().join("prepared.png");
    let mut source = RgbImage::from_pixel(10, 10, Rgb([40, 60, 80]));
    source.put_pixel(5, 5, Rgb([255, 80, 30]));
    source.save(&input).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_image-manipulation"))
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .args(["--background-color", "283C50"])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
    let failed_output = dir.path().join("failed.png");
    for size in ["0", "NaN", "inf", "100000000000"] {
        let result = Command::new(env!("CARGO_BIN_EXE_image-manipulation"))
            .arg("--input")
            .arg(&input)
            .arg("--output")
            .arg(&failed_output)
            .args(["--size", size])
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(!failed_output.exists());
    }
}

#[test]
fn cli_applies_the_complete_strip_height_limit() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("tall.png");
    let output = dir.path().join("bounded.png");
    RgbImage::from_pixel(10, 100, Rgb([80, 20, 150]))
        .save(&input)
        .unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_image-manipulation"))
        .arg("--input")
        .arg(input)
        .arg("--output")
        .arg(&output)
        .args(["--size", "2", "--max-height-mm", "180"])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let image = image::open(&output).unwrap();
    let width_mm = 48.0;
    let height_mm = f64::from(image.height()) * width_mm / f64::from(image.width());
    assert!(height_mm <= 180.0);
    assert!(height_mm > 179.0);
    assert_eq!(image.width(), 567);
}

#[test]
fn cli_position_presets_share_dimensions_and_raise_the_artwork() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("figure.png");
    RgbImage::from_pixel(10, 100, Rgb([80, 20, 150]))
        .save(&input)
        .unwrap();
    let mut dimensions = Vec::new();
    let mut gaps = Vec::new();
    for position in ["grounded", "hovering", "flying"] {
        let output = dir.path().join(format!("{position}.png"));
        let result = Command::new(env!("CARGO_BIN_EXE_image-manipulation"))
            .arg("--input")
            .arg(&input)
            .arg("--output")
            .arg(&output)
            .args(["--position", position])
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let image = image::open(output).unwrap().to_rgba8();
        dimensions.push(image.dimensions());
        let artwork_bottom = image
            .enumerate_pixels()
            .filter(|(_, y, pixel)| *y >= image.height() / 2 && pixel.0 == [80, 20, 150, 255])
            .map(|(_, y, _)| y)
            .max()
            .unwrap();
        let base_top = image.height() / 2 + image.width() * 3 / 2;
        gaps.push(base_top - artwork_bottom - 1);
    }
    assert!(dimensions.windows(2).all(|pair| pair[0] == pair[1]));
    assert!(gaps[0] < gaps[1] && gaps[1] < gaps[2]);
}
