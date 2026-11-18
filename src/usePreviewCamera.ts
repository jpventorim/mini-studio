import {
  type KeyboardEvent,
  type PointerEvent,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";

type Point = { x: number; y: number };
type Camera = Point & { zoom: number };

const fittedCamera: Camera = { x: 0, y: 0, zoom: 1 };
const panKeys: Record<string, Point> = {
  ArrowLeft: { x: -40, y: 0 },
  ArrowRight: { x: 40, y: 0 },
  ArrowUp: { x: 0, y: -40 },
  ArrowDown: { x: 0, y: 40 },
};

function clamp(value: number, min: number, max: number) {
  return Math.min(max, Math.max(min, value));
}

function keepSheetsInView(
  camera: Camera,
  viewport: Point,
  sheet: Point,
  center: Point,
  layout: Point,
) {
  const halfWidth = (sheet.x * camera.zoom) / 2;
  const halfHeight = (sheet.y * camera.zoom) / 2;
  const x = clamp(
    camera.x,
    56 - center.x - (layout.x - sheet.x / 2) * camera.zoom,
    viewport.x - center.x + halfWidth - 56,
  );
  const y = clamp(
    camera.y,
    56 - center.y - (layout.y - sheet.y / 2) * camera.zoom,
    viewport.y - center.y + halfHeight - 56,
  );
  return x === camera.x && y === camera.y ? camera : { ...camera, x, y };
}

export function usePreviewCamera() {
  const stage = useRef<HTMLElement>(null);
  const anchor = useRef<HTMLDivElement>(null);
  const strip = useRef<HTMLDivElement>(null);
  const [camera, setCamera] = useState(fittedCamera);
  const [panning, setPanning] = useState(false);
  const pointer = useRef<(Point & { id: number }) | null>(null);

  const update = useCallback((change: (previous: Camera) => Camera) => {
    const canvas = stage.current;
    const page = anchor.current;
    const pages = strip.current;
    if (!canvas || !page || !pages || !canvas.clientWidth) return;
    const viewport = { x: canvas.clientWidth, y: canvas.clientHeight };
    const paper = { x: page.offsetWidth, y: page.offsetHeight };
    const center = {
      x: page.offsetLeft + paper.x / 2,
      y: page.offsetTop + paper.y / 2,
    };
    const layout = { x: pages.offsetWidth, y: pages.offsetHeight };
    setCamera((previous) =>
      keepSheetsInView(change(previous), viewport, paper, center, layout),
    );
  }, []);

  const zoomAt = useCallback(
    (factor: number, anchor: Point) => {
      update((previous) => {
        const zoom = clamp(
          previous.zoom * factor,
          Math.min(0.25, previous.zoom),
          8,
        );
        if (zoom === previous.zoom) return previous;
        const ratio = zoom / previous.zoom;
        return {
          x: anchor.x + (previous.x - anchor.x) * ratio,
          y: anchor.y + (previous.y - anchor.y) * ratio,
          zoom,
        };
      });
    },
    [update],
  );

  useEffect(() => {
    const canvas = stage.current;
    const page = anchor.current;
    const pages = strip.current;
    if (!canvas || !page || !pages) return;

    const wheel = (event: WheelEvent) => {
      event.preventDefault();
      const bounds = canvas.getBoundingClientRect();
      const unit =
        event.deltaMode === 1 ? 16 : event.deltaMode === 2 ? bounds.height : 1;
      const delta = clamp(event.deltaY * unit, -300, 300);
      zoomAt(Math.exp(-delta * 0.0018), {
        x: event.clientX - bounds.left - page.offsetLeft - page.offsetWidth / 2,
        y: event.clientY - bounds.top - page.offsetTop - page.offsetHeight / 2,
      });
    };

    const resize = new ResizeObserver(() => update((previous) => previous));
    resize.observe(canvas);
    resize.observe(page);
    resize.observe(pages);
    canvas.addEventListener("wheel", wheel, { passive: false });
    return () => {
      resize.disconnect();
      canvas.removeEventListener("wheel", wheel);
    };
  }, [update, zoomAt]);

  function fit() {
    const canvas = stage.current;
    const page = anchor.current;
    const pages = strip.current;
    if (!canvas || !page || !pages?.offsetWidth) return;
    const stripWidth = pages.offsetWidth;
    const stripHeight = pages.offsetHeight;
    const pageWidth = page.offsetWidth;
    const pageHeight = page.offsetHeight;
    if (stripWidth === pageWidth && stripHeight === pageHeight) {
      setCamera(fittedCamera);
      return;
    }
    const availableWidth =
      2 * (canvas.clientWidth - page.offsetLeft) - pageWidth;
    const styles = getComputedStyle(canvas);
    const availableHeight =
      canvas.clientHeight -
      Number.parseFloat(styles.paddingTop) -
      Number.parseFloat(styles.paddingBottom);
    const zoom = Math.min(
      1,
      Math.max(1, availableWidth - 48) / stripWidth,
      Math.max(1, availableHeight - 30) / stripHeight,
    );
    setCamera({
      x: ((pageWidth - stripWidth) * zoom) / 2,
      y: ((pageHeight - stripHeight) * zoom) / 2,
      zoom,
    });
  }

  function pan(x: number, y: number) {
    update((previous) => ({
      ...previous,
      x: previous.x + x,
      y: previous.y + y,
    }));
  }

  function onPointerDown(event: PointerEvent<HTMLElement>) {
    if (event.button !== 0 || pointer.current) return;
    pointer.current = {
      id: event.pointerId,
      x: event.clientX,
      y: event.clientY,
    };
    event.currentTarget.setPointerCapture(event.pointerId);
    event.currentTarget.focus({ preventScroll: true });
    setPanning(true);
  }

  function onPointerMove(event: PointerEvent<HTMLElement>) {
    const start = pointer.current;
    if (start?.id !== event.pointerId) return;
    pointer.current = { ...start, x: event.clientX, y: event.clientY };
    pan(event.clientX - start.x, event.clientY - start.y);
  }

  function endPan(event: PointerEvent<HTMLElement>) {
    if (pointer.current?.id !== event.pointerId) return;
    pointer.current = null;
    setPanning(false);
  }

  function onKeyDown(event: KeyboardEvent<HTMLElement>) {
    if (event.altKey || event.ctrlKey || event.metaKey) return;
    const direction = panKeys[event.key];
    if (direction) pan(direction.x, direction.y);
    else if (event.key === "+" || event.key === "=")
      zoomAt(1.2, { x: 0, y: 0 });
    else if (event.key === "-") zoomAt(1 / 1.2, { x: 0, y: 0 });
    else if (event.key === "0") fit();
    else return;
    event.preventDefault();
  }

  return {
    anchor,
    strip,
    camera,
    fit,
    zoomIn: () => zoomAt(1.2, { x: 0, y: 0 }),
    zoomOut: () => zoomAt(1 / 1.2, { x: 0, y: 0 }),
    panning,
    stageProps: {
      ref: stage,
      onPointerDown,
      onPointerMove,
      onPointerUp: endPan,
      onPointerCancel: endPan,
      onLostPointerCapture: endPan,
      onKeyDown,
    },
  };
}
