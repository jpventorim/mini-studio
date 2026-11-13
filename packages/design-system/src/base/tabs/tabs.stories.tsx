import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { Tabs } from "./index";

const meta = {
  title: "Base/Tabs",
  component: Tabs,
  tags: ["autodocs"],
  parameters: {
    docs: {
      description: {
        component:
          "A simple section index with a selected underline and unframed content. Arrow keys move focus; Enter or Space activates.",
      },
    },
  },
} satisfies Meta;
export default meta;

export const Default: StoryObj = {
  render: () => (
    <Tabs
      label="Notes navigation"
      items={[
        { value: "notes", label: "Notes", content: "Your collected notes." },
        {
          value: "history",
          label: "History",
          content: "Nothing has been forgotten.",
        },
        {
          value: "locked",
          label: "Locked",
          content: "Hidden content",
          disabled: true,
        },
      ]}
    />
  ),
};
export const SecondTab: StoryObj = {
  render: () => (
    <Tabs
      label="Notes navigation"
      defaultValue="history"
      items={[
        { value: "notes", label: "Notes", content: "Your collected notes." },
        {
          value: "history",
          label: "History",
          content: "Nothing has been forgotten.",
        },
      ]}
    />
  ),
};

export const NarrowLabels: StoryObj = {
  render: () => (
    <div style={{ maxWidth: 300 }}>
      <Tabs
        label="Notes navigation"
        items={[
          {
            value: "notes",
            label: "Collected notes",
            content: "Your collected notes.",
          },
          {
            value: "history",
            label: "Previous entries",
            content: "Earlier note entries.",
          },
          {
            value: "references",
            label: "References",
            content: "Supporting references.",
          },
          {
            value: "unavailable",
            label: "Unavailable",
            content: "Unavailable",
            disabled: true,
          },
        ]}
      />
    </div>
  ),
};
