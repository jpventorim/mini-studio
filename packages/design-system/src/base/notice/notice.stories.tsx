import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { Button } from "../button";
import { Notice } from "./index";

const meta = {
  title: "Base/Notice",
  component: Notice,
  tags: ["autodocs"],
} satisfies Meta;
export default meta;

export const Default: StoryObj = {
  render: () => <Notice>Saved print-sheet.pdf</Notice>,
};
export const ErrorWithAction: StoryObj = {
  render: () => (
    <Notice
      tone="error"
      action={
        <Button variant="ghost" size="sm">
          Dismiss
        </Button>
      }
    >
      This image does not fit the page.
    </Notice>
  ),
};
export const Inline: StoryObj = {
  render: () => (
    <Notice tone="error" variant="inline">
      Enter a positive width.
    </Notice>
  ),
};
