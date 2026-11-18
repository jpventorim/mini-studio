import { Image } from "lucide-react";
import { useState } from "react";
import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { SelectableRow } from "./index";

const meta = {
  title: "Base/SelectableRow",
  component: SelectableRow,
  tags: ["autodocs"],
} satisfies Meta;
export default meta;

function Choices() {
  const [selected, setSelected] = useState("Scout");
  return (
    <div className="rf-stack" style={{ width: 260, maxWidth: "100%" }}>
      {["Scout", "Guardian with a long file name.png"].map((label) => (
        <SelectableRow
          key={label}
          label={label}
          detail="×2"
          leading={<Image size={18} aria-hidden="true" />}
          selected={selected === label}
          onClick={() => setSelected(label)}
        />
      ))}
    </div>
  );
}
export const Default: StoryObj = { render: () => <Choices /> };
