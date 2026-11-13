import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { Switch } from "./index";

const meta = {
  title: "Base/Switch",
  component: Switch,
  tags: ["autodocs"],
  parameters: {
    docs: {
      description: {
        component:
          "A compact switch with clear position and contrast. The accessible state is announced without printed ON/OFF labels.",
      },
    },
  },
} satisfies Meta;
export default meta;

export const Default: StoryObj = {
  render: () => <Switch label="Enable ambient sound" />,
};
export const On: StoryObj = {
  render: () => <Switch label="Enable ambient sound" defaultChecked />,
};
export const Disabled: StoryObj = {
  render: () => <Switch label="Enable ambient sound" disabled />,
};
export const DisabledOn: StoryObj = {
  render: () => <Switch label="Enable ambient sound" disabled defaultChecked />,
};
