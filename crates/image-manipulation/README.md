# image-manipulation

Create a foldable paper standee from an image with or without transparency.

## Processing pipeline

The crate prepares one source image into a mirrored strip with fixed-height
artwork panels and two square base tabs for printing, cutting, and folding. Physical dimensions use millimetres.
Raster operations use whole pixels.

1. **Load and normalize the source.** The development CLI decodes PNG or JPEG.
   The library accepts a borrowed image view, including opaque and transparent
   inputs, and copies retained pixels into RGBA8 during trimming.
2. **Calculate the grid footprint.** Multiply `size` by `grid_size_mm` to obtain
   the nominal footprint. For example, size 2 on a 25 mm grid occupies a nominal
   50 × 50 mm square.
3. **Apply base clearance.** Automatic clearance reserves 0.5 mm per edge for
   sizes below 1, or 1 mm per edge for sizes 1 and above. Subtract the total
   clearance from both base dimensions. `base_clearance_mm` can override it.
4. **Trim empty margins.** Auto mode uses alpha when any pixel is non-opaque,
   and near-white margins for fully opaque images. The default alpha threshold
   is 0, and solid-color tolerance is 8 per RGB channel. An explicit color can
   identify another plain background. Cropping retains the bounding rectangle
   of the foreground, including interior pixels.
5. **Calculate the artwork box.** Use at most `artwork_width_ratio` of the actual
   base width, defaulting to 90%. The artwork panel, including its insets and
   center-fold stroke, has a fixed height of 1.5 times the actual base width.
6. **Reserve clean cutting space.** Keep at least 1 mm around the artwork,
   excluding fold lines. Convert this clearance using the effective pixel scale
   and round up, so pixel rounding preserves the physical minimum. The width
   limit can leave additional horizontal space.
7. **Check the complete-strip height limit.** Include both fixed artwork panels
   and both square base tabs in `max_height_mm`, which defaults to 297 mm. Return
   `Error::HeightLimitExceeded` if the strip exceeds this limit. Fixed panel
   dimensions round down to whole pixels.
8. **Reserve elevation and resize proportionally.** Position reserves 0%, 10%,
   or 30% of panel height below grounded, hovering, or flying artwork. Scale the
   trimmed image into the remaining width and height, including cutting insets. Lanczos3 resampling uses premultiplied alpha to prevent
   hidden colors in transparent pixels from producing fringes. Pixel rounding
   can change either dimension by less than one pixel.
9. **Position the artwork and add the base.** Center the resized artwork
   horizontally inside its panel, aligned toward the base at the chosen
   elevation. Add an opaque white square tab below the panel. Preserve
   transparency in the artwork panel and its clean inset.
10. **Draw the base lines.** Add one-pixel black horizontal lines at the tab's
    midpoint and the join between the artwork panel and tab.
11. **Create the mirrored pair and center fold.** Reflect the completed half
    vertically above the upright half, including its base tab and divider lines.
    Add a symmetric two-row black center-fold mark outside the clean insets.
    The resulting order is
    `[flipped base | flipped artwork | upright artwork | upright base]`.
12. **Return or export the result.** Return the RGBA image, physical dimensions,
    effective DPI, retained source bounds, upright artwork and base bounds, and
    fold coordinate. The optional CLI encodes a PNG with physical-resolution
    metadata and writes it to the output path.

Input validation rejects invalid dimensions, empty foregrounds, insufficient
space for the base tabs and cutting insets, and working images over the pixel
limit. See the library configuration and limits below.

## Sizing and clearance

The size parameter multiplies the grid to define the nominal footprint. The
automatic clearance leaves 0.5 mm on each edge for sizes below 1, and 1 mm on
each edge for sizes 1 and up. On a 25 mm grid, size 0.5 produces an 11.5 × 11.5 mm
base, size 1 produces a 23 × 23 mm base, and size 2 produces a 48 × 48 mm base.
Set `base_clearance_mm` to `Some(total_mm)` (CLI `--base-clearance-mm`) to override
this allowance, including zero for an exact fit. `None` selects the automatic
rule. Clearance is applied once after the grid multiplier. Base shape currently
supports `Rectangle`.

The fixed panel height is 1.5 times the actual base width, including cutting
insets and the center-fold stroke. Every image in the same size category has
the same panel and strip dimensions, independent of aspect ratio and position.

| Size | Grid footprint | Actual square base | Fixed panel height | Unfolded strip height |
|---|---|---|---|---|
| 0.5 | 12.5 mm | 11.5 mm | 17.25 mm | 57.5 mm |
| 1 | 25 mm | 23 mm | 34.5 mm | 115 mm |
| 2 | 50 mm | 48 mm | 72 mm | 240 mm |
| 3 | 75 mm | 73 mm | 109.5 mm | 365 mm |

