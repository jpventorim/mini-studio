import { defineGlobalStyles } from "@pandacss/dev";

export const checkboxStyles = defineGlobalStyles({
  ".rf-checkbox-choice": {
    display: "inline-flex",
    alignItems: "center",
    gap: "6px",
    minHeight: "44px",
    maxWidth: "100%",
    fontSize: "0.9375rem",
    lineHeight: "1.6",
    cursor: "pointer",
    width: "fit-content",
  },
  ".rf-checkbox-choice > span": {
    minWidth: "0",
    overflowWrap: "anywhere",
  },
  ".rf-checkbox-choice:has(.rf-checkbox[data-disabled])": {
    cursor: "not-allowed",
    color: "var(--rf-muted)",
  },
  ".rf-checkbox": {
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
    WebkitTapHighlightColor: "transparent",
    borderRadius: "2px",
  },
  ".rf-checkbox::before": {
    content: '""',
    position: "absolute",
    width: "22px",
    height: "22px",
    boxSizing: "border-box",
    border: "1.5px solid var(--rf-control-edge)",
    background: "var(--rf-paper)",
    pointerEvents: "none",
    borderRadius: "2px",
  },
  ".rf-checkbox:hover:not([data-disabled])::before": {
    borderColor: "var(--rf-ink)",
  },
  ".rf-checkbox:focus-visible": {
    outline: "3px solid var(--rf-focus)",
    outlineOffset: "2px",
  },
  ".rf-checkbox-indicator": {
    position: "relative",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    width: "22px",
    height: "22px",
    borderRadius: "2px",
    background: "var(--rf-sapphire)",
    color: "#fffef8",
    pointerEvents: "none",
  },
  ".rf-checkbox[data-indeterminate] .rf-checkbox-indicator svg": {
    display: "none",
  },
  ".rf-checkbox[data-indeterminate] .rf-checkbox-indicator::after": {
    content: '""',
    width: "10px",
    height: "2px",
    background: "currentColor",
  },
  ".rf-checkbox[data-disabled]::before": {
    borderStyle: "dashed",
  },
  ".rf-checkbox[data-disabled] .rf-checkbox-indicator": {
    background: "var(--rf-paper-muted)",
  },
  "@media (forced-colors: active)": {
    ".rf-checkbox::before": {
      background: "Canvas",
      borderColor: "ButtonText",
    },
    ".rf-checkbox-indicator": {
      background: "Highlight",
      color: "HighlightText",
      forcedColorAdjust: "none",
    },
    ".rf-checkbox:focus-visible": {
      outlineColor: "Highlight",
    },
    ".rf-checkbox[data-disabled]::before": {
      borderColor: "GrayText",
    },
    ".rf-checkbox[data-disabled] .rf-checkbox-indicator": {
      background: "GrayText",
      color: "Canvas",
    },
  },
});
