import { useState } from "react";
import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { NumberField } from "../number-field";
import { Slider } from "./index";

const meta = {
  title: "Base/Slider",
  component: Slider,
  tags: ["autodocs"],
  parameters: {
    docs: {
      description: {
        component:
          "A quiet continuous track and round thumb. The value is visible; keyboard steps remain available.",
      },
    },
  },
} satisfies Meta;
export default meta;

export const Default: StoryObj = {
  render: () => <Slider label="Ambient volume" defaultValue={40} />,
};

function EditableScale() {
  const [scale, setScale] = useState(100);
  return (
    <div style={{ maxWidth: 300 }}>
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
    </div>
  );
}

export const WithNumericInput: StoryObj = { render: () => <EditableScale /> };
export const Disabled: StoryObj = {
  render: () => <Slider label="Ambient volume" defaultValue={40} disabled />,
};
export const Stepped: StoryObj = {
  render: () => (
    <Slider label="Grid size" defaultValue={5} min={1} max={10} step={1} />
  ),
};

export const Vertical: StoryObj = {
  render: () => (
    <Slider label="Ambient volume" defaultValue={40} orientation="vertical" />
  ),
};
