import { defineGlobalStyles } from "@pandacss/dev";

export const buttonStyles = defineGlobalStyles({
  ".rf-button": {
    appearance: "none",
    display: "inline-flex",
    justifyContent: "center",
    alignItems: "center",
    gap: "8px",
    maxWidth: "100%",
    minHeight: "44px",
    minWidth: "44px",
    padding: "10px 20px",
    border: "1px solid var(--rf-accent)",
    borderRadius: "2px",
    cursor: "pointer",
    color: "var(--rf-on-accent)",
    fontWeight: "650",
    fontSize: "0.87rem",
    lineHeight: "1.4",
    background: "var(--rf-accent)",
    overflowWrap: "anywhere",
    textAlign: "center",
    transition: "background-color 140ms, box-shadow 140ms, transform 140ms",
  },
  ".rf-button:hover:not(:disabled, [data-disabled])": {
    boxShadow: "inset 0 0 0 100px #00000012",
  },
  ".rf-button:active:not(:disabled, [data-disabled])": {
    transform: "translateY(1px)",
    boxShadow: "var(--rf-depth-pressed)",
  },
  '.rf-button[data-variant="outline"]': {
    background: "var(--rf-paper)",
    color: "var(--rf-paper-ink)",
    borderColor: "var(--rf-control-edge)",
  },
  '.rf-button[data-variant="ghost"]': {
    background: "transparent",
    color: "var(--rf-ink)",
    borderColor: "transparent",
  },
  '.rf-button[data-variant="ghost"]:hover:not(:disabled, [data-disabled])': {
    textDecoration: "underline",
    textUnderlineOffset: "4px",
  },
  '.rf-button[data-size="sm"]': {
    padding: "6px 12px",
    fontSize: "0.78rem",
  },
  ".rf-button[data-icon-only]": {
    width: "44px",
    padding: "0",
  },
  ".rf-button:focus-visible": {
    outline: "3px solid var(--rf-focus)",
    outlineOffset: "3px",
  },
  ".rf-button:is(:disabled, [data-disabled])": {
    background: "var(--rf-paper-light)",
    color: "var(--rf-paper-muted)",
    border: "1px dashed var(--rf-control-edge)",
    cursor: "not-allowed",
    boxShadow: "none",
  },
  '.rf-button[data-variant="ghost"][data-icon-only]:is(:disabled, [data-disabled])':
    {
      background: "transparent",
      borderColor: "transparent",
    },
  "@media (prefers-reduced-motion: reduce)": {
    ".rf-button": {
      transition: "none",
    },
    ".rf-button:active:not(:disabled, [data-disabled])": {
      transform: "none",
    },
  },
  "@media (forced-colors: active)": {
    ".rf-button, .rf-button[data-variant]": {
      background: "ButtonFace",
      color: "ButtonText",
      borderColor: "ButtonText",
      boxShadow: "none",
    },
    ".rf-button:focus-visible": {
      outlineColor: "Highlight",
    },
    ".rf-button:is(:disabled, [data-disabled])": {
      color: "GrayText",
      borderColor: "GrayText",
    },
  },
});
