import {
  Accordion,
  Button,
  Checkbox,
  Notice,
  NumberField,
  Readout,
  Select,
} from "@rust-foundry/ui";
import { Trash2 } from "lucide-react";
import {
  isStandeeSize,
  millimetres,
  type PreparedPreviewImage,
  type PrintImage,
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
  const baseWidth =
    image.size * gridSizeMm - (image.baseClearanceMm ?? automaticClearance);
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
          unit="× grid"
          value={image.size}
          min={0.25}
          max={5}
          allowedValues={standeeSizes}
          onChange={(size) => {
            if (isStandeeSize(size)) update({ size });
          }}
        />
        <Readout
          label="Folded footprint"
          value={
            prepared
              ? `${millimetres.format(prepared.baseWidthMm)} × ${millimetres.format(prepared.baseDepthMm * (image.size >= 4 ? 2 : 1))} mm`
              : `${millimetres.format(baseWidth)} × ${millimetres.format(baseWidth)} mm`
          }
        />
        <Checkbox
          label="Number copies"
          markAlignment="start"
          checked={image.numberCopies}
          onCheckedChange={(numberCopies) => update({ numberCopies })}
        />
        <Readout
          label="Standee print size"
          value={
            prepared
              ? `${millimetres.format(prepared.widthMm)} × ${millimetres.format(prepared.heightMm)} mm`
              : problem
                ? "Unavailable"
                : "Preparing…"
          }
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
                <Checkbox
                  label="Automatic base clearance"
                  markAlignment="start"
                  checked={image.baseClearanceMm === null}
                  onCheckedChange={(automatic) =>
                    update({
                      baseClearanceMm: automatic ? null : automaticClearance,
                    })
                  }
                />
                {image.baseClearanceMm !== null && (
                  <NumberField
                    label="Total base clearance"
                    unit="mm"
                    value={image.baseClearanceMm}
                    min={0}
                    max={100}
                    keyboardStep={0.5}
                    onChange={(baseClearanceMm) => update({ baseClearanceMm })}
                  />
                )}
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
                {image.trim !== "none" && (
                  <>
                    <NumberField
                      label="Alpha threshold"
                      value={image.alphaThreshold}
                      min={0}
                      max={255}
                      integer
                      keyboardStep={1}
                      onChange={(alphaThreshold) => update({ alphaThreshold })}
                    />
                    {image.trim !== "alpha" && (
                      <NumberField
                        label="Color tolerance"
                        value={image.colorTolerance}
                        min={0}
                        max={255}
                        integer
                        keyboardStep={1}
                        onChange={(colorTolerance) =>
                          update({ colorTolerance })
                        }
                      />
                    )}
                  </>
                )}
                <Checkbox
                  label="Allow 90° rotation"
                  markAlignment="start"
                  checked={image.allowRotation}
                  onCheckedChange={(allowRotation) => update({ allowRotation })}
                />
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
