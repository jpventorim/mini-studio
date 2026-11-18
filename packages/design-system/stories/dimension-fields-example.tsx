import { useState } from "react";
import { DimensionFields } from "../src/base/dimension-fields";

export function DimensionFieldsExample() {
  const [size, setSize] = useState({ width: 24, height: 32 });
  const [locked, setLocked] = useState(true);
  const ratio = 24 / 32;

  return (
    <DimensionFields
      {...size}
      min={1}
      max={1000}
      unit="mm"
      locked={locked}
      onWidthChange={(width) =>
        setSize((current) => ({
          width,
          height: locked ? width / ratio : current.height,
        }))
      }
      onHeightChange={(height) =>
        setSize((current) => ({
          height,
          width: locked ? height * ratio : current.width,
        }))
      }
      onLockedChange={(next) => {
        setLocked(next);
        if (next)
          setSize((current) => ({ ...current, height: current.width / ratio }));
      }}
    />
  );
}
