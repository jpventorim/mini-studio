import { Feather, Files, Image, Settings2 } from "lucide-react";
import { type ReactNode, useState } from "react";
import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { Accordion } from "../src/base/accordion";
import { Alert } from "../src/base/alert";
import { Avatar } from "../src/base/avatar";
import { Badge } from "../src/base/badge";
import { Button } from "../src/base/button";
import { Checkbox } from "../src/base/checkbox";
import { Dialog } from "../src/base/dialog";
import { EmptyState } from "../src/base/empty-state";
import { Notice } from "../src/base/notice";
import { NumberField } from "../src/base/number-field";
import { Panel } from "../src/base/panel";
import { Popover } from "../src/base/popover";
import { PreviewControls } from "../src/base/preview-controls";
import { Progress } from "../src/base/progress";
import { RadioGroup } from "../src/base/radio-group";
import { Readout } from "../src/base/readout";
import { Select } from "../src/base/select";
import { SelectableRow } from "../src/base/selectable-row";
import { Separator } from "../src/base/separator";
import { Slider } from "../src/base/slider";
import { Switch } from "../src/base/switch";
import { Tabs } from "../src/base/tabs";
import { TextField } from "../src/base/text-field";
import { Textarea } from "../src/base/textarea";
import { Tooltip } from "../src/base/tooltip";
import { darkTheme, lightTheme } from "../src/theme/contracts";
import { useTheme } from "../src/theme/provider";
import { DimensionFieldsExample } from "./dimension-fields-example";

function Section({
  title,
  description,
  children,
}: {
  title: string;
  description: string;
  children: ReactNode;
}) {
  return (
    <section className="rf-catalog-section rf-paper-surface">
      <div className="rf-catalog-heading">
        <div>
          <h2>{title}</h2>
          <p>{description}</p>
        </div>
      </div>
      {children}
    </section>
  );
}

function Specimen({ name, children }: { name: string; children: ReactNode }) {
  return (
    <div>
      <h3 className="rf-demo-label">{name}</h3>
      {children}
    </div>
  );
}

