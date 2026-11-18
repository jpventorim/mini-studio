import { Link2, Link2Off } from "lucide-react";
import { Button } from "../button";
import { NumberField } from "../number-field";

export function DimensionFields({
  width,
  height,
  min,
  max,
  unit,
  locked,
  onWidthChange,
  onHeightChange,
  onLockedChange,
}: {
  width: number;
  height: number;
  min: number;
  max: number;
  unit?: string;
  locked: boolean;
  onWidthChange: (width: number) => void;
  onHeightChange: (height: number) => void;
  onLockedChange: (locked: boolean) => void;
}) {
  const LinkIcon = locked ? Link2 : Link2Off;
  return (
    <div className="rf-dimension-fields">
      <NumberField
        label="Width"
        unit={unit}
        min={min}
        max={max}
        value={width}
        onChange={onWidthChange}
      />
      <NumberField
        label="Height"
        unit={unit}
        min={min}
        max={max}
        value={height}
        onChange={onHeightChange}
      />
      <Button
        type="button"
        iconOnly
        variant="outline"
        className="rf-dimension-link"
        aria-label="Lock aspect ratio"
        aria-pressed={locked}
        title={locked ? "Unlock aspect ratio" : "Lock aspect ratio"}
        onClick={() => onLockedChange(!locked)}
      >
        <LinkIcon size={18} aria-hidden="true" />
      </Button>
    </div>
  );
}
