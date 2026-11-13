import type { Meta, StoryObj } from "storybook-react-rsbuild";
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
