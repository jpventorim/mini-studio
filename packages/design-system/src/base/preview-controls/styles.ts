import { defineGlobalStyles } from "@pandacss/dev";

export const previewControlsStyles = defineGlobalStyles({
  ".rf-preview-controls": {
    display: "flex",
    alignItems: "center",
    flexWrap: "wrap",
    justifyContent: "flex-end",
    gap: "7px",
  },
  ".rf-preview-control-group": {
    display: "flex",
    alignItems: "center",
    gap: "2px",
    margin: "0",
    minInlineSize: "0",
    padding: "1px 3px",
    border: "1px solid var(--rf-paper-edge)",
    borderRadius: "3px",
    background: "var(--rf-paper)",
    boxShadow: "var(--rf-depth-paper)",
    fontSize: "0.8125rem",
    fontVariantNumeric: "tabular-nums",
  },
  ".rf-preview-control-group .rf-button[data-icon-only]": {
    width: "34px",
    minWidth: "34px",
    minHeight: "36px",
  },
  ".rf-preview-control-divider": {
    height: "18px",
    borderInlineEnd: "1px solid var(--rf-paper-edge)",
  },
  ".rf-preview-zoom-value": { width: "4ch", textAlign: "center" },
  "@media (pointer: coarse)": {
    ".rf-preview-control-group .rf-button[data-icon-only]": {
      minWidth: "44px",
      minHeight: "44px",
    },
  },
});
