import { Files, ImagePlus } from "lucide-react";
import { useState } from "react";
import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { EmptyState } from "./index";

const meta = {
  title: "Base/EmptyState",
  component: EmptyState,
  tags: ["autodocs"],
} satisfies Meta;
export default meta;

function DropTarget() {
  const [activated, setActivated] = useState(false);
  return (
    <EmptyState
      variant="drop"
      icon={<Files size={23} />}
      title="Drop images here"
      description={
        activated ? "File picker requested" : "or browse PNG or JPEG files"
      }
      onActivate={() => setActivated(true)}
    />
  );
}
export const Default: StoryObj = {
  render: () => (
    <EmptyState
      icon={<ImagePlus size={24} />}
      title="Your print sheet"
      description="Add images to see them on the page."
    />
  ),
};
export const DropTargetButton: StoryObj = { render: () => <DropTarget /> };
