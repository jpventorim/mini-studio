import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { Button } from "../button";
import { Readout } from "./index";

const meta = {
  title: "Base/Readout",
  component: Readout,
  tags: ["autodocs"],
} satisfies Meta;
export default meta;

export const Default: StoryObj = {
  render: () => <Readout label="Print size" value="25.4 × 32 mm" />,
};
export const WithAction: StoryObj = {
  render: () => (
    <Readout
      label="Custom size"
      value="28 × 35 mm"
      action={
        <Button variant="ghost" size="sm">
          Reset to scale
        </Button>
      }
    />
  ),
};
