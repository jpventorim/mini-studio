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
    display: "grid",
    gap: "4px",
    width: "fit-content",
    maxWidth: "100%",
    minWidth: "0",
  },
  ".rf-radio-choice": {
    display: "flex",
    alignItems: "flex-start",
    minWidth: "0",
    minHeight: "44px",
    borderRadius: "2px",
    fontSize: "0.9375rem",
    lineHeight: "24px",
    cursor: "pointer",
    color: "var(--rf-ink)",
  },
  ".rf-radio-choice-label": {
    minWidth: "0",
    overflowWrap: "anywhere",
    paddingBlock: "10px",
    paddingInlineEnd: "12px",
  },
  ".rf-radio-choice:hover:has(.rf-radio:not([data-checked], [data-disabled], [data-readonly]))":
    {
      background: "color-mix(in srgb, var(--rf-ink) 5%, transparent)",
    },
  ".rf-theme .rf-radio:focus-visible": {
    outline: "none",
  },
  ".rf-radio-choice:has(.rf-radio:focus-visible)": {
    outline: "3px solid var(--rf-focus)",
    outlineOffset: "-3px",
  },
  ".rf-radio-choice:has(.rf-radio[data-readonly])": {
    cursor: "default",
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
    background: "transparent",
    cursor: "inherit",
  },
  ".rf-radio::before": {
    content: '""',
    position: "absolute",
    width: "20px",
    height: "20px",
    border: "1px solid var(--rf-control-edge)",
    borderRadius: "50%",
    background: "var(--rf-paper)",
    pointerEvents: "none",
  },
  ".rf-radio-choice:hover .rf-radio:not([data-checked], [data-disabled], [data-readonly])::before":
    {
      borderColor: "var(--rf-ink)",
    },
  ".rf-radio[data-checked]::before": {
    border: "2px solid var(--rf-paper-focus)",
  },
  ".rf-radio-indicator": {
    position: "relative",
    width: "8px",
    height: "8px",
    borderRadius: "50%",
    background: "var(--rf-paper-focus)",
    pointerEvents: "none",
  },
  ".rf-radio[data-disabled]::before": {
    borderColor: "var(--rf-control-edge)",
    borderStyle: "dashed",
    background: "var(--rf-paper-light)",
  },
  ".rf-radio[data-disabled] .rf-radio-indicator": {
    background: "var(--rf-paper-muted)",
  },
  "@media (forced-colors: active)": {
    ".rf-radio::before": {
      background: "Canvas",
      borderColor: "ButtonText",
    },
    ".rf-radio[data-checked]::before": {
      borderColor: "Highlight",
    },
    ".rf-radio-indicator": {
      background: "Highlight",
      forcedColorAdjust: "none",
    },
    ".rf-radio-choice:has(.rf-radio:focus-visible)": {
      outlineColor: "Highlight",
    },
    ".rf-radio-choice:has(.rf-radio[data-disabled])": {
      color: "GrayText",
    },
    ".rf-radio[data-disabled]::before": {
      borderColor: "GrayText",
    },
    ".rf-radio[data-disabled] .rf-radio-indicator": {
      background: "GrayText",
    },
  },
});
