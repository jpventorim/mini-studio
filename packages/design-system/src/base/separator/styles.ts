import { defineGlobalStyles } from "@pandacss/dev";

export const separatorStyles = defineGlobalStyles({
  ".rf-separator": {
    flexShrink: "0",
    minWidth: "0",
    height: "1px",
    margin: "20px 0",
    border: "0",
    background: "var(--rf-paper-edge)",
  },
  '.rf-separator[data-orientation="vertical"]': {
    height: "auto",
    alignSelf: "stretch",
    width: "1px",
    margin: "0 16px",
  },
  "@media (forced-colors: active)": {
    ".rf-separator": {
      background: "CanvasText",
      forcedColorAdjust: "none",
    },
  },
});
