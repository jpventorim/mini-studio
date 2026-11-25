import {
  Accordion,
  Button,
  Checkbox,
  Notice,
  NumberField,
  Readout,
  Select,
} from "@mini-studio/ui";
import { Trash2 } from "lucide-react";
import {
  isStandeeSize,
  millimetres,
  type PreparedPreviewImage,
  type PrintImage,
  standeeSizeLabels,
  standeeSizes,
} from "./print-workspace";

export function ImageInspector({
  image,
  prepared,
  gridSizeMm,
  problem,
  update,
  remove,
}: {
  image: PrintImage;
  prepared?: PreparedPreviewImage;
  gridSizeMm: number;
  problem?: string;
  update: (patch: Partial<PrintImage>) => void;
  remove: () => void;
}) {
  const automaticClearance = image.size < 1 ? 1 : 2;
  const baseWidth = image.size * gridSizeMm - automaticClearance;
  return (
    <section className="inspector" aria-labelledby="image-settings-heading">
      <div className="inspector-heading">
        <div>
          <h2 id="image-settings-heading" title={image.name}>
            {image.name}
          </h2>
          <p className="asset-pixels">
            {image.width} × {image.height} px
          </p>
        </div>
        <Button
          type="button"
          variant="ghost"
          iconOnly
          aria-label="Remove image"
          onClick={remove}
        >
          <Trash2 size={17} aria-hidden="true" />
        </Button>
      </div>
      <div className="inspector-controls">
        <NumberField
          label="Quantity"
          labelLayout="inline"
          integer
          step={1}
          value={image.quantity}
          min={1}
          max={999}
          onChange={(quantity) => update({ quantity })}
        />
        <NumberField
          label="Size"
          unit={`× grid · ${standeeSizeLabels[image.size]}`}
          value={image.size}
          min={0.5}
          max={5}
          allowedValues={standeeSizes}
          onChange={(size) => {
            if (isStandeeSize(size)) update({ size });
          }}
        />
        <Readout
          label="Final base size"
          value={
            prepared
              ? `${millimetres.format(prepared.baseWidthMm)} × ${millimetres.format(prepared.baseDepthMm * (image.size >= 4 ? 2 : 1))} mm`
              : `${millimetres.format(baseWidth)} × ${millimetres.format(baseWidth)} mm`
          }
        />
        <Readout
          label="Final height (folded)"
          value={
            prepared
              ? `${millimetres.format(prepared.foldedHeightMm)} mm`
              : problem
                ? "Unavailable"
                : "Preparing…"
          }
        />
        <Checkbox
          label="Numbered bases"
          markAlignment="start"
          checked={image.numberCopies}
          onCheckedChange={(numberCopies) => update({ numberCopies })}
        />
        {problem && (
          <Notice tone="error" variant="inline" announce={false}>
            {problem}
          </Notice>
        )}
      </div>
      <Accordion
        headingLevel={3}
        items={[
          {
            value: "advanced",
            title: "Advanced",
            content: (
              <div className="rf-stack">
                <NumberField
                  label="Border"
                  unit="px"
                  value={image.borderPx}
                  min={0}
                  max={30}
                  integer
                  keyboardStep={1}
                  onChange={(borderPx) => update({ borderPx })}
                />
                <NumberField
                  label="Artwork width"
                  unit="% of base"
                  value={image.artworkWidthRatio * 100}
                  min={1}
                  max={100}
                  keyboardStep={1}
                  onChange={(value) =>
                    update({ artworkWidthRatio: value / 100 })
                  }
                />
                <Select
                  label="Trim background"
                  value={image.trim}
                  items={[
                    { value: "auto", label: "Automatic" },
                    { value: "alpha", label: "Transparency only" },
                    { value: "color", label: "Solid color" },
                    { value: "none", label: "Keep full image" },
                  ]}
                  onValueChange={(trim) => {
                    if (
                      trim === "auto" ||
                      trim === "alpha" ||
                      trim === "color" ||
                      trim === "none"
                    )
                      update({ trim });
                  }}
                />
                {image.trim === "color" && (
                  <label className="rf-label">
                    Background color
                    <input
                      type="color"
                      value={image.backgroundColor}
                      onChange={(event) =>
                        update({ backgroundColor: event.currentTarget.value })
                      }
                    />
                  </label>
                )}
                <p className="asset-pixels">
                  The base is rectangular. Standee height must fit the selected
                  paper after margins. Artwork keeps at least 1 mm of cutting
                  space.
                </p>
              </div>
            ),
          },
        ]}
      />
    </section>
  );
}
