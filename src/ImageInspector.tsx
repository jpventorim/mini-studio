import {
  Accordion,
  Button,
  Checkbox,
  DimensionFields,
  Notice,
  NumberField,
  Readout,
  Slider,
} from "@rust-foundry/ui";
import { Trash2 } from "lucide-react";
import {
  type ImageSize,
  imageDimensions,
  millimetres,
  type PrintImage,
} from "./print-workspace";

export function ImageInspector({
  image,
  dpi,
  problem,
  update,
  remove,
}: {
  image: PrintImage;
  dpi: number;
  problem?: string;
  update: (patch: Partial<PrintImage>) => void;
  remove: () => void;
}) {
  const size = imageDimensions(image, dpi);
  const ratio = image.width / image.height;

  function setDimension(key: keyof ImageSize, value: number) {
    const customSize = { ...size, [key]: value };
    if (image.lockRatio) {
      if (key === "width") customSize.height = value / ratio;
      else customSize.width = value * ratio;
    }
    update({ customSize });
  }

  function setScale(scale: number) {
    update({ scale, customSize: undefined });
  }

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
        <div>
          <NumberField
            label="Scale"
            labelLayout="inline"
            integer
            min={1}
            max={400}
            value={image.scale}
            unit="%"
            onChange={setScale}
          />
          <Slider
            label="Image scale"
            hideHeading
            min={1}
            max={400}
            step={1}
            value={image.scale}
            onValueChange={setScale}
          />
        </div>
        <Readout
          label={image.customSize ? "Custom size" : "Print size"}
          value={`${millimetres.format(size.width)} × ${millimetres.format(size.height)} mm`}
          action={
            image.customSize && (
              <Button
                type="button"
                variant="ghost"
                size="sm"
                onClick={() => update({ customSize: undefined })}
              >
                Reset to scale
              </Button>
            )
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
                <DimensionFields
                  unit="mm"
                  width={size.width}
                  height={size.height}
                  min={0.01}
                  max={1000}
                  locked={image.lockRatio}
                  onWidthChange={(value) => setDimension("width", value)}
                  onHeightChange={(value) => setDimension("height", value)}
                  onLockedChange={(lockRatio) =>
                    update({
                      lockRatio,
                      customSize:
                        lockRatio && image.customSize
                          ? { width: size.width, height: size.width / ratio }
                          : image.customSize,
                    })
                  }
                />
                <Checkbox
                  label="Allow 90° rotation"
                  markAlignment="start"
                  checked={image.allowRotation}
                  onCheckedChange={(allowRotation) => update({ allowRotation })}
                />
              </div>
            ),
          },
        ]}
      />
    </section>
  );
}
