use std::borrow::Cow;

use image::{DynamicImage, metadata::Cicp};
use moxcms::{ColorProfile, DataColorSpace, Layout, TransformOptions};

use crate::{Error, check_dimensions};

/// Convert decoded pixels to sRGB before trimming and preparing standees.
///
/// Embedded RGB/grayscale ICC profiles take precedence over CICP metadata.
/// Untagged inputs use sRGB. Conversion retains floating-point precision and
/// alpha; a missing profile does not reduce 16-bit inputs to 8 bits.
///
/// # Errors
/// Returns an error for malformed/unsupported ICC profiles or unsupported color
/// conversions. CMYK profiles cannot be applied to already RGB-decoded pixels.
pub fn normalize_color(mut image: DynamicImage, icc: Option<&[u8]>) -> Result<DynamicImage, Error> {
    check_dimensions(image.width(), image.height())?;
    let Some(icc) = icc else {
        if image.color_space() != Cicp::SRGB {
            // Convert in float so wide-gamut/linear input isn't rounded to 8 bits.
            image = DynamicImage::ImageRgba32F(image.to_rgba32f());
            image
                .apply_color_space(Cicp::SRGB, Default::default())
                .map_err(|error| Error::Color(error.to_string()))?;
        }
        return Ok(image);
    };
    let profile = ColorProfile::new_from_slice(icc).map_err(|e| Error::Color(e.to_string()))?;
    let rgba = image.to_rgba32f();
    let (layout, input) = match profile.color_space {
        DataColorSpace::Rgb => (Layout::Rgba, Cow::Borrowed(rgba.as_raw().as_slice())),
        DataColorSpace::Gray
            if matches!(
                image.color(),
                image::ColorType::L8
                    | image::ColorType::La8
                    | image::ColorType::L16
                    | image::ColorType::La16
            ) =>
        {
            (
                Layout::GrayAlpha,
                Cow::Owned(rgba.pixels().flat_map(|p| [p[0], p[3]]).collect()),
            )
        }
        _ => {
            return Err(Error::Color(
                "ICC profile does not match supported decoded RGB/grayscale pixels".into(),
            ));
        }
    };
    let transform = profile
        .create_transform_f32(
            layout,
            &ColorProfile::new_srgb(),
            Layout::Rgba,
            TransformOptions {
                prefer_fixed_point: false,
                ..Default::default()
            },
        )
        .map_err(|e| Error::Color(e.to_string()))?;
    let mut output = image::Rgba32FImage::new(image.width(), image.height());
    transform
        .transform(&input, output.as_mut())
        .map_err(|e| Error::Color(e.to_string()))?;
    // Alpha is coverage, independent of the source color profile.
    for (p, original) in output.pixels_mut().zip(rgba.pixels()) {
        p[3] = original[3];
    }
    Ok(DynamicImage::ImageRgba32F(output))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    #[test]
    fn untagged_sixteen_bit_pixels_are_not_reduced_on_import() {
        let source = DynamicImage::ImageRgba16(ImageBuffer::from_pixel(
            2,
            2,
            Rgba([12345, 23456, 34567, 45678]),
        ));
        let result = normalize_color(source.clone(), None).unwrap();
        assert_eq!(result.as_rgba16(), source.as_rgba16());
    }

    #[test]
    fn rgb_icc_conversion_changes_color_and_preserves_alpha() {
        // Adobe RGB has approximately gamma 2.2. Neutral 0.5 maps to ~0.504 sRGB.
        let source =
            DynamicImage::ImageRgba32F(ImageBuffer::from_pixel(2, 2, Rgba([0.5, 0.5, 0.5, 0.25])));
        let profile = ColorProfile::new_adobe_rgb().encode().unwrap();
        let result = normalize_color(source, Some(&profile))
            .unwrap()
            .to_rgba32f();
        let p = result.get_pixel(0, 0);
        assert!((p[0] - 0.504).abs() < 0.003, "{p:?}");
        assert!(
            (p[0] - 0.5).abs() > 0.001,
            "ICC must actually transform pixels"
        );
        assert_eq!(p[3], 0.25);
    }

    #[test]
    fn linear_grayscale_icc_is_converted_before_rgb_output() {
        let source =
            DynamicImage::ImageLumaA16(ImageBuffer::from_pixel(2, 2, image::LumaA([32768, 16384])));
        let profile = ColorProfile::new_gray_with_gamma(1.0).encode().unwrap();
        let result = normalize_color(source, Some(&profile))
            .unwrap()
            .to_rgba32f();
        let p = result.get_pixel(0, 0);
        assert!((p[0] - 0.735).abs() < 0.005, "{p:?}");
        assert!((p[3] - 0.25).abs() < 0.001);
    }

    #[test]
    fn cicp_linear_light_is_encoded_as_srgb_without_losing_alpha() {
        let mut source =
            DynamicImage::ImageRgba32F(ImageBuffer::from_pixel(2, 2, Rgba([0.5, 0.5, 0.5, 0.25])));
        source.set_color_space(Cicp::SRGB_LINEAR).unwrap();
        let result = normalize_color(source, None).unwrap().to_rgba32f();
        let p = result.get_pixel(0, 0);
        assert!((p[0] - 0.735).abs() < 0.005, "{p:?}");
        assert_eq!(p[3], 0.25);
    }

    #[test]
    fn invalid_or_incompatible_profiles_are_not_silently_ignored() {
        let source = DynamicImage::new_rgba8(2, 2);
        assert!(normalize_color(source.clone(), Some(b"invalid ICC")).is_err());
        let gray = ColorProfile::new_gray_with_gamma(1.0).encode().unwrap();
        assert!(normalize_color(source, Some(&gray)).is_err());
    }
}
