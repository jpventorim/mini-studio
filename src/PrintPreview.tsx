import {
  Badge,
  Button,
  EmptyState,
  Notice,
  PreviewControls,
} from "@mini-studio/ui";
import { ImagePlus, X } from "lucide-react";
import { type CSSProperties, memo, useId, useMemo } from "react";
import type {
  ImageSize,
  PreparedPreviewImage,
  PreviewPage,
  PrintImage,
} from "./print-workspace";
import { usePreviewCamera } from "./usePreviewCamera";

const PageArtwork = memo(function PageArtwork({
  page,
  index,
  sources,
}: {
  page: PreviewPage;
  index: number;
  sources: ReadonlyMap<number, PreparedPreviewImage>;
}) {
  return (
    <div
      className="page-artwork"
      data-animate={page.placements.length <= 100}
      role="img"
      aria-label={`Page ${index + 1}, ${page.placements.length} image copies`}
    >
      {page.placements.map((placement) => {
        const image = sources.get(placement.sourceIndex);
        if (!image) return null;
        const x = (placement.xMm / page.widthMm) * 100;
        const y = (placement.yMm / page.heightMm) * 100;
        return (
          <div
            key={`${placement.imageId}-${placement.copyIndex}`}
            className="page-placement"
            style={{
              transform: `translate(${x}cqw, ${y}cqh)`,
              width: `${(placement.widthMm / page.widthMm) * 100}%`,
              height: `${(placement.heightMm / page.heightMm) * 100}%`,
            }}
          >
            <img
              src={image.preview}
              decoding="async"
              alt=""
              draggable={false}
              style={
                placement.rotated
                  ? {
                      width: `${(placement.heightMm / placement.widthMm) * 100}%`,
                      height: `${(placement.widthMm / placement.heightMm) * 100}%`,
                      transform: "translate(-50%, -50%) rotate(90deg)",
                    }
                  : undefined
              }
            />
          </div>
        );
      })}
    </div>
  );
});

export function PrintPreview({
  pages,
  images,
  preparedImages,
  size,
  pending,
  problem,
  onDismissProblem,
}: {
  pages: PreviewPage[];
  images: PrintImage[];
  preparedImages: PreparedPreviewImage[];
  size: ImageSize;
  pending: boolean;
  problem?: string;
  onDismissProblem: () => void;
}) {
  const helpId = useId();
  const { anchor, camera, fit, panning, strip, stageProps, zoomIn, zoomOut } =
    usePreviewCamera();
  const width = pages[0]?.widthMm ?? size.width;
  const height = pages[0]?.heightMm ?? size.height;
  // Image settings can change while the displayed preview stays the same.
  // Only membership changes need a new visibility map before repacking finishes.
  const imageIds = images.map((image) => image.id).join(",");
  const sources = useMemo(() => {
    const active = new Set(
      imageIds === "" ? [] : imageIds.split(",").map(Number),
    );
    return new Map(
      preparedImages
        .map((image, index) => [index, image] as const)
        .filter(([, image]) => active.has(image.imageId)),
    );
  }, [imageIds, preparedImages]);
  const sheets = useMemo(
    () =>
      pages.map((page, index) => (
        <figure
          className="preview-page"
          // biome-ignore lint/suspicious/noArrayIndexKey: Each sheet is a fixed slot in print order.
          key={index}
        >
          <div className="preview-sheet">
            <PageArtwork page={page} index={index} sources={sources} />
          </div>
          {pages.length > 1 && (
            <figcaption className="preview-page-label">
              <Badge tone="neutral">Page {index + 1}</Badge>
            </figcaption>
          )}
        </figure>
      )),
    [pages, sources],
  );

  return (
    <div className="preview">
      <p className="rf-visually-hidden" id={helpId}>
        Scroll to zoom and drag to pan. With the canvas focused, use plus or
        minus to zoom, the arrow keys to pan, and 0 to fit the sheets.
      </p>
      <section
        {...stageProps}
        className="preview-stage"
        data-panning={panning}
        style={
          {
            "--camera-x": `${camera.x}px`,
            "--camera-y": `${camera.y}px`,
            "--camera-zoom": camera.zoom,
          } as CSSProperties
        }
        aria-label="Print preview canvas"
        aria-describedby={helpId}
        // biome-ignore lint/a11y/noNoninteractiveTabindex: The canvas handles keyboard pan and zoom.
        tabIndex={0}
      >
        <div
          className="preview-pages"
          ref={anchor}
          style={
            {
              "--sheet-ratio": width / height,
              "--sheet-columns": Math.min(3, Math.max(1, pages.length)),
            } as CSSProperties
          }
        >
          <div className="preview-strip" ref={strip}>
            {pages.length > 0 ? (
              sheets
            ) : (
              <div className="preview-sheet">
                <EmptyState
                  title={
                    images.length ? "Waiting for layout" : "Your print sheet"
                  }
                  description={
                    images.length
                      ? "Images appear here when they fit."
                      : "Add images to see them on the page."
                  }
                  icon={<ImagePlus size={28} strokeWidth={1.3} />}
                />
              </div>
            )}
          </div>
        </div>
      </section>

      {problem && (
        <Notice
          className="preview-message"
          tone="error"
          action={
            <Button
              type="button"
              variant="ghost"
              iconOnly
              aria-label="Dismiss error"
              onClick={onDismissProblem}
            >
              <X size={17} aria-hidden="true" />
            </Button>
          }
        >
          {problem}
        </Notice>
      )}
      {pending && (
        <Notice className="preview-message preview-updating">
          Updating layout…
        </Notice>
      )}
      <PreviewControls
        className="preview-dock"
        fitLabel={pages.length > 1 ? "Fit sheets" : "Fit page"}
        zoom={camera.zoom}
        onFit={fit}
        onZoomIn={zoomIn}
        onZoomOut={zoomOut}
        canZoomIn={camera.zoom < 8}
        canZoomOut={camera.zoom > 0.25}
      />
    </div>
  );
}
