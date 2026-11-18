import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { DimensionFieldsExample } from "../../../stories/dimension-fields-example";
import { DimensionFields } from "./index";

const meta = {
  title: "Base/DimensionFields",
  component: DimensionFields,
  tags: ["autodocs"],
} satisfies Meta;
export default meta;

export const Default: StoryObj = {
  render: () => (
    <div style={{ width: 280, maxWidth: "100%" }}>
      <DimensionFieldsExample />
    </div>
  ),
};

export const Narrow: StoryObj = {
  render: () => (
    <div style={{ width: 218, maxWidth: "100%" }}>
      <DimensionFieldsExample />
    </div>
  ),
};
