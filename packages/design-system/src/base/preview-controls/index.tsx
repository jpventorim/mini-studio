import { ChevronLeft, ChevronRight, Maximize, Minus, Plus } from "lucide-react";
import { Button } from "../button";

export function PreviewControls({
  page,
  pageCount,
  onPageChange,
  zoom,
  onFit,
  onZoomIn,
  onZoomOut,
  canZoomIn = true,
  canZoomOut = true,
  className = "",
}: {
  page: number;
  pageCount: number;
  onPageChange: (page: number) => void;
  zoom: number;
  onFit: () => void;
  onZoomIn: () => void;
  onZoomOut: () => void;
  canZoomIn?: boolean;
  canZoomOut?: boolean;
  className?: string;
}) {
  return (
    <div className={`rf-preview-controls ${className}`}>
      {pageCount > 1 && (
        <nav
          className="rf-preview-control-group rf-paper-surface"
          aria-label="Preview pages"
        >
          <Button
            type="button"
            iconOnly
            variant="ghost"
            aria-label="Previous page"
            disabled={page <= 0}
            onClick={() => onPageChange(page - 1)}
          >
            <ChevronLeft size={16} aria-hidden="true" />
          </Button>
          <span aria-live="polite">
            Page {page + 1} of {pageCount}
          </span>
          <Button
            type="button"
            iconOnly
            variant="ghost"
            aria-label="Next page"
            disabled={page >= pageCount - 1}
            onClick={() => onPageChange(page + 1)}
          >
            <ChevronRight size={16} aria-hidden="true" />
          </Button>
        </nav>
      )}
      <fieldset
        className="rf-preview-control-group rf-paper-surface"
        aria-label="Preview zoom"
      >
        <Button
          type="button"
          iconOnly
          variant="ghost"
          aria-label="Fit page"
          title="Fit page"
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
          title="Zoom relative to fit page"
        >
          {Math.round(zoom * 100)}%
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
