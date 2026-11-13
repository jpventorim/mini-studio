import { defineGlobalStyles } from "@pandacss/dev";

export const dialogStyles = defineGlobalStyles({
  ".rf-dialog-backdrop": {
    position: "fixed",
    inset: "0",
    zIndex: "200",
    background: "#282d3266",
    transition: "opacity 160ms ease-out",
  },
  ".rf-dialog-backdrop:is([data-starting-style], [data-ending-style])": {
    opacity: "0",
  },
  ".rf-dialog": {
    "--rf-ink": "var(--rf-paper-ink)",
    "--rf-muted": "var(--rf-paper-muted)",
    "--rf-focus": "var(--rf-paper-focus)",
    "--rf-error": "var(--rf-paper-error)",
    position: "fixed",
    top: "50%",
    left: "50%",
    zIndex: "201",
    transform: "translate(-50%, -50%)",
    width: "min(520px, calc(100vw - 32px))",
    maxHeight: "calc(100dvh - 48px)",
    overflowY: "auto",
    overscrollBehavior: "contain",
    scrollbarGutter: "stable",
    padding: "0",
    border: "1px solid var(--rf-paper-edge)",
    borderRadius: "2px",
    background: "var(--rf-paper)",
    color: "var(--rf-paper-ink)",
    boxShadow: "var(--rf-depth-overlay)",
    transition: "opacity 160ms ease-out",
  },
  ".rf-dialog:focus-visible": {
    outline: "3px solid var(--rf-paper-focus)",
    outlineOffset: "3px",
  },
  ".rf-dialog:is([data-starting-style], [data-ending-style])": {
    opacity: "0",
  },
  ".rf-dialog-sheet": {
    "--rf-ink": "var(--rf-paper-ink)",
    "--rf-muted": "var(--rf-paper-muted)",
    "--rf-focus": "var(--rf-paper-focus)",
    "--rf-error": "var(--rf-paper-error)",
    position: "relative",
    minWidth: "0",
    padding: "28px",
    background: "var(--rf-paper)",
    color: "var(--rf-paper-ink)",
  },
  ".rf-dialog-header": {
    display: "flex",
    alignItems: "flex-start",
    gap: "12px",
    borderBottom: "1px solid var(--rf-paper-edge)",
    paddingBottom: "12px",
  },
  ".rf-dialog-title": {
    flex: "1",
    minWidth: "0",
    margin: "8px 0 0",
    fontFamily: "var(--rf-font-display)",
    fontSize: "24px",
    fontWeight: "500",
    lineHeight: "1.25",
    overflowWrap: "anywhere",
  },
  ".rf-dialog-close": {
    display: "inline-flex",
    alignItems: "center",
    justifyContent: "center",
    flexShrink: "0",
    width: "44px",
    height: "44px",
    padding: "0",
    border: "1px solid transparent",
    borderRadius: "2px",
    background: "transparent",
    color: "var(--rf-paper-ink)",
    cursor: "pointer",
  },
  ".rf-dialog-close:hover": {
    background: "var(--rf-paper-light)",
    borderColor: "var(--rf-control-edge)",
  },
  ".rf-dialog-close:focus-visible": {
    outline: "3px solid var(--rf-paper-focus)",
    outlineOffset: "3px",
  },
  ".rf-dialog-close:disabled": {
    cursor: "not-allowed",
    color: "var(--rf-paper-muted)",
  },
  ".rf-dialog-description": {
    margin: "16px 0 0",
    color: "var(--rf-paper-muted)",
    overflowWrap: "anywhere",
  },
  ".rf-dialog-content": {
    marginTop: "20px",
    minWidth: "0",
    overflowWrap: "anywhere",
  },
  "@media (max-width: 480px)": {
    ".rf-dialog": {
      width: "calc(100vw - 24px)",
      maxHeight: "calc(100dvh - 24px)",
    },
    ".rf-dialog-sheet": {
      padding: "20px",
    },
  },
  "@media (prefers-reduced-motion: reduce)": {
    ".rf-dialog, .rf-dialog-backdrop": {
      transition: "none",
    },
  },
  "@media (forced-colors: active)": {
    ".rf-dialog, .rf-dialog-sheet, .rf-dialog-close": {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
    },
    ".rf-dialog-header": {
      borderColor: "CanvasText",
    },
    ".rf-dialog-description": {
      color: "CanvasText",
    },
    ".rf-dialog:focus-visible, .rf-dialog-close:focus-visible": {
      outlineColor: "Highlight",
    },
    ".rf-dialog-close:disabled": {
      color: "GrayText",
    },
  },
});
