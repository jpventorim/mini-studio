import { defineGlobalStyles } from "@pandacss/dev";

export const progressStyles = defineGlobalStyles({
  ".rf-progress": {
    width: "100%",
    minWidth: "0",
  },
  ".rf-progress-heading": {
    display: "flex",
    alignItems: "baseline",
    justifyContent: "space-between",
    flexWrap: "wrap",
    gap: "12px",
    paddingBottom: "8px",
    color: "var(--rf-ink)",
  },
  ".rf-progress-heading .rf-label": {
    minWidth: "0",
    overflowWrap: "anywhere",
  },
  ".rf-progress-value": {
    marginInlineStart: "auto",
    flexShrink: "0",
    fontSize: "0.78rem",
    fontWeight: "600",
    fontVariantNumeric: "tabular-nums",
  },
  ".rf-progress-complete": {
    display: "none",
  },
  ".rf-progress[data-complete] .rf-progress-complete": {
    display: "inline",
  },
  ".rf-progress-track": {
    position: "relative",
    height: "12px",
    overflow: "hidden",
    background: "var(--rf-paper)",
    border: "1px solid var(--rf-control-edge)",
  },
  ".rf-progress-indicator": {
    height: "100%",
    background: "var(--rf-accent)",
    boxShadow: "inset -1px 0 0 var(--rf-paper-ink)",
    transition: "width 180ms var(--rf-ease)",
  },
  ".rf-progress[data-indeterminate] .rf-progress-indicator": {
    width: "30%",
    animation: "rf-loading 1.5s ease-in-out infinite alternate",
  },
  "@media (prefers-reduced-motion: reduce)": {
    ".rf-progress-indicator": {
      transition: "none",
    },
    ".rf-progress[data-indeterminate] .rf-progress-indicator": {
      animation: "none",
      transform: "none",
      width: "40%",
      marginInline: "30%",
    },
  },
  "@media (forced-colors: active)": {
    ".rf-progress-track": {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
    },
    ".rf-progress-indicator": {
      forcedColorAdjust: "none",
      background: "Highlight",
      boxShadow: "none",
    },
  },
});
