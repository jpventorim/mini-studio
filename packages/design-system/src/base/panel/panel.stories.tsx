import type { Meta, StoryObj } from "storybook-react-rsbuild";
import { Panel } from "./index";
export default {
  title: "Base/Panel",
  component: Panel,
  tags: ["autodocs"],
  parameters: {
    docs: {
      description: {
        component:
          "A working sheet with a shallow paper edge. Use the clipped variant to give a standalone sheet a stationery clip.",
      },
    },
  },
} satisfies Meta;
export const Default: StoryObj = {
  render: () => (
    <Panel>
      <h2 className="rf-title">A blank page</h2>
      <p className="rf-muted">A place to compose your next interface.</p>
    </Panel>
  ),
};
export const Clipped: StoryObj = {
  render: () => (
    <Panel clipped>
      <h2 className="rf-title">Session notes</h2>
      <p className="rf-muted">One small clip keeps related notes together.</p>
    </Panel>
  ),
};
export const Narrow: StoryObj = {
  render: () => (
    <div style={{ width: 280, maxWidth: "100%" }}>
      <Panel>
        <h2 className="rf-title">Writing sheet</h2>
        <p className="rf-muted">
          A clear edge and readable copy at a narrow width.
        </p>
      </Panel>
    </div>
  ),
};