Raster dimensions can be slightly smaller after rounding. The full strip must
fit `max_height_mm`, defaulting to 297 mm for A4 portrait. Size 3 requires a
larger available height, for example `--max-height-mm 365`. A smaller height
limit returns a typed error so the selected size keeps its fixed proportions.

## Position

Position controls how high the artwork sits above its base. Set
`ManipulationOptions::position` or CLI `--position`:

| Position | Elevation above the grounded location |
|---|---|
| `Grounded` / `grounded` | 0%, the default |
| `Hovering` / `hovering` | 10% of panel height |
| `Flying` / `flying` | 30% of panel height |

All positions keep at least 1 mm of clear cutting space around the artwork.
Elevation is reserved before fitting the image, so hovering and flying may
produce smaller artwork. Aspect ratio is preserved and the image stays centered
horizontally. The complete positioned half is mirrored vertically.

Panel and strip sizes stay the same for every position. On the default 25 mm
grid at size 1, the base is 23 mm wide, each panel is 34.5 mm high, and the full
strip is 23 × 115 mm. Hovering adds 3.45 mm of elevation; flying adds 10.35 mm.
Pixel rounding may slightly change these measurements.

```sh
cargo run -p image-manipulation --features cli -- \
  --input figure.png --output flying.png --position flying
```

## Library

```rust
use image_manipulation::{ManipulationOptions, prepare};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = image::open("figure.png")?;
    let output = prepare(&source, &ManipulationOptions::default())?;
    output.image.save("standee.png")?;
    println!("{} x {} mm", output.width_mm, output.height_mm);
    Ok(())
}
```

`prepare` and `trim` borrow `GenericImageView` values with 8-bit channels, including
`ImageBuffer`, `DynamicImage`, and subimages. Mixed source formats can use
`DynamicImage`; its view normalizes higher bit depths to RGBA8. The output is an
owned `RgbaImage`. Source alpha is retained around the artwork, while the base
tabs are opaque white. Resampling uses premultiplied alpha to avoid dark or
colored fringes from hidden transparent pixels.

Defaults are size 1, 25 mm grid, automatic base clearance, 300 target DPI,
rectangular base, grounded position, 90% artwork width limit, maximum strip
height 297 mm,
1 mm minimum clear inset, alpha threshold 0, and solid-background tolerance 8
per RGB channel.
`TrimMode::Alpha` preserves opaque white margins, `TrimMode::Color([r, g, b])`
selects another solid background, and `TrimMode::None` keeps the full rectangle.
Fully empty foregrounds and invalid sizes return typed errors. Any working image
is limited to 16,777,216 pixels. Raster rounding can change aspect ratios by less
than one pixel per dimension. Color profiles are not preserved.

The result includes physical dimensions, effective DPI, the retained source
bounds, upright artwork/base rectangles, and the center fold coordinate. Physical
width equals grid size times size minus clearance; height and effective DPI use
the same pixel scale after rounding. `RgbaImage::save` does not retain this
physical-size metadata; use the CLI or keep the result dimensions with the image.

## Development CLI

```sh
cargo run -p image-manipulation --features cli -- \
  --input figure.png --output standee.png \
  --size 1 --grid-size-mm 25 --base-shape rectangle --position grounded
```

The optional CLI reads PNG/JPEG inputs and writes RGBA PNG with resolution
metadata. Use `--position grounded|hovering|flying` to choose placement, and
`--max-height-mm` to match the available paper height after paper margins.
Use `--dpi`, `--artwork-width-ratio`, `--trim-background`,
`--background-color RRGGBB`, `--alpha-threshold`, and `--color-tolerance` to adjust
preparation. Run `--help` for details. No output file is written until preparation
and encoding succeed. An existing destination is overwritten on success.

## Packing

Pass the result directly to `image-packing` with its physical width:

```rust,ignore
let item = image_packing::ImageItem::new("standee", &output.image, 10)
    .with_width_mm(output.width_mm);
let pdf = image_packing::compose(&[item], &image_packing::LayoutOptions::default())?;
```

For a size-1 PNG on the default grid, the packing CLI equivalent is:

```sh
cargo run -p image-packing --features cli -- \
  --item standee.png 10 --width-mm 23 --border-px 0 --output standees.pdf
```

The explicit width preserves grid sizing independently of pixel rounding. Zero
border avoids shrinking the standee to make room for an additional validation
border. Larger sizes use their corresponding physical width.
