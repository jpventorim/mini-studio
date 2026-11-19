use image::{DynamicImage, Rgb, RgbImage, Rgba, RgbaImage};
use image_manipulation::{
    Error, ManipulationOptions, PixelRect, Position, TrimMode, TrimOptions, prepare,
    prepare_copies, trim,
};

#[test]
fn border_preserves_pixels_physical_scale_numbering_and_metadata() {
    let source = RgbaImage::from_pixel(10, 20, Rgba([80, 20, 150, 255]));
    let blank = prepare(&source, &ManipulationOptions::default()).unwrap();
    let options = ManipulationOptions {
        border_px: 3,
        number_copies: true,
        ..Default::default()
    };
    let single = prepare(&source, &options).unwrap();
    let first = prepare_copies(&source, 2, &options)
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    assert_eq!(single.image, first.image);
    assert_eq!(single.image.width(), blank.image.width() + 6);
    assert_eq!(single.image.height(), blank.image.height() + 6);
    assert_eq!(single.dpi, blank.dpi);
    assert!((single.width_mm - blank.width_mm - 6.0 * 25.4 / blank.dpi).abs() < 1e-8);
    assert!((single.height_mm - blank.height_mm - 6.0 * 25.4 / blank.dpi).abs() < 1e-8);
    assert_eq!(single.base_bounds.x, blank.base_bounds.x + 3);
    assert_eq!(single.base_bounds.y, blank.base_bounds.y + 3);
    assert_eq!(single.fold_y, blank.fold_y + 3);
    for (x, y, pixel) in single.image.enumerate_pixels() {
        assert_eq!(
            pixel,
            single.image.get_pixel(x, single.image.height() - 1 - y)
        );
        if x < 3 || y < 3 || x >= single.image.width() - 3 || y >= single.image.height() - 3 {
            assert_eq!(pixel, &Rgba([0, 0, 0, 255]));
        } else if y >= single.artwork_bounds.y && y < single.base_bounds.y {
            assert_eq!(pixel, blank.image.get_pixel(x - 3, y - 3));
        }
    }
    let limited = ManipulationOptions {
        max_height_mm: blank.height_mm,
        ..options
    };
    let bounded = prepare(&source, &limited).unwrap();
    assert!(bounded.height_mm <= blank.height_mm);
    assert_eq!(bounded.width_mm, single.width_mm);
    assert_eq!(bounded.base_bounds.width, single.base_bounds.width);
    assert!(matches!(
        prepare(
            &source,
            &ManipulationOptions {
                border_px: u32::MAX,
                ..Default::default()
            }
        ),
        Err(Error::ImageTooLarge)
    ));
}

