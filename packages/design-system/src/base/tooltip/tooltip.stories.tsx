import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { Button } from "../button";
import { Tooltip } from "./index";

const meta = {
  title: "Base/Tooltip",
  component: Tooltip,
  tags: ["autodocs"],
  parameters: {
    docs: {
      description: {
        component:
          "A compact high-contrast ink label, available on keyboard focus as well as pointer hover.",
      },
    },
  },
} satisfies Meta;
export default meta;

export const Default: StoryObj = {
  render: () => (
    <Tooltip content="Save your current changes">
      <Button variant="outline">Save changes</Button>
    </Tooltip>
  ),
};

export const WrappedLabel: StoryObj = {
  render: () => (
    <Tooltip content="Save a local copy of this document. Your changes remain available when you return to the writing table.">
      <Button variant="outline">Save local copy</Button>
    </Tooltip>
  ),
};
