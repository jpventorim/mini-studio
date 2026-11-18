import {
  Button,
  Checkbox,
  EmptyState,
  Notice,
  NumberField,
  Panel,
  Popover,
  Select,
  SelectableRow,
  ThemeProvider,
} from "@rust-foundry/ui";
import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import {
  FileDown,
  Files,
  ImagePlus,
  PanelLeftClose,
  PanelLeftOpen,
  Settings2,
} from "lucide-react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { ImageInspector } from "./ImageInspector";
import { PrintPreview } from "./PrintPreview";
import {
  defaultSettings,
  desktop,
  type PreviewPage,
  type PrintError,
  type PrintImage,
  type PrintRequest,
  pageDimensions,
  paperSizes,
  printError,
  printRequest,
  type SheetSettings,
} from "./print-workspace";
import "@rust-foundry/ui/styles.css";
import "./workspace.css";

const paperOptions = Object.entries(paperSizes).map(([value, { label }]) => ({
  value,
  label,
}));

type PreviewResult = {
  request: PrintRequest;
  error?: PrintError;
};

function App() {
  const [settings, setSettings] = useState(defaultSettings);
  const [images, setImages] = useState<PrintImage[]>([]);
  const [selectedId, setSelectedId] = useState<number>();
  const [pages, setPages] = useState<PreviewPage[]>([]);
  const [preview, setPreview] = useState<PreviewResult>();
  const [importing, setImporting] = useState(false);
  const [dragging, setDragging] = useState(false);
  const [exporting, setExporting] = useState(false);
  const [notice, setNotice] = useState<{ text: string; error?: boolean }>();
  const [imagesOpen, setImagesOpen] = useState(true);
  const importActive = useRef(false);
  const previewQueue = useRef(Promise.resolve());

  const request = useMemo(
    () => printRequest(images, settings),
    [images, settings],
  );
  const selected = images.find((image) => image.id === selectedId) ?? images[0];
  const current = preview?.request === request;
  const pending = images.length > 0 && !current;
  const problem = current ? preview.error : undefined;
  const problemImage = images.find((image) => image.id === problem?.imageId);
  const problemText =
    problemImage && problem
      ? `${problemImage.name} — ${problem.message}`
      : problem?.message;

  function updateSettings(patch: Partial<SheetSettings>) {
    setSettings((previous) => ({ ...previous, ...patch }));
  }

  const addImages = useCallback(async (paths?: string[]) => {
    if (importActive.current) return;
    if (!isTauri()) {
      setNotice({ text: "Open the desktop app to add images.", error: true });
      return;
    }
    importActive.current = true;
    setImporting(true);
    setNotice(undefined);
    try {
      const result = await (paths
        ? desktop.importImages(paths)
        : desktop.chooseImages());
      const added = result.images.map((image) => ({
        ...image,
        quantity: 1,
        scale: 100,
        lockRatio: true,
      }));
      if (added.length) {
        setImages((previous) => [...previous, ...added]);
        setSelectedId(added[0].id);
        setImagesOpen(true);
      }
      if (result.errors.length)
        setNotice({ text: result.errors.join("\n"), error: true });
    } catch (error) {
      setNotice({ text: printError(error).message, error: true });
    } finally {
      importActive.current = false;
      setImporting(false);
    }
  }, []);

  useEffect(() => {
    if (!isTauri()) return;
    const listener = getCurrentWebview().onDragDropEvent(({ payload }) => {
      setDragging(payload.type === "enter" || payload.type === "over");
      if (payload.type === "drop") void addImages(payload.paths);
    });
    void listener.catch((error: unknown) => {
      setNotice({ text: printError(error).message, error: true });
    });
    return () => {
      void listener.then(
        (unlisten) => unlisten(),
        () => {},
      );
    };
  }, [addImages]);

  useEffect(() => {
    if (!request.items.length) return;
    let active = true;
    const timeout = window.setTimeout(() => {
      const update = async () => {
        if (!active) return;
        try {
          const result = await desktop.preview(request);
          if (active) {
            setPages(result);
            setPreview({ request });
          }
        } catch (error) {
          if (active) setPreview({ request, error: printError(error) });
        }
      };
      previewQueue.current = previewQueue.current.then(update);
    }, 100);
    return () => {
      active = false;
      window.clearTimeout(timeout);
    };
  }, [request]);

  function updateSelected(patch: Partial<PrintImage>) {
    setImages((previous) =>
      previous.map((image) =>
        image.id === selected?.id ? { ...image, ...patch } : image,
      ),
    );
  }

  function removeSelected() {
    if (!selected) return;
    setImages((previous) =>
      previous.filter((image) => image.id !== selected.id),
    );
    // Keep a snapshot while repacking, without showing copies of a removed image.
    if (images.length === 1) setPages([]);
    setSelectedId(undefined);
    window.requestAnimationFrame(() => {
      document
        .querySelector<HTMLElement>(
          ".rf-selectable-row[aria-pressed='true'], .add-images",
        )
        ?.focus();
    });
    void desktop.removeImage(selected.id).catch((error: unknown) => {
      setNotice({ text: printError(error).message, error: true });
    });
  }

  async function exportPdf() {
    setExporting(true);
    setNotice(undefined);
    try {
      const path = await desktop.export(request);
      if (path) setNotice({ text: `Saved ${path.split(/[\\/]/).pop()}` });
    } catch (error) {
      setNotice({ text: printError(error).message, error: true });
    } finally {
      setExporting(false);
    }
  }

  return (
    <ThemeProvider className="rf-workbench" theme="light">
      <main className="print-workspace" data-images-open={imagesOpen}>
        <header className="workspace-toolbar">
          <h1 className="workspace-brand">
            <img src="/rusty_bob.svg" alt="" width={23} height={33} />
            Rust Foundry
          </h1>
          <Panel className="paper-controls">
            <Select
              label="Paper"
              hideLabel
              compact
              surface="mounted"
              value={settings.paper}
              items={paperOptions}
              onValueChange={(paper) => {
                if (paper === "a4" || paper === "letter")
                  updateSettings({ paper });
              }}
            />
            <Popover
              title="Print settings"
              description="Apply to every sheet in the PDF."
              trigger={
                <Button
                  type="button"
                  variant="outline"
                  iconOnly
                  aria-label="Print settings"
                  title="Print settings"
                >
                  <Settings2 size={18} aria-hidden="true" />
                </Button>
              }
            >
              <div className="print-settings">
                <NumberField
                  label="Image DPI"
                  value={settings.dpi}
                  min={36}
                  max={2400}
                  integer
                  onChange={(dpi) => updateSettings({ dpi })}
                />
                <div className="dimension-fields">
                  <NumberField
                    label="Margin"
                    unit="mm"
                    value={settings.marginMm}
                    min={0}
                    max={100}
                    onChange={(marginMm) => updateSettings({ marginMm })}
                  />
                  <NumberField
                    label="Spacing"
                    unit="mm"
                    value={settings.spacingMm}
                    min={0}
                    max={100}
                    onChange={(spacingMm) => updateSettings({ spacingMm })}
                  />
                </div>
                <Checkbox
                  label="Allow 90° rotation"
                  checked={settings.allowRotation}
                  onCheckedChange={(allowRotation) =>
                    updateSettings({ allowRotation })
                  }
                />
              </div>
            </Popover>
            <Button
              type="button"
              className="export-button"
              disabled={pending || !!problem || !images.length || exporting}
              onClick={() => {
                void exportPdf();
              }}
            >
              <FileDown size={17} aria-hidden="true" />
              {exporting ? "Exporting…" : "Export PDF…"}
            </Button>
          </Panel>
        </header>

        <aside className="workspace-sidebar" aria-labelledby="images-heading">
          <Panel className="image-panel">
            <header className="image-toolbar">
              <div>
                <h2 id="images-heading">Images</h2>
                {imagesOpen && (
                  <p>
                    {images.length} {images.length === 1 ? "source" : "sources"}
                  </p>
                )}
              </div>
              <div className="image-toolbar-actions">
                {imagesOpen && images.length > 0 && (
                  <Button
                    type="button"
                    variant="outline"
                    className="add-images"
                    disabled={importing}
                    onClick={() => {
                      void addImages();
                    }}
                  >
                    <ImagePlus size={17} aria-hidden="true" />
                    {importing ? "Adding…" : "Add images"}
                  </Button>
                )}
                <Button
                  type="button"
                  iconOnly
                  variant="ghost"
                  aria-label={imagesOpen ? "Collapse images" : "Expand images"}
                  aria-expanded={imagesOpen}
                  aria-controls="image-editor"
                  onClick={() => setImagesOpen((open) => !open)}
                >
                  {imagesOpen ? (
                    <PanelLeftClose size={16} aria-hidden="true" />
                  ) : (
                    <PanelLeftOpen size={16} aria-hidden="true" />
                  )}
                </Button>
              </div>
            </header>

            <div
              className="image-editor-scroll"
              id="image-editor"
              hidden={!imagesOpen}
            >
              {images.length ? (
                <ul className="asset-list" aria-label="Imported images">
                  {images.map((image) => (
                    <li key={image.id}>
                      <SelectableRow
                        selected={selected?.id === image.id}
                        onClick={() => setSelectedId(image.id)}
                        title={image.name}
                        label={image.name}
                        leading={<img src={image.preview} alt="" />}
                        detail={`×${image.quantity}`}
                      />
                    </li>
                  ))}
                </ul>
              ) : (
                <EmptyState
                  variant="drop"
                  className="add-images empty-assets"
                  title={importing ? "Adding images…" : "Drop images here"}
                  description="or browse PNG or JPEG files"
                  icon={<Files size={23} strokeWidth={1.4} />}
                  onActivate={() => {
                    void addImages();
                  }}
                  disabled={importing}
                />
              )}
              {selected && (
                <ImageInspector
                  key={selected.id}
                  image={selected}
                  dpi={settings.dpi}
                  problem={
                    problem?.imageId === selected.id
                      ? problem.message
                      : undefined
                  }
                  update={updateSelected}
                  remove={removeSelected}
                />
              )}
            </div>
          </Panel>
        </aside>

        <PrintPreview
          pages={pages}
          images={images}
          size={pageDimensions(settings)}
          pending={pending}
          problem={problemText}
        />

        {notice && (
          <Notice
            className="workspace-notice"
            tone={notice.error ? "error" : "neutral"}
            action={
              <Button
                type="button"
                size="sm"
                variant="ghost"
                onClick={() => setNotice(undefined)}
              >
                Dismiss
              </Button>
            }
          >
            {notice.text}
          </Notice>
        )}
        {dragging && (
          <div className="drop-overlay">
            <span>Drop to add images</span>
          </div>
        )}
      </main>
    </ThemeProvider>
  );
}

export default App;
