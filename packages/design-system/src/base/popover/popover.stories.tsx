import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { Button } from "../button";
import { Popover } from "./index";

const meta = {
  title: "Base/Popover",
  component: Popover,
  tags: ["autodocs"],
  parameters: {
    docs: {
      description: {
        component:
          "A small paper reference note with a directional pointer. Opens from a named trigger and restores focus on dismissal.",
      },
    },
  },
} satisfies Meta;
export default meta;

export const Default: StoryObj = {
  render: () => (
    <Popover
      trigger={<Button variant="outline">Open details</Button>}
      title="A useful detail"
      description="Context stays close to the control that needs it."
    >
      <p className="rf-hint">Press Escape to close.</p>
    </Popover>
  ),
};

export const LongNote: StoryObj = {
  render: () => (
    <Popover
      defaultOpen
      trigger={<Button variant="outline">Read field note</Button>}
      title="A longer note from the workshop"
      description="The sheet stays within the available space. Scroll to read the rest."
    >
      {[
        "Keep the working notes beside the tools they describe. Leave enough room for a clear heading and legible writing.",
        "The reference note stays close to its trigger while leaving room for a readable title and description.",
        "Read each measurement before adding it to the sheet. A clear annotation makes the next review easier.",
        "Keep the edge of the writing surface clear. Tools and loose offcuts belong beside the note rather than across its words.",
        "When a note grows longer than the space around it, the paper can be read by scrolling without moving the surrounding work.",
        "Close the note with its close button or press Escape to return to the control that opened it.",
      ].map((paragraph) => (
        <p key={paragraph}>{paragraph}</p>
      ))}
    </Popover>
  ),
};
