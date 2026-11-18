import { useState } from "react";
import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { PreviewControls } from "./index";

const meta = {
  title: "Base/PreviewControls",
  component: PreviewControls,
  tags: ["autodocs"],
} satisfies Meta;
export default meta;

function Controls({ fitLabel }: { fitLabel?: string }) {
  const [zoom, setZoom] = useState(1);
  return (
    <PreviewControls
      zoom={zoom}
      fitLabel={fitLabel}
      onFit={() => setZoom(1)}
      onZoomIn={() => setZoom((value) => value + 0.25)}
      onZoomOut={() => setZoom((value) => value - 0.25)}
      canZoomIn={zoom < 2}
      canZoomOut={zoom > 0.25}
    />
  );
}
export const Default: StoryObj = { render: () => <Controls /> };
export const MultipleSheets: StoryObj = {
  render: () => <Controls fitLabel="Fit sheets" />,
};
