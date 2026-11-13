import { defineGlobalStyles } from "@pandacss/dev";

export const radioGroupStyles = defineGlobalStyles({
  ".rf-radio-field": {
    display: "grid",
    gap: "4px",
    minWidth: "0",
  },
  ".rf-radio-label": {
    color: "var(--rf-ink)",
    fontSize: "var(--rf-label-size)",
    fontWeight: "600",
    overflowWrap: "anywhere",
  },
  ".rf-radio-group": {
    display: "flex",
    flexWrap: "wrap",
    gap: "0 20px",
    minWidth: "0",
  },
  ".rf-radio-choice": {
    display: "inline-flex",
    alignItems: "flex-start",
    gap: "0",
    minHeight: "44px",
    maxWidth: "100%",
    fontSize: "0.9375rem",
    lineHeight: "24px",
    cursor: "pointer",
    color: "var(--rf-ink)",
  },
  ".rf-radio-choice > span": {
    minWidth: "0",
    overflowWrap: "anywhere",
    paddingBlock: "10px",
  },
  ".rf-radio-choice:has(.rf-radio[data-checked]) > span": {
    fontWeight: "600",
  },
  ".rf-radio-choice:has(.rf-radio[data-disabled])": {
    cursor: "not-allowed",
    color: "var(--rf-muted)",
  },
  ".rf-radio": {
    position: "relative",
    display: "inline-flex",
    alignItems: "center",
    justifyContent: "center",
    flex: "0 0 44px",
    width: "44px",
    height: "44px",
    padding: "0",
    border: "0",
    borderRadius: "4px",
    background: "transparent",
    cursor: "inherit",
  },
  ".rf-radio::before": {
    content: '""',
    position: "absolute",
    width: "18px",
    height: "18px",
    border: "1px solid var(--rf-control-edge)",
    borderRadius: "50%",
    background: "var(--rf-paper)",
    pointerEvents: "none",
  },
  ".rf-radio[data-checked]::before": {
    border: "2px solid var(--rf-sapphire)",
  },
  ".rf-radio:hover:not([data-disabled])::before": {
    borderColor: "var(--rf-ink)",
  },
  ".rf-radio:focus-visible": {
    outline: "3px solid var(--rf-focus)",
    outlineOffset: "3px",
  },
  ".rf-radio-indicator": {
    position: "relative",
    width: "8px",
    height: "8px",
    borderRadius: "50%",
    background: "var(--rf-sapphire)",
    pointerEvents: "none",
  },
  ".rf-radio[data-disabled]::before": {
    borderColor: "var(--rf-control-edge)",
    background: "var(--rf-paper-light)",
  },
  ".rf-radio[data-disabled] .rf-radio-indicator": {
    background: "var(--rf-paper-muted)",
  },
  "@media (forced-colors: active)": {
    ".rf-radio::before, .rf-radio[data-checked]::before": {
      background: "Canvas",
      borderColor: "ButtonText",
    },
    ".rf-radio-indicator": {
      background: "Highlight",
      forcedColorAdjust: "none",
    },
    ".rf-radio:focus-visible": {
      outlineColor: "Highlight",
    },
    ".rf-radio[data-disabled]::before": {
      borderColor: "GrayText",
    },
    ".rf-radio[data-disabled] .rf-radio-indicator": {
      background: "GrayText",
    },
  },
});
