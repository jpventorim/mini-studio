import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { Accordion } from "./index";

const meta = {
  title: "Base/Accordion",
  component: Accordion,
  tags: ["autodocs"],
  parameters: {
    docs: {
      description: {
        component:
          "Printed section headings and rules reveal content on the same sheet. Keyboard operation, configurable headings, and reduced motion remain supported.",
      },
    },
  },
} satisfies Meta;
export default meta;

export const Default: StoryObj = {
  render: () => (
    <Accordion
      items={[
        {
          value: "movement",
          title: "Movement",
          content: "Move up to your speed on your turn.",
        },
        {
          value: "actions",
          title: "Actions",
          content: "Choose an action, then resolve its effects.",
        },
      ]}
    />
  ),
};
export const Expanded: StoryObj = {
  render: () => (
    <Accordion
      defaultValue={["movement"]}
      items={[
        {
          value: "movement",
          title: "Movement",
          content: "Move up to your speed on your turn.",
        },
        {
          value: "actions",
          title: "Actions",
          content: "Choose an action, then resolve its effects.",
        },
      ]}
    />
  ),
};

export const LongSections: StoryObj = {
  render: () => (
    <div style={{ maxWidth: 360 }}>
      <h2 className="rf-title">Reference sections</h2>
      <Accordion
        headingLevel={3}
        multiple
        defaultValue={["details", "paper"]}
        items={[
          {
            value: "details",
            title: "A long section heading follows the width of the sheet",
            content: (
              <p>
                The section opens on the same sheet, with room for readable
                notes and a <a href="#paper">linked reference</a>.
              </p>
            ),
          },
          {
            value: "paper",
            title: "A second opened sheet",
            content: "Several sections can remain open for comparison.",
          },
          {
            value: "unavailable",
            title: "Unavailable section",
            content: "This section is currently unavailable.",
            disabled: true,
          },
        ]}
      />
    </div>
  ),
};
