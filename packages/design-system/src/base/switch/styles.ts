import { defineGlobalStyles } from "@pandacss/dev";

export const switchStyles = defineGlobalStyles({
  ".rf-switch-choice": {
    display: "inline-flex",
    alignItems: "center",
    gap: "10px",
    minHeight: "44px",
    maxWidth: "100%",
    fontSize: "0.9375rem",
    lineHeight: "1.6",
    cursor: "pointer",
  },
  ".rf-switch-choice > span": {
    minWidth: "0",
    overflowWrap: "anywhere",
  },
  ".rf-switch": {
    position: "relative",
    display: "inline-flex",
    width: "48px",
    height: "44px",
    flex: "0 0 auto",
    padding: "0",
    border: "0",
    borderRadius: "4px",
    background: "transparent",
    cursor: "pointer",
  },
  ".rf-switch::before": {
    content: '""',
    position: "absolute",
    inset: "9px 0",
    borderRadius: "14px",
    border: "1px solid var(--rf-control-edge)",
    background: "var(--rf-paper)",
    transition: "background-color 160ms",
  },
  ".rf-switch-thumb": {
    position: "absolute",
    top: "13px",
    left: "4px",
    width: "18px",
    height: "18px",
    borderRadius: "50%",
    background: "var(--rf-control-edge)",
    transition: "transform 160ms var(--rf-ease)",
  },
  ".rf-switch[data-checked]::before": {
    background: "var(--rf-sapphire)",
    borderColor: "var(--rf-sapphire)",
  },
  ".rf-switch[data-checked] .rf-switch-thumb": {
    transform: "translateX(22px)",
    background: "#fffef8",
  },
  ".rf-switch:focus-visible": {
    outline: "3px solid var(--rf-focus)",
    outlineOffset: "3px",
  },
  ".rf-switch:hover:not([data-disabled])::before": {
    borderColor: "var(--rf-ink)",
  },
  ".rf-switch[data-disabled]": {
    cursor: "not-allowed",
  },
  ".rf-switch[data-disabled]::before": {
    borderStyle: "dashed",
    borderColor: "var(--rf-control-edge)",
    background: "var(--rf-paper-light)",
  },
  ".rf-switch[data-disabled] .rf-switch-thumb": {
    background: "var(--rf-paper-muted)",
  },
  ".rf-switch-choice:has([data-disabled])": {
    cursor: "not-allowed",
    color: "var(--rf-muted)",
  },
  "@media (prefers-reduced-motion: reduce)": {
    ".rf-switch-thumb, .rf-switch::before": {
      transition: "none",
    },
  },
  "@media (forced-colors: active)": {
    ".rf-switch::before": {
      background: "Canvas",
      borderColor: "ButtonText",
    },
    ".rf-switch-thumb": {
      background: "ButtonText",
      forcedColorAdjust: "none",
    },
    ".rf-switch[data-checked]::before": {
      background: "Highlight",
      borderColor: "Highlight",
      forcedColorAdjust: "none",
    },
    ".rf-switch[data-checked] .rf-switch-thumb": {
      background: "HighlightText",
    },
    ".rf-switch[data-disabled]::before": {
      background: "Canvas",
      borderColor: "GrayText",
    },
    ".rf-switch[data-disabled] .rf-switch-thumb": {
      background: "GrayText",
    },
    ".rf-switch:focus-visible": {
      outlineColor: "Highlight",
    },
  },
});
