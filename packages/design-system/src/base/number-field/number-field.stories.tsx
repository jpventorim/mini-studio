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
  const [size, setSize] = useState(1);
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
        keyboardStep={0.5}
        unit="mm"
        min={0.1}
        max={200}
        value={width}
        onChange={setWidth}
      />
      <NumberField
        label="Size"
        unit="× grid"
        value={size}
        min={0.25}
        max={5}
        allowedValues={[0.25, 0.5, 1, 2, 3, 4, 5]}
        onChange={setSize}
      />
    </div>
  );
}
export const Default: StoryObj = { render: () => <Fields /> };
