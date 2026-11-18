import { defineGlobalStyles } from "@pandacss/dev";

export const selectStyles = defineGlobalStyles({
  ".rf-select-field": {
    maxWidth: "100%",
    gap: "8px",
  },
  ".rf-select-field > .rf-label": {
    overflowWrap: "anywhere",
  },
  ".rf-select-trigger": {
    "--rf-ink": "var(--rf-paper-ink)",
    "--rf-muted": "var(--rf-paper-muted)",
    "--rf-focus": "var(--rf-paper-focus)",
    "--rf-error": "var(--rf-paper-error)",
    display: "flex",
    position: "relative",
    alignItems: "center",
    width: "100%",
    minWidth: "0",
    minHeight: "46px",
    padding: "0",
    border: "1px solid var(--rf-control-edge)",
    borderRadius: "2px",
    background: "var(--rf-paper)",
    color: "var(--rf-paper-ink)",
    font: "inherit",
    textAlign: "start",
    cursor: "pointer",
  },
  ".rf-select-value": {
    fontSize: "16px",
    display: "flex",
    alignItems: "center",
    flex: "1 1 0",
    minWidth: "0",
    padding: "10px 12px",
    overflowWrap: "anywhere",
    lineHeight: "1.5",
  },
  ".rf-select-opener": {
    display: "flex",
    position: "relative",
    flex: "0 0 44px",
    minHeight: "44px",
    alignSelf: "stretch",
    alignItems: "center",
    justifyContent: "center",
    color: "var(--rf-paper-muted)",
  },
  ".rf-select-opener::before": {
    content: '""',
    position: "absolute",
    insetInlineStart: "0",
    height: "20px",
    borderInlineStart: "1px solid var(--rf-paper-edge)",
  },
  ".rf-select-trigger:hover:not([data-disabled], [data-readonly])": {
    borderColor: "var(--rf-paper-ink)",
  },
  ".rf-select-trigger[data-popup-open] .rf-select-opener svg": {
    transform: "rotate(180deg)",
  },
  ".rf-select-trigger[data-popup-open] .rf-select-opener": {
    color: "var(--rf-paper-focus)",
  },
  ".rf-theme .rf-select-trigger:focus-visible": {
    outline: "3px solid var(--rf-focus)",
    outlineOffset: "3px",
  },
  ".rf-select-value[data-placeholder]": {
    color: "var(--rf-paper-muted)",
  },
  ".rf-select-trigger[data-readonly]": {
    cursor: "default",
    borderStyle: "dashed",
  },
  ".rf-select-trigger[data-disabled]": {
    cursor: "not-allowed",
    borderStyle: "dashed",
    background: "var(--rf-paper-light)",
    color: "var(--rf-paper-muted)",
  },
  ".rf-select-positioner": {
    minWidth: "0",
  },
  ".rf-select-popup": {
    "--rf-ink": "var(--rf-paper-ink)",
    "--rf-muted": "var(--rf-paper-muted)",
    "--rf-focus": "var(--rf-paper-focus)",
    "--rf-error": "var(--rf-paper-error)",
    position: "relative",
    width:
      "min(max(12rem, var(--anchor-width)), var(--available-width), calc(100vw - 24px))",
    maxHeight: "var(--available-height)",
    overflow: "auto",
    overscrollBehavior: "contain",
    background: "var(--rf-paper)",
    color: "var(--rf-paper-ink)",
    border: "1px solid var(--rf-paper-edge)",
    borderRadius: "2px",
    boxShadow: "var(--rf-depth-overlay)",
    transformOrigin: "var(--transform-origin)",
    transition: "opacity 140ms",
  },
  ".rf-select-popup:is([data-starting-style], [data-ending-style])": {
    opacity: "0",
  },
  ".rf-select-list": {
    display: "grid",
    gap: "2px",
    padding: "4px",
  },
  ".rf-select-item": {
    display: "grid",
    gridTemplateColumns: "minmax(0, 1fr) 16px",
    alignItems: "center",
    gap: "12px",
    padding: "10px 12px",
    minHeight: "44px",
    cursor: "default",
    outline: "none",
  },
  ".rf-select-item-text": {
    minWidth: "0",
    overflowWrap: "anywhere",
  },
  ".rf-select-check": {
    display: "flex",
    color: "var(--rf-paper-focus)",
  },
  ".rf-select-item[data-selected]": {
    fontWeight: "600",
    background: "color-mix(in srgb, var(--rf-paper-focus) 9%, transparent)",
  },
  ".rf-select-item[data-highlighted]:not([data-selected])": {
    background: "var(--rf-paper-light)",
  },
  ".rf-theme .rf-select-item[data-highlighted], .rf-theme .rf-select-item:focus-visible":
    {
      outline: "2px solid var(--rf-paper-focus)",
      outlineOffset: "-2px",
    },
  ".rf-select-item[data-disabled]": {
    color: "var(--rf-paper-muted)",
    textDecoration: "line-through",
  },
  '.rf-select-field[data-surface="integrated"] .rf-select-trigger, :where(.rf-paper-surface) .rf-select-field[data-surface="auto"] .rf-select-trigger':
    {
      background: "transparent",
      border: "0",
      borderBottom: "1px solid var(--rf-control-edge)",
      borderRadius: "0",
      boxShadow: "none",
    },
  '.rf-select-field[data-surface] .rf-select-trigger[data-popup-open]:not([aria-invalid="true"])':
    {
      borderColor: "var(--rf-paper-focus)",
      boxShadow: "inset 0 -1px 0 var(--rf-paper-focus)",
    },
  '.rf-select-field[data-surface] .rf-select-trigger[aria-invalid="true"]': {
    borderColor: "var(--rf-paper-error)",
    borderBottom: "2px solid var(--rf-paper-error)",
    boxShadow: "inset 2px 0 0 var(--rf-paper-error)",
  },
  ".rf-select-field[data-surface] .rf-select-trigger:is([data-disabled], [data-readonly])":
    {
      borderBottomStyle: "dashed",
    },
  "@media (forced-colors: active)": {
    ".rf-select-trigger, .rf-select-popup": {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
    },
    ".rf-select-opener, .rf-select-check": {
      color: "CanvasText",
    },
    ".rf-theme .rf-select-trigger:focus-visible": {
      outlineColor: "Highlight",
    },
    ".rf-theme .rf-select-item[data-highlighted], .rf-theme .rf-select-item:focus-visible":
      {
        background: "Highlight",
        color: "HighlightText",
        outlineColor: "HighlightText",
        forcedColorAdjust: "none",
      },
    ".rf-select-item:is([data-highlighted], :focus-visible) .rf-select-check": {
      color: "HighlightText",
    },
    '.rf-select-field[data-surface] .rf-select-trigger[data-popup-open]:not([aria-invalid="true"])':
      {
        borderColor: "Highlight",
        boxShadow: "none",
      },
    ".rf-select-trigger[data-disabled], .rf-select-item[data-disabled]": {
      color: "GrayText",
      borderColor: "GrayText",
    },
    '.rf-select-field[data-surface] .rf-select-trigger[aria-invalid="true"]': {
      borderColor: "CanvasText",
      borderStyle: "dashed",
      boxShadow: "none",
    },
  },
});
