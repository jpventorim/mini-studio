import { useState } from "react";
import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { PreviewControls } from "./index";

const meta = {
  title: "Base/PreviewControls",
  component: PreviewControls,
  tags: ["autodocs"],
} satisfies Meta;
export default meta;

function Controls({ pageCount }: { pageCount: number }) {
  const [page, setPage] = useState(0);
  const [zoom, setZoom] = useState(1);
  return (
    <PreviewControls
      page={page}
      pageCount={pageCount}
      onPageChange={setPage}
      zoom={zoom}
      onFit={() => setZoom(1)}
      onZoomIn={() => setZoom((value) => value + 0.25)}
      onZoomOut={() => setZoom((value) => value - 0.25)}
      canZoomIn={zoom < 2}
      canZoomOut={zoom > 0.25}
    />
  );
}
export const Default: StoryObj = { render: () => <Controls pageCount={3} /> };
export const SinglePage: StoryObj = {
  render: () => <Controls pageCount={1} />,
};
