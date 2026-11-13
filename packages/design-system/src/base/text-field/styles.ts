import { defineGlobalStyles } from "@pandacss/dev";

export const textFieldStyles = defineGlobalStyles({
  ".rf-text-field": {
    gap: "8px",
    maxWidth: "100%",
  },
  ".rf-text-field-frame": {
    minWidth: "0",
    padding: "0",
    border: "0",
    background: "transparent",
    boxShadow: "none",
  },
  ".rf-text-field-input": {
    "--rf-ink": "var(--rf-paper-ink)",
    "--rf-muted": "var(--rf-paper-muted)",
    "--rf-focus": "var(--rf-paper-focus)",
    "--rf-error": "var(--rf-paper-error)",
    display: "block",
    width: "100%",
    minWidth: "0",
    minHeight: "44px",
    padding: "10px 12px",
    border: "1px solid var(--rf-control-edge)",
    borderRadius: "2px",
    color: "var(--rf-paper-ink)",
    caretColor: "var(--rf-paper-ink)",
    background: "var(--rf-paper)",
    transition: "border-color 140ms",
    boxShadow: "none",
  },
  ".rf-theme .rf-text-field-input": {
    fontSize: "max(16px, 1em)",
    lineHeight: "1.5",
  },
  ".rf-text-field-input::placeholder": {
    color: "var(--rf-paper-muted)",
    opacity: "1",
  },
  '.rf-text-field-input:hover:not(:disabled, [readonly], [aria-invalid="true"])':
    {
      borderColor: "var(--rf-paper-ink)",
    },
  ".rf-theme .rf-text-field-input:focus-visible": {
    outline: "3px solid var(--rf-focus)",
    outlineOffset: "3px",
  },
  ".rf-text-field-input:disabled": {
    background: "var(--rf-paper-light)",
    color: "var(--rf-paper-muted)",
    WebkitTextFillColor: "var(--rf-paper-muted)",
    cursor: "not-allowed",
    borderStyle: "dashed",
    opacity: "1",
  },
  ".rf-text-field-input[readonly]": {
    background: "var(--rf-paper-light)",
    borderStyle: "dashed",
  },
  ".rf-text-field > :is(.rf-label, .rf-hint, .rf-error)": {
    overflowWrap: "anywhere",
  },
  ".rf-text-field > .rf-error": {
    borderLeft: "2px solid currentColor",
    paddingLeft: "8px",
  },
  '.rf-text-field[data-surface="integrated"] .rf-text-field-input, :where(.rf-paper-surface) .rf-text-field[data-surface="auto"] .rf-text-field-input':
    {
      background: "transparent",
      border: "0",
      borderBottom: "1px solid var(--rf-control-edge)",
      borderRadius: "0",
      boxShadow: "none",
    },
  '.rf-text-field[data-surface] .rf-text-field-input[aria-invalid="true"]': {
    borderColor: "var(--rf-paper-error)",
    borderBottom: "2px solid var(--rf-paper-error)",
    boxShadow: "inset 2px 0 0 var(--rf-paper-error)",
  },
  ".rf-text-field[data-surface] .rf-text-field-input:is(:disabled, [readonly])":
    {
      borderBottomStyle: "dashed",
    },
  "@media (forced-colors: active)": {
    ".rf-text-field-input, .rf-text-field-input:is(:disabled, [readonly])": {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
    },
    ".rf-text-field-input:disabled": {
      color: "GrayText",
      WebkitTextFillColor: "GrayText",
      borderColor: "GrayText",
    },
    ".rf-text-field-input::placeholder": {
      color: "GrayText",
    },
    ".rf-theme .rf-text-field-input:focus-visible": {
      outlineColor: "Highlight",
    },
    '.rf-text-field[data-surface] .rf-text-field-input[aria-invalid="true"]': {
      borderColor: "CanvasText",
      borderStyle: "dashed",
      boxShadow: "none",
    },
  },
});
