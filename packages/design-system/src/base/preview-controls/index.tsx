import { Maximize, Minus, Plus } from "lucide-react";
import { Button } from "../button";

export function PreviewControls({
  zoom,
  onFit,
  fitLabel = "Fit page",
  onZoomIn,
  onZoomOut,
  canZoomIn = true,
  canZoomOut = true,
  className = "",
}: {
  zoom: number;
  onFit: () => void;
  fitLabel?: string;
  onZoomIn: () => void;
  onZoomOut: () => void;
  canZoomIn?: boolean;
  canZoomOut?: boolean;
  className?: string;
}) {
  return (
    <div className={`rf-preview-controls ${className}`}>
      <fieldset
        className="rf-preview-control-group rf-paper-surface"
        aria-label="Preview zoom"
      >
        <Button
          type="button"
          iconOnly
          variant="ghost"
          aria-label={fitLabel}
          title={fitLabel}
          onClick={onFit}
        >
          <Maximize size={15} aria-hidden="true" />
        </Button>
        <span className="rf-preview-control-divider" aria-hidden="true" />
        <Button
          type="button"
          iconOnly
          variant="ghost"
          aria-label="Zoom out"
          title="Zoom out"
          disabled={!canZoomOut}
          onClick={onZoomOut}
        >
          <Minus size={15} aria-hidden="true" />
        </Button>
        <span
          className="rf-preview-zoom-value"
          title="Zoom relative to a fitted page"
        >
          {zoom < 0.01 ? "<1%" : `${Math.round(zoom * 100)}%`}
        </span>
        <Button
          type="button"
          iconOnly
          variant="ghost"
          aria-label="Zoom in"
          title="Zoom in"
          disabled={!canZoomIn}
          onClick={onZoomIn}
        >
          <Plus size={15} aria-hidden="true" />
        </Button>
      </fieldset>
    </div>
  );
}
