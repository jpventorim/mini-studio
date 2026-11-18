use image::{DynamicImage, Rgb, RgbImage, Rgba, RgbaImage};
use image_manipulation::{
    Error, ManipulationOptions, PixelRect, Position, TrimMode, TrimOptions, prepare, trim,
};

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
fn composition_is_a_vertical_reflection_with_full_white_square_tabs() {
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
        assert_eq!(output.base_bounds.height, expected_width);
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
        assert_eq!(output.base_bounds.width, output.base_bounds.height);
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
fn fixed_strip_height_is_checked_against_the_available_paper_height() {
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
        assert!((output.height_mm - 5.0 * output.width_mm).abs() < 0.1);
        assert_eq!(output.base_bounds.width, output.image.width());
        assert_eq!(output.base_bounds.height, output.image.width());
    }
    for (size, max_height_mm) in [(3.0, 297.0), (2.0, 180.0)] {
        assert!(matches!(
            prepare(
                &source,
                &ManipulationOptions {
                    size,
                    max_height_mm,
                    ..Default::default()
                }
            ),
            Err(Error::HeightLimitExceeded { .. })
        ));
    }
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
