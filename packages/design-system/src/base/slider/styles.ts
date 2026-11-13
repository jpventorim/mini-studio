import { defineGlobalStyles } from "@pandacss/dev";

export const sliderStyles = defineGlobalStyles({
  ".rf-slider": {
    width: "100%",
    minWidth: "0",
  },
  ".rf-slider-heading": {
    display: "flex",
    alignItems: "baseline",
    justifyContent: "space-between",
    gap: "12px",
  },
  ".rf-slider-heading .rf-label": {
    minWidth: "0",
    overflowWrap: "anywhere",
  },
  ".rf-slider-value": {
    flexShrink: "0",
    minWidth: "3ch",
    color: "var(--rf-ink)",
    fontVariantNumeric: "tabular-nums",
    fontFamily: "ui-monospace, monospace",
    fontSize: "0.85rem",
    textAlign: "end",
  },
  ".rf-slider-control": {
    display: "flex",
    alignItems: "center",
    position: "relative",
    minHeight: "52px",
    marginInline: "22px",
    touchAction: "none",
    userSelect: "none",
    cursor: "pointer",
  },
  ".rf-slider-track": {
    position: "relative",
    width: "100%",
    height: "2px",
    background: "var(--rf-control-edge)",
    border: "0",
  },
  ".rf-slider-indicator": {
    height: "100%",
    background: "var(--rf-sapphire)",
  },
  ".rf-slider-thumb": {
    width: "44px",
    height: "44px",
    background: "transparent",
    border: "0",
    borderRadius: "2px",
    cursor: "grab",
  },
  ".rf-slider-thumb::before": {
    content: '""',
    position: "absolute",
    inset: "13px",
    border: "2px solid var(--rf-sapphire)",
    background: "var(--rf-paper)",
    borderRadius: "50%",
  },
  ".rf-slider-thumb[data-dragging]": {
    cursor: "grabbing",
  },
  ".rf-slider-thumb[data-dragging]::before": {
    background: "var(--rf-sapphire)",
  },
  ".rf-slider-thumb:focus-visible::before, .rf-slider-thumb:has(:focus-visible)::before":
    {
      outline: "3px solid var(--rf-focus)",
      outlineOffset: "3px",
    },
  ".rf-slider[data-disabled] .rf-slider-control, .rf-slider[data-disabled] .rf-slider-thumb":
    {
      cursor: "not-allowed",
    },
  ".rf-slider[data-disabled] .rf-slider-thumb::before": {
    borderColor: "var(--rf-control-edge)",
    borderStyle: "dashed",
    boxShadow: "none",
  },
  ".rf-slider[data-disabled] .rf-slider-indicator": {
    background: "var(--rf-control-edge)",
  },
  '.rf-slider[data-orientation="vertical"] .rf-slider-control': {
    width: "52px",
    height: "200px",
    marginInline: "0",
    marginBlock: "22px",
    justifyContent: "center",
  },
  '.rf-slider[data-orientation="vertical"] .rf-slider-track': {
    width: "2px",
    height: "100%",
  },
  '.rf-slider[data-orientation="vertical"] .rf-slider-indicator': {
    width: "100%",
  },
  '.rf-slider[data-orientation="vertical"] .rf-slider-thumb::before': {
    inset: "13px",
  },
  "@media (forced-colors: active)": {
    ".rf-slider-track, .rf-slider-thumb::before": {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
    },
    ".rf-slider-track": {
      background: "CanvasText",
      forcedColorAdjust: "none",
    },
    ".rf-slider-indicator": {
      background: "Highlight",
      forcedColorAdjust: "none",
    },
    ".rf-slider-thumb[data-dragging]::before": {
      background: "Highlight",
    },
    ".rf-slider-thumb:focus-visible::before, .rf-slider-thumb:has(:focus-visible)::before":
      {
        outlineColor: "Highlight",
      },
    ".rf-slider[data-disabled] .rf-slider-thumb::before": {
      borderColor: "GrayText",
    },
    ".rf-slider[data-disabled] .rf-slider-indicator": {
      background: "GrayText",
    },
  },
});
