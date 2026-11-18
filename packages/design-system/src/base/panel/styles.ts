import { defineGlobalStyles } from "@pandacss/dev";
export const panelStyles = defineGlobalStyles({
  ".rf-panel": {
    "--rf-ink": "var(--rf-paper-ink)",
    "--rf-muted": "var(--rf-paper-muted)",
    "--rf-focus": "var(--rf-paper-focus)",
    "--rf-error": "var(--rf-paper-error)",
    position: "relative",
    minWidth: "0",
    minHeight: "72px",
    padding: "28px",
    border: "1px solid var(--rf-paper-edge)",
    borderRadius: "2px",
    color: "var(--rf-paper-ink)",
    background: "var(--rf-paper)",
    boxShadow: "inset 0 -2px 0 var(--rf-paper-light), var(--rf-depth-paper)",
    overflowWrap: "anywhere",
  },
  ".rf-panel:focus-visible": {
    outline: "3px solid var(--rf-focus)",
    outlineOffset: "4px",
  },
  '.rf-panel[data-clipped="true"]::after': {
    content: '""',
    position: "absolute",
    top: "-11px",
    right: "22px",
    width: "13px",
    height: "31px",
    border: "1.5px solid var(--rf-control-edge)",
    borderRadius: "8px",
    transform: "rotate(10deg)",
    pointerEvents: "none",
  },
  '.rf-panel[data-clipped="true"]::before': {
    content: '""',
    position: "absolute",
    top: "-7px",
    right: "25px",
    width: "7px",
    height: "21px",
    border: "1px solid var(--rf-control-edge)",
    borderTop: "0",
    borderRadius: "0 0 6px 6px",
    transform: "rotate(10deg)",
    pointerEvents: "none",
  },
  "@media (max-width: 480px)": {
    ".rf-panel": {
      padding: "24px",
    },
  },
  "@media (forced-colors: active)": {
    ".rf-panel": {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
    },
    ".rf-panel::before, .rf-panel::after": {
      display: "none",
    },
    ".rf-panel:focus-visible": {
      outlineColor: "Highlight",
    },
  },
});