#[test]
fn trims_transparency_without_discarding_white_foreground() {
    let mut source = RgbaImage::from_pixel(12, 15, Rgba([230, 20, 180, 0]));
    for y in 4..10 {
        for x in 3..8 {
            source.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }
    let output = trim(&source, &TrimOptions::default()).unwrap();
    assert_eq!(
        output.source_bounds,
        PixelRect {
            x: 3,
            y: 4,
            width: 5,
            height: 6
        }
    );
    assert!(output.image.pixels().all(|p| *p == Rgba([255; 4])));
}

#[test]
fn copy_numbers_restart_for_each_source_and_only_change_the_base_tabs() {
    let options = ManipulationOptions {
        number_copies: true,
        ..Default::default()
    };
    for (color, quantity) in [([80, 20, 150, 255], 4), ([20, 150, 80, 255], 3)] {
        let source = RgbaImage::from_pixel(10, 20, Rgba(color));
        let blank = prepare(&source, &ManipulationOptions::default()).unwrap();
        let copies = prepare_copies(&source, quantity, &options).unwrap();
        assert_eq!(copies.len(), quantity);
        let mut previous = None;
        for (index, copy) in copies.enumerate() {
            let copy = copy.unwrap();
            assert_eq!(copy.copy_number, Some(index + 1));
            assert_eq!(copy.image.dimensions(), blank.image.dimensions());
            assert_eq!(
                (copy.width_mm, copy.height_mm),
                (blank.width_mm, blank.height_mm)
            );
            let base = copy.base_bounds;
            let inset = (copy.dpi / 25.4).ceil() as u32;
            let mut changed = 0;
            let mut min_x = u32::MAX;
            let mut min_y = u32::MAX;
            let mut max_x = 0;
            let mut max_y = 0;
            for (x, y, pixel) in copy.image.enumerate_pixels() {
                assert_eq!(pixel, copy.image.get_pixel(x, copy.image.height() - 1 - y));
                if pixel != blank.image.get_pixel(x, y) && y >= copy.fold_y {
                    changed += 1;
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                    assert!(x >= inset && x < base.width - inset);
                    assert!(y >= base.y + 1 + inset);
                    assert!(y < base.y + base.height / 2 - inset);
                    assert_eq!(pixel[3], 255);
                    assert_eq!(pixel[0], pixel[1]);
                    assert_eq!(pixel[1], pixel[2]);
                }
            }
            assert!(changed > 0);
            let left = min_x - base.x - inset;
            let right = base.x + base.width - inset - max_x - 1;
            let top = min_y - base.y - 1 - inset;
            let bottom = base.y + base.height / 2 - inset - max_y - 1;
            assert!(left.abs_diff(right) <= 2);
            assert!(top.abs_diff(bottom) <= 2);
            let area_width = base.width - 2 * inset;
            let area_height = base.height / 2 - 2 * inset - 1;
            assert!(max_x - min_x + 1 >= area_width - 3 || max_y - min_y + 1 >= area_height - 3);
            assert!(f64::from(max_y - min_y + 1) * 25.4 / copy.dpi >= 5.9);
            if let Some(previous) = previous {
                assert_ne!(copy.image, previous);
            }
            previous = Some(copy.image);
        }
    }
}

#[test]
fn numbering_is_optional_and_single_image_preparation_starts_at_one() {
    let source = RgbaImage::from_pixel(10, 20, Rgba([80, 20, 150, 255]));
    let options = ManipulationOptions::default();
    let blank = prepare(&source, &options).unwrap();
    assert_eq!(blank.copy_number, None);
    for copy in prepare_copies(&source, 3, &options).unwrap() {
        let copy = copy.unwrap();
        assert_eq!(copy.copy_number, None);
        assert_eq!(copy.image, blank.image);
    }
    let numbered = ManipulationOptions {
        number_copies: true,
        ..options
    };
    let single = prepare(&source, &numbered).unwrap();
    let first = prepare_copies(&source, 4, &numbered)
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    assert_eq!(single.copy_number, Some(1));
    assert_eq!(single.image, first.image);
    assert!(matches!(
        prepare_copies(&source, 0, &numbered),
        Err(Error::InvalidInput(_))
    ));
}

#[test]
fn multi_digit_labels_work_at_small_and_large_sizes_without_hiding_unreadable_labels() {
    let source = RgbaImage::from_pixel(10, 20, Rgba([80, 20, 150, 255]));
    for size in [0.5, 1.0, 2.0, 4.0] {
        for dpi in [72.0, 300.0, 600.0] {
            let options = ManipulationOptions {
                size,
                dpi,
                number_copies: true,
                ..Default::default()
            };
            let mut copies = prepare_copies(&source, 12, &options).unwrap();
            assert_eq!(copies.len(), 12);
            let last = copies.nth(11).unwrap().unwrap();
            assert_eq!(last.copy_number, Some(12));
            assert_eq!(copies.len(), 0);
            assert!(copies.next().is_none());
        }
    }
    for (size, dpi) in [(0.25, 300.0), (1.0, 10.0)] {
        let mut options = ManipulationOptions {
            size,
            dpi,
            ..Default::default()
        };
        assert!(prepare(&source, &options).is_ok());
        options.number_copies = true;
        assert!(matches!(
            prepare(&source, &options),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            prepare_copies(&source, 1, &options).unwrap().next(),
            Some(Err(Error::InvalidInput(_)))
        ));
    }
}

#[test]
fn opaque_white_and_custom_color_margins_are_trimmed_without_removing_interior() {
    let mut source = RgbImage::from_pixel(12, 15, Rgb([255; 3]));
    for y in 4..10 {
        for x in 3..8 {
            source.put_pixel(x, y, Rgb([80, 10, 140]));
        }
    }
    source.put_pixel(5, 6, Rgb([255; 3]));
    let output = trim(&source, &TrimOptions::default()).unwrap();
    assert_eq!(output.image.dimensions(), (5, 6));
    assert_eq!(*output.image.get_pixel(2, 2), Rgba([255; 4]));
    let mut colored = RgbImage::from_pixel(6, 8, Rgb([20, 60, 80]));
    colored.put_pixel(2, 3, Rgb([255, 20, 100]));
    let output = trim(
        &colored,
        &TrimOptions {
            mode: TrimMode::Color([20, 60, 80]),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(
        output.source_bounds,
        PixelRect {
            x: 2,
            y: 3,
            width: 1,
            height: 1
        }
    );
    // Auto does not guess non-white solid backgrounds from a corner pixel.
    assert_eq!(
        trim(&colored, &TrimOptions::default())
            .unwrap()
            .image
            .dimensions(),
        (6, 8)
    );
}

#[test]
fn trim_thresholds_and_no_trim_are_respected() {
    let mut source = RgbaImage::new(8, 9);
    source.put_pixel(0, 0, Rgba([80, 40, 10, 5]));
    source.put_pixel(3, 4, Rgba([200, 30, 70, 255]));
    assert_eq!(
        trim(
            &source,
            &TrimOptions {
                alpha_threshold: 8,
                ..Default::default()
            }
        )
        .unwrap()
        .source_bounds,
        PixelRect {
            x: 3,
            y: 4,
            width: 1,
            height: 1
        }
    );
    let all = trim(
        &source,
        &TrimOptions {
            mode: TrimMode::None,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(all.image, source);
    let opaque = RgbImage::from_pixel(4, 4, Rgb([250; 3]));
    assert!(matches!(
        trim(&opaque, &TrimOptions::default()),
        Err(Error::EmptyImage)
    ));
    assert!(
        trim(
            &opaque,
            &TrimOptions {
                color_tolerance: 0,
                ..Default::default()
            }
        )
        .is_ok()
    );
}

#[test]
fn composition_is_a_vertical_reflection_with_grid_sized_tabs() {
    let mut source = RgbaImage::new(8, 12);
    for y in 2..10 {
        for x in 2..6 {
            source.put_pixel(
                x,
                y,
                if y < 6 {
                    Rgba([220, 30, 70, 255])
                } else {
                    Rgba([20, 60, 220, 255])
                },
            );
        }
    }
    let options = ManipulationOptions {
        grid_size_mm: 25.4,
        base_clearance_mm: Some(0.0),
        dpi: 100.0,
        ..Default::default()
    };
    let output = prepare(&source, &options).unwrap();
    assert_eq!(output.width_mm, 25.4);
    assert_eq!(output.image.dimensions(), (100, 500));
    assert_eq!(
        output.source_bounds,
        PixelRect {
            x: 2,
            y: 2,
            width: 4,
            height: 8
        }
    );
    assert_eq!(
        output.artwork_bounds,
        PixelRect {
            x: 14,
            y: 255,
            width: 71,
            height: 141
        }
    );
    assert_eq!(
        output.base_bounds,
        PixelRect {
            x: 0,
            y: 400,
            width: 100,
            height: 100
        }
    );
    for y in 0..output.image.height() {
        for x in 0..output.image.width() {
            assert_eq!(
                output.image.get_pixel(x, y),
                output.image.get_pixel(x, output.image.height() - 1 - y)
            );
        }
    }
    assert_eq!(*output.image.get_pixel(30, 410), Rgba([255; 4]));
    assert_eq!(*output.image.get_pixel(30, 450), Rgba([0, 0, 0, 255]));
    assert_eq!(*output.image.get_pixel(30, 490), Rgba([255; 4]));
    assert_eq!(*output.image.get_pixel(20, 265), Rgba([220, 30, 70, 255]));
    assert_eq!(*output.image.get_pixel(20, 385), Rgba([20, 60, 220, 255]));
    assert_eq!(*output.image.get_pixel(0, 270), Rgba([0; 4]));
}

#[test]
fn size_grid_and_resolution_scale_the_base_with_aspect_ratio_preserved() {
    let source = DynamicImage::ImageRgb8(RgbImage::from_pixel(40, 60, Rgb([80, 10, 150])));
    for (size, grid, dpi, expected_width) in [
        (1.0, 25.4, 100.0, 100),
        (2.0, 25.4, 100.0, 200),
        (1.0, 50.8, 100.0, 200),
        (1.0, 25.4, 200.0, 200),
    ] {
        let output = prepare(
            &source,
            &ManipulationOptions {
                size,
                grid_size_mm: grid,
                base_clearance_mm: Some(0.0),
                dpi,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(output.image.width(), expected_width);
        assert!(
            (f64::from(output.base_bounds.height) / f64::from(output.fold_y) - 0.4).abs()
                <= 1.0 / f64::from(output.fold_y)
        );
        assert!(
            (f64::from(output.artwork_bounds.height) / f64::from(output.artwork_bounds.width)
                - 1.5)
                .abs()
                < 0.01
        );
        assert!((output.width_mm - size * grid).abs() < 1e-8);
    }
}

#[test]
fn larger_sizes_grow_artwork_while_preserving_the_folded_footprint() {
    let source = RgbImage::from_pixel(20, 40, Rgb([80, 20, 150]));
    for grid_size_mm in [20.0, 25.0] {
        let mut previous_art_height = 0.0;
        for size in [1.0, 2.0, 3.0] {
            let output = prepare(
                &source,
                &ManipulationOptions {
                    size,
                    grid_size_mm,
                    ..Default::default()
                },
            )
            .unwrap();
            assert!(output.base_bounds.height == output.base_bounds.width);
            let art_height = f64::from(output.artwork_bounds.height) * 25.4 / output.dpi;
            assert!(art_height > previous_art_height + 0.5);
            previous_art_height = art_height;
        }
    }
}

#[test]
fn size_four_and_up_keep_only_the_numbered_base_section() {
    let source = RgbImage::from_pixel(20, 40, Rgb([80, 20, 150]));
    for size in [3.5, 3.99, 4.0, 4.5] {
        let options = ManipulationOptions {
            size,
            dpi: 100.0,
            max_height_mm: 1000.0,
            border_px: 3,
            number_copies: true,
            ..Default::default()
        };
        for (index, output) in prepare_copies(&source, 2, &options).unwrap().enumerate() {
            let output = output.unwrap();
            let base = output.base_bounds;
            assert_eq!(output.base_has_flap, size < 4.0);
            assert_eq!(output.copy_number, Some(index + 1));
            let footprint_depth = if output.base_has_flap {
                base.height
            } else {
                2 * base.height
            };
            assert!(base.width.abs_diff(footprint_depth) <= 1);
            let is_line = |y| {
                (base.x..base.x + base.width)
                    .all(|x| output.image.get_pixel(x, y).0 == [0, 0, 0, 255])
            };
            assert!(is_line(base.y));
            assert_eq!(is_line(base.y + base.height / 2), output.base_has_flap);
            let inset = (output.dpi / 25.4).ceil() as u32;
            assert!((base.x + inset..base.x + base.width - inset).any(|x| {
                (base.y + 1 + inset..base.y + base.height - inset)
                    .any(|y| output.image.get_pixel(x, y)[0] < 255)
            }));
            for (x, y, pixel) in output.image.enumerate_pixels() {
                assert_eq!(
                    pixel,
                    output.image.get_pixel(x, output.image.height() - 1 - y)
                );
            }
        }
    }
}

#[test]
fn folded_footprint_matches_the_grid_at_every_size_even_when_height_is_capped() {
    let source = RgbImage::from_pixel(20, 40, Rgb([80, 20, 150]));
    for grid_size_mm in [20.0, 25.0] {
        for size in [0.25, 0.5, 1.0, 2.0, 3.0, 4.0, 5.0] {
            for max_height_mm in [293.0, 1000.0] {
                let output = prepare(
                    &source,
                    &ManipulationOptions {
                        size,
                        grid_size_mm,
                        max_height_mm,
                        border_px: 3,
                        dpi: 100.0,
                        ..Default::default()
                    },
                )
                .unwrap();
                let foot_depth_px = if output.base_has_flap {
                    output.base_bounds.height / 2
                } else {
                    output.base_bounds.height
                };
                let folded_depth_px = foot_depth_px * 2;
                assert!(output.base_bounds.width.abs_diff(folded_depth_px) <= 1);
                let expected_mm = grid_size_mm * size - if size < 1.0 { 1.0 } else { 2.0 };
                let mm_per_pixel = 25.4 / output.dpi;
                assert!(
                    (f64::from(folded_depth_px) * mm_per_pixel - expected_mm).abs()
                        <= mm_per_pixel + 1e-8
                );
                assert!(output.height_mm <= max_height_mm);
            }
        }
    }
}

#[test]
fn tall_artwork_fits_the_final_panel_height_at_multiple_base_sizes() {
    let source = RgbImage::from_pixel(8, 80, Rgb([80, 20, 160]));
    for size in [1.0, 2.0] {
        let output = prepare(
            &source,
            &ManipulationOptions {
                size,
                ..Default::default()
            },
        )
        .unwrap();
        let panel_height = output.base_bounds.y - output.fold_y;
        let artwork = output.artwork_bounds;
        assert!(f64::from(panel_height) <= f64::from(output.image.width()) * 1.5);
        assert!((f64::from(artwork.width) - f64::from(artwork.height) / 10.0).abs() <= 0.5);
        let mm_per_pixel = output.width_mm / f64::from(output.image.width());
        assert!(f64::from(artwork.y - output.fold_y - 1) * mm_per_pixel >= 1.0);
        assert!(f64::from(output.base_bounds.y - artwork.y - artwork.height) * mm_per_pixel >= 1.0);
        assert!((output.image.width() - artwork.width).abs_diff(2 * artwork.x) <= 1);
        assert!((output.width_mm - (25.0 * size - 2.0)).abs() < 1e-8);
        assert!(output.height_mm <= 5.0 * output.width_mm);
    }
}

#[test]
fn base_clearance_is_shared_by_both_edges_of_the_grid_footprint() {
    let source = RgbImage::from_pixel(10, 10, Rgb([80, 20, 160]));
    for (size, clearance) in [(0.5, 1.0), (0.99, 1.0), (1.0, 2.0), (2.0, 2.0)] {
        let output = prepare(
            &source,
            &ManipulationOptions {
                size,
                ..Default::default()
            },
        )
        .unwrap();
        let footprint = size * 25.0;
        assert!((output.width_mm - (footprint - clearance)).abs() < 1e-8);
        assert!(((footprint - output.width_mm) / 2.0 - clearance / 2.0).abs() < 1e-8);
        assert!(
            (f64::from(output.base_bounds.height) / f64::from(output.fold_y) - 0.4).abs()
                <= 1.0 / f64::from(output.fold_y)
        );
    }
    for clearance in [0.0, 0.5, 2.0] {
        let output = prepare(
            &source,
            &ManipulationOptions {
                base_clearance_mm: Some(clearance),
                ..Default::default()
            },
        )
        .unwrap();
        assert!((output.width_mm - (25.0 - clearance)).abs() < 1e-8);
    }
    for clearance in [-1.0, 25.0, 26.0, f64::NAN, f64::INFINITY] {
        assert!(
            prepare(
                &source,
                &ManipulationOptions {
                    base_clearance_mm: Some(clearance),
                    ..Default::default()
                }
            )
            .is_err()
        );
    }
}

#[test]
fn panel_height_is_capped_without_reducing_the_base_width() {
    let source = RgbImage::from_pixel(8, 80, Rgb([80, 20, 160]));
    for (size, max_height_mm) in [(1.0, 115.0), (2.0, 240.0), (3.0, 365.0)] {
        let output = prepare(
            &source,
            &ManipulationOptions {
                size,
                max_height_mm,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(output.height_mm <= max_height_mm);
        let panel = output.base_bounds.y - output.fold_y;
        assert!((f64::from(panel) - f64::from(output.image.width()) * 1.5).abs() < 1.0);
        assert_eq!(output.base_bounds.width, output.image.width());
        assert!(
            (f64::from(output.base_bounds.height) / f64::from(output.fold_y) - 0.4).abs()
                <= 1.0 / f64::from(output.fold_y)
        );
    }
    for (size, max_height_mm, border_px, dpi) in [
        (3.0, 297.0, 0, 300.0),
        (4.0, 297.0, 0, 300.0),
        (2.0, 180.0, 0, 300.0),
        (3.0, 293.0, 3, 72.0),
        (4.0, 293.0, 3, 100.0),
        (3.0, 293.0, 3, 300.0),
    ] {
        let output = prepare(
            &source,
            &ManipulationOptions {
                size,
                max_height_mm,
                border_px,
                dpi,
                ..Default::default()
            },
        )
        .unwrap();
        let base_mm = f64::from(output.base_bounds.width) * 25.4 / output.dpi;
        assert!((base_mm - (25.0 * size - 2.0)).abs() < 1e-8);
        let expected_depth = if size >= 4.0 { base_mm / 2.0 } else { base_mm };
        assert!(
            (f64::from(output.base_bounds.height) * 25.4 / output.dpi - expected_depth).abs()
                <= 25.4 / output.dpi
        );
        assert!(output.height_mm <= max_height_mm);
        let ideal_height =
            3.0 * base_mm + 2.0 * expected_depth + 2.0 * f64::from(border_px) * 25.4 / output.dpi;
        assert!(
            (output.height_mm - ideal_height.min(max_height_mm)).abs()
                < 3.0 * 25.4 / output.dpi + 1e-8
        );
        let mm_per_pixel = 25.4 / output.dpi;
        assert!(f64::from(output.artwork_bounds.y - output.fold_y - 1) * mm_per_pixel >= 1.0);
        assert!(
            f64::from(
                output.base_bounds.y - output.artwork_bounds.y - output.artwork_bounds.height
            ) * mm_per_pixel
                >= 1.0
        );
    }
    let too_short = ManipulationOptions {
        size: 2.0,
        max_height_mm: 4.0,
        ..Default::default()
    };
    assert!(matches!(
        prepare(&source, &too_short),
        Err(Error::HeightLimitExceeded { .. })
    ));
}

#[test]
fn one_mm_of_clear_space_is_preserved_at_different_resolutions() {
    let source = RgbImage::from_pixel(10, 10, Rgb([80, 20, 160]));
    for dpi in [72.0, 100.0, 300.0, 600.0] {
        let output = prepare(
            &source,
            &ManipulationOptions {
                dpi,
                artwork_width_ratio: 1.0,
                ..Default::default()
            },
        )
        .unwrap();
        let artwork = output.artwork_bounds;
        let mm_per_pixel = output.width_mm / f64::from(output.image.width());
        let gaps = [
            artwork.x,
            output.image.width() - artwork.x - artwork.width,
            artwork.y - output.fold_y - 1,
            output.base_bounds.y - artwork.y - artwork.height,
        ];
        for (index, gap) in gaps.into_iter().enumerate() {
            let gap_mm = f64::from(gap) * mm_per_pixel;
            assert!(gap_mm >= 1.0);
            // Grounded artwork uses remaining panel space above the figure.
            if index != 2 {
                assert!(gap_mm < 1.0 + mm_per_pixel);
            }
        }
        // Check the clear rows next to the artwork, excluding the fold and join.
        for y in output.fold_y + 1..artwork.y {
            assert_eq!(output.image.get_pixel(artwork.x, y).0[3], 0);
        }
        for y in artwork.y + artwork.height..output.base_bounds.y {
            assert_eq!(output.image.get_pixel(artwork.x, y).0[3], 0);
        }
    }
}

#[test]
fn transparent_hidden_colors_do_not_bleed_during_resize() {
    let mut source = RgbaImage::from_pixel(5, 5, Rgba([0, 255, 0, 0]));
    source.put_pixel(2, 2, Rgba([255, 0, 0, 255]));
    let output = prepare(
        &source,
        &ManipulationOptions {
            trim: TrimOptions {
                mode: TrimMode::None,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();
    let bounds = output.artwork_bounds;
    let mut found_partial_alpha = false;
    for y in bounds.y..bounds.y + bounds.height {
        for x in bounds.x..bounds.x + bounds.width {
            let p = output.image.get_pixel(x, y).0;
            if p[3] > 0 {
                assert!(
                    p[0] >= 250 && p[1] == 0 && p[2] == 0,
                    "hidden green must not leak: {p:?}"
                );
            }
            found_partial_alpha |= p[3] > 0 && p[3] < 255;
        }
    }
    assert!(found_partial_alpha);
}

#[test]
fn validation_rejects_empty_sources_and_invalid_physical_dimensions() {
    let source = RgbaImage::from_pixel(4, 4, Rgba([80, 10, 180, 255]));
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(
            prepare(
                &source,
                &ManipulationOptions {
                    max_height_mm: invalid,
                    ..Default::default()
                }
            )
            .is_err()
        );
        assert!(
            prepare(
                &source,
                &ManipulationOptions {
                    size: invalid,
                    ..Default::default()
                }
            )
            .is_err()
        );
        assert!(
            prepare(
                &source,
                &ManipulationOptions {
                    grid_size_mm: invalid,
                    ..Default::default()
                }
            )
            .is_err()
        );
        assert!(
            prepare(
                &source,
                &ManipulationOptions {
                    dpi: invalid,
                    ..Default::default()
                }
            )
            .is_err()
        );
    }
    for ratio in [0.0, -0.1, 1.1, f64::NAN] {
        assert!(
            prepare(
                &source,
                &ManipulationOptions {
                    artwork_width_ratio: ratio,
                    ..Default::default()
                }
            )
            .is_err()
        );
    }
    assert!(
        prepare(
            &source,
            &ManipulationOptions {
                size: 1e20,
                ..Default::default()
            }
        )
        .is_err()
    );
    assert!(
        prepare(
            &source,
            &ManipulationOptions {
                size: 1e-20,
                ..Default::default()
            }
        )
        .is_err()
    );
    assert!(matches!(
        prepare(&RgbaImage::new(4, 4), &ManipulationOptions::default()),
        Err(Error::EmptyImage)
    ));
    assert!(prepare(&RgbaImage::new(0, 0), &ManipulationOptions::default()).is_err());
}

#[test]
fn prepared_image_can_be_packed_at_its_physical_width() {
    let source = RgbImage::from_pixel(20, 40, Rgb([80, 20, 160]));
    let output = prepare(&source, &ManipulationOptions::default()).unwrap();
    let items = [
        image_packing::ImageItem::new("standee", &output.image, 3).with_width_mm(output.width_mm)
    ];
    let pdf = image_packing::compose(&items, &image_packing::LayoutOptions::default()).unwrap();
    assert!(pdf.pdf.starts_with(b"%PDF-"));
    let placements: Vec<_> = pdf
        .layout
        .pages()
        .iter()
        .flat_map(|p| &p.placements)
        .collect();
    assert_eq!(placements.len(), 3);
    for p in placements {
        let (width, height) = if p.rotated {
            (p.height_mm, p.width_mm)
        } else {
            (p.width_mm, p.height_mm)
        };
        assert!((width - output.width_mm).abs() < 1e-8);
        assert!((height - output.height_mm).abs() < 1e-8);
    }
}

#[test]
fn positions_reserve_elevation_with_identical_panels_and_clean_insets() {
    for (source_width, source_height) in [(8, 80), (80, 8), (10, 10)] {
        let source = RgbImage::from_pixel(source_width, source_height, Rgb([80, 20, 160]));
        for (position, elevation) in [
            (Position::Grounded, 0),
            (Position::Hovering, 15),
            (Position::Flying, 45),
        ] {
            let output = prepare(
                &source,
                &ManipulationOptions {
                    grid_size_mm: 25.4,
                    base_clearance_mm: Some(0.0),
                    dpi: 100.0,
                    position,
                    ..Default::default()
                },
            )
            .unwrap();
            let artwork = output.artwork_bounds;
            assert_eq!(output.image.dimensions(), (100, 500));
            assert_eq!(output.base_bounds.y - output.fold_y, 150);
            assert_eq!(
                output.base_bounds.y - artwork.y - artwork.height,
                4 + elevation
            );
            assert!(artwork.y - output.fold_y > 4);
            assert!(artwork.x >= 4);
            assert!(output.image.width() - artwork.x - artwork.width >= 4);
            let scale = (90.0 / f64::from(source_width))
                .min(f64::from(141 - elevation) / f64::from(source_height));
            assert!((f64::from(artwork.width) - f64::from(source_width) * scale).abs() <= 0.5);
            assert!((f64::from(artwork.height) - f64::from(source_height) * scale).abs() <= 0.5);
            for y in 0..output.fold_y {
                for x in 0..output.image.width() {
                    assert_eq!(
                        output.image.get_pixel(x, y),
                        output.image.get_pixel(x, output.image.height() - 1 - y)
                    );
                }
            }
        }
    }
}