function AllComponents() {
  const theme = useTheme() === "dark" ? darkTheme : lightTheme;
  const [quantity, setQuantity] = useState(2);
  const [scale, setScale] = useState(100);
  const [selectedImage, selectImage] = useState("Scout.png");
  const [notice, setNotice] = useState<string | null>("Saved print-sheet.pdf");
  const [page, setPage] = useState(0);
  const [zoom, setZoom] = useState(1);
  const materialSwatches = [
    { name: "Working sheet", color: theme["--rf-paper"] },
    {
      name: "Graph canvas",
      color: theme["--rf-canvas"],
      texture: "var(--rf-canvas-texture)",
    },
    { name: "Graphite", color: theme["--rf-paper-ink"] },
    { name: "Blue ink", color: "#285a88" },
    { name: "Red ink", color: "#a13742" },
    { name: "Green ink", color: "#2f684f" },
    { name: "Ochre marker", color: "#e7c66b" },
  ];
  return (
    <div className="rf-catalog">
      <header className="rf-catalog-header">
        <h1>Design system</h1>
        <p>
          Base components inspired by graph notebooks, printed reference sheets,
          and the paper around a tabletop RPG session.
        </p>
      </header>
      <Section
        title="Materials and colors"
        description="Quiet graph paper, plain working sheets, and a restrained ink palette."
      >
        <div className="rf-swatch-grid">
          {materialSwatches.map((swatch) => (
            <div key={swatch.name}>
              <div
                className="rf-swatch"
                style={{
                  background:
                    "texture" in swatch ? swatch.texture : swatch.color,
                }}
              />
              <span className="rf-swatch-name">{swatch.name}</span>
              <span className="rf-swatch-code">{swatch.color}</span>
            </div>
          ))}
        </div>
      </Section>
      <Section
        title="Actions and identity"
        description="Buttons, status labels, and avatars."
      >
        <div className="rf-demo-grid">
          <Specimen name="Button">
            <div className="rf-row">
              <Button>
                <Feather size={16} aria-hidden="true" />
                Save note
              </Button>
              <Button variant="outline">Secondary</Button>
              <Popover
                title="Print settings"
                description="Margins and spacing apply to every sheet."
                trigger={
                  <Button
                    iconOnly
                    variant="outline"
                    aria-label="Print settings"
                  >
                    <Settings2 size={18} aria-hidden="true" />
                  </Button>
                }
              />
            </div>
          </Specimen>
          <Specimen name="Badge">
            <div className="rf-row">
              <Badge tone="emerald">Available</Badge>
              <Badge tone="ruby">Requires attention</Badge>
            </div>
          </Specimen>
          <Specimen name="Avatar">
            <div className="rf-row">
              <Avatar name="Mira Vale" />
              <span className="rf-muted">Mira Vale</span>
            </div>
          </Specimen>
        </div>
      </Section>
      <Section
        title="Form controls"
        description="Text entry, selection, range, and progress controls."
      >
        <div className="rf-demo-grid">
          <Specimen name="Text field">
            <TextField
              label="Name"
              placeholder="Enter a name"
              description="Shown next to your notes."
            />
          </Specimen>
          <Specimen name="Select">
            <Select
              label="Language"
              defaultValue="common"
              items={[
                { value: "common", label: "Common" },
                { value: "elvish", label: "Elvish" },
                { value: "dwarvish", label: "Dwarvish" },
              ]}
            />
          </Specimen>
          <Specimen name="Compact select">
            <div style={{ width: 102 }}>
              <Select
                label="Paper"
                hideLabel
                compact
                surface="mounted"
                defaultValue="a4"
                items={[
                  { value: "a4", label: "A4" },
                  { value: "letter", label: "Letter" },
                ]}
              />
            </div>
          </Specimen>
          <Specimen name="Textarea">
            <Textarea label="Notes" placeholder="Enter notes" />
          </Specimen>
          <Specimen name="Checkbox">
            <Checkbox label="Keep these notes private" defaultChecked />
          </Specimen>
          <Specimen name="Switch">
            <Switch label="Enable ambient sound" defaultChecked />
          </Specimen>
          <Specimen name="Radio group">
            <RadioGroup
              label="Visibility"
              defaultValue="party"
              items={[
                { value: "private", label: "Private" },
                { value: "party", label: "Party" },
              ]}
            />
          </Specimen>
          <Specimen name="Slider">
            <NumberField
              label="Scale"
              labelLayout="inline"
              integer
              unit="%"
              min={1}
              max={400}
              value={scale}
              onChange={setScale}
            />
            <Slider
              label="Image scale"
              hideHeading
              min={1}
              max={400}
              value={scale}
              onValueChange={setScale}
            />
          </Specimen>
          <Specimen name="Progress">
            <Progress label="Preparing your workspace" value={65} />
          </Specimen>
        </div>
      </Section>
      <Section
        title="Surfaces and navigation"
        description="Panels, alerts, separators, tabs, and accordions."
      >
        <div className="rf-demo-grid">
          <Specimen name="Panel">
            <Panel clipped>
              <h4 className="rf-title">Panel</h4>
              <p className="rf-muted">A plain sheet for related content.</p>
            </Panel>
          </Specimen>
          <Specimen name="Alert">
            <Alert title="Your changes are safe" tone="emerald">
              Your changes have been saved.
            </Alert>
          </Specimen>
          <Specimen name="Separator">
            <p className="rf-muted">Note title</p>
            <Separator />
            <p className="rf-muted">Second section content.</p>
          </Specimen>
          <Specimen name="Tabs">
            <Tabs
              label="Page sections"
              items={[
                {
                  value: "notes",
                  label: "Notes",
                  content: "Notes for the selected item.",
                },
                {
                  value: "history",
                  label: "History",
                  content: "Changes to the selected item.",
                },
              ]}
            />
          </Specimen>
          <Specimen name="Accordion">
            <Accordion
              headingLevel={4}
              items={[
                {
                  value: "details",
                  title: "Section details",
                  content: "Content for the selected section.",
                },
                {
                  value: "more",
                  title: "Additional details",
                  content: "Additional section content.",
                },
              ]}
            />
          </Specimen>
        </div>
      </Section>
      <Section
        title="Print workspace"
        description="Image selection, print dimensions, and page preview controls."
      >
        <div className="rf-demo-grid">
          <Specimen name="Number field">
            <NumberField
              label="Quantity"
              labelLayout="inline"
              integer
              step={1}
              min={1}
              max={99}
              value={quantity}
              onChange={setQuantity}
            />
          </Specimen>
          <Specimen name="Selectable row">
            <div className="rf-stack">
              {["Scout.png", "Guardian.png"].map((label) => (
                <SelectableRow
                  key={label}
                  label={label}
                  selected={selectedImage === label}
                  leading={<Image size={18} aria-hidden="true" />}
                  detail={label === "Scout.png" ? `×${quantity}` : "×1"}
                  onClick={() => selectImage(label)}
                />
              ))}
            </div>
          </Specimen>
          <Specimen name="Empty state">
            <EmptyState
              variant="drop"
              title="Drop images here"
              description="or browse PNG or JPEG files"
              icon={<Files size={23} />}
              onActivate={() => setNotice("File picker requested.")}
            />
          </Specimen>
          <Specimen name="Readout">
            <Readout
              label="Print width"
              value={`${((25.4 * scale) / 100).toFixed(2)} mm`}
              action={
                scale !== 100 && (
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => setScale(100)}
                  >
                    Reset to scale
                  </Button>
                )
              }
            />
          </Specimen>
          <Specimen name="Notice">
            {notice ? (
              <Notice
                action={
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => setNotice(null)}
                  >
                    Dismiss
                  </Button>
                }
              >
                {notice}
              </Notice>
            ) : (
              <Button
                variant="outline"
                size="sm"
                onClick={() => setNotice("Saved print-sheet.pdf")}
              >
                Show notice
              </Button>
            )}
          </Specimen>
          <Specimen name="Preview controls">
            <PreviewControls
              page={page}
              pageCount={3}
              onPageChange={setPage}
              zoom={zoom}
              onFit={() => setZoom(1)}
              onZoomIn={() => setZoom((value) => value + 0.25)}
              onZoomOut={() => setZoom((value) => value - 0.25)}
              canZoomIn={zoom < 2}
              canZoomOut={zoom > 0.25}
            />
          </Specimen>
          <Specimen name="Dimension fields">
            <div
              className="rf-scroll-area"
              style={{ maxHeight: 116, padding: 4 }}
            >
              <Accordion
                headingLevel={4}
                items={[
                  {
                    value: "advanced",
                    title: "Advanced",
                    content: <DimensionFieldsExample />,
                  },
                ]}
              />
            </div>
          </Specimen>
        </div>
      </Section>
      <Section title="Overlays" description="Dialogs, popovers, and tooltips.">
        <div className="rf-demo-grid">
          <Specimen name="Dialog">
            <Dialog
              trigger={<Button>Open a dialog</Button>}
              title="Edit note"
              description="Enter a title for this note."
            >
              <TextField label="Note title" placeholder="Note title" />
            </Dialog>
          </Specimen>
          <Specimen name="Popover">
            <Popover
              trigger={<Button variant="outline">Read the details</Button>}
              title="Details"
              description="Additional information about the selected item."
            />
          </Specimen>
          <Specimen name="Tooltip">
            <Tooltip content="Save your current changes">
              <Button variant="ghost">Save changes</Button>
            </Tooltip>
          </Specimen>
        </div>
      </Section>
    </div>
  );
}

export default {
  title: "Design system/All components",
  parameters: { layout: "fullscreen", controls: { disable: true } },
} satisfies Meta;
export const Default: StoryObj = { render: () => <AllComponents /> };
