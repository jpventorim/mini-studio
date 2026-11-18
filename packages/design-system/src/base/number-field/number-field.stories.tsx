import { useState } from "react";
import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { NumberField } from "./index";

const meta = {
  title: "Base/NumberField",
  component: NumberField,
  tags: ["autodocs"],
} satisfies Meta;
export default meta;

function Fields() {
  const [count, setCount] = useState(2);
  const [width, setWidth] = useState(25.4);
  return (
    <div className="rf-stack" style={{ maxWidth: 320 }}>
      <NumberField
        label="Quantity"
        labelLayout="inline"
        integer
        step={1}
        min={1}
        max={5}
        value={count}
        onChange={setCount}
      />
      <NumberField
        label="Width"
        unit="mm"
        min={0.1}
        max={200}
        value={width}
        onChange={setWidth}
      />
    </div>
  );
}
export const Default: StoryObj = { render: () => <Fields /> };
