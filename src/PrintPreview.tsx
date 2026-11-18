import { EmptyState, Notice, PreviewControls } from "@rust-foundry/ui";
import { ImagePlus } from "lucide-react";
import { type CSSProperties, memo, useId, useMemo, useState } from "react";
import type { ImageSize, PreviewPage, PrintImage } from "./print-workspace";
import { usePreviewCamera } from "./usePreviewCamera";

const PageArtwork = memo(function PageArtwork({
  page,
  index,
  sources,
}: {
  page: PreviewPage;
  index: number;
  sources: ReadonlyMap<number, PrintImage>;
}) {
  return (
    <div
      className="page-artwork"
      data-animate={page.placements.length <= 100}
      role="img"
      aria-label={`Page ${index + 1}, ${page.placements.length} image copies`}
    >
      {page.placements.map((placement) => {
        const image = sources.get(placement.imageId);
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
  size,
  pending,
  problem,
}: {
  pages: PreviewPage[];
  images: PrintImage[];
  size: ImageSize;
  pending: boolean;
  problem?: string;
}) {
  const [pageIndex, setPageIndex] = useState(0);
  const helpId = useId();
  const { camera, fit, panning, sheet, stageProps, zoomIn, zoomOut } =
    usePreviewCamera();
  const currentIndex = Math.min(pageIndex, Math.max(0, pages.length - 1));
  const page = pages[currentIndex];
  const width = page?.widthMm ?? size.width;
  const height = page?.heightMm ?? size.height;
  const sources = useMemo(
    () => new Map(images.map((image) => [image.id, image])),
    [images],
  );

  return (
    <div className="preview">
      <p className="rf-visually-hidden" id={helpId}>
        Scroll to zoom and drag to pan. With the canvas focused, use plus or
        minus to zoom, the arrow keys to pan, and 0 to fit the page.
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
          className="preview-sheet"
          ref={sheet}
          style={
            {
              "--sheet-ratio": width / height,
            } as CSSProperties
          }
        >
          {page ? (
            <PageArtwork
              key={currentIndex}
              page={page}
              index={currentIndex}
              sources={sources}
            />
          ) : (
            <EmptyState
              title={images.length ? "Waiting for layout" : "Your print sheet"}
              description={
                images.length
                  ? "Images appear here when they fit."
                  : "Add images to see them on the page."
              }
              icon={<ImagePlus size={28} strokeWidth={1.3} />}
            />
          )}
        </div>
      </section>

      {problem && (
        <Notice className="preview-message" tone="error">
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
        page={currentIndex}
        pageCount={pages.length}
        onPageChange={setPageIndex}
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
