import { defineGlobalStyles } from "@pandacss/dev";

export const textareaStyles = defineGlobalStyles({
  ".rf-textarea-field": {
    minWidth: "0",
  },
  ".rf-textarea-frame": {
    minWidth: "0",
    padding: "0",
    border: "0",
    background: "transparent",
    boxShadow: "none",
    position: "relative",
  },
  ".rf-textarea-frame::before, .rf-textarea-frame::after": {
    display: "none",
  },
  ".rf-textarea": {
    "--rf-ink": "var(--rf-paper-ink)",
    "--rf-muted": "var(--rf-paper-muted)",
    "--rf-focus": "var(--rf-paper-focus)",
    "--rf-error": "var(--rf-paper-error)",
    display: "block",
    width: "100%",
    minWidth: "0",
    minHeight: "144px",
    padding: "10px 12px",
    border: "1px solid var(--rf-control-edge)",
    borderRadius: "2px",
    color: "var(--rf-paper-ink)",
    caretColor: "var(--rf-paper-ink)",
    background:
      "repeating-linear-gradient(transparent 0 23px, var(--rf-rule) 23px 24px) 0 0 / 100% 24px content-box, linear-gradient(var(--rf-paper), var(--rf-paper)) border-box",
    transition: "border-color 140ms",
    boxShadow: "none",
    maxWidth: "100%",
    fontSize: "16px",
    lineHeight: "24px",
    resize: "vertical",
    colorScheme: "light",
  },
  ".rf-textarea::placeholder": {
    color: "var(--rf-paper-muted)",
    opacity: "1",
  },
  '.rf-textarea:hover:not(:disabled, [readonly], [aria-invalid="true"])': {
    borderColor: "var(--rf-paper-ink)",
  },
  ".rf-textarea:focus-visible": {
    outline: "3px solid var(--rf-focus)",
    outlineOffset: "3px",
  },
  ".rf-textarea:disabled": {
    background: "var(--rf-paper-light)",
    color: "var(--rf-paper-muted)",
    cursor: "not-allowed",
    borderStyle: "dashed",
    opacity: "1",
  },
  ".rf-textarea[readonly]": {
    borderStyle: "dashed",
  },
  '.rf-textarea-field[data-surface="integrated"] .rf-textarea, :where(.rf-paper-surface) .rf-textarea-field[data-surface="auto"] .rf-textarea':
    {
      background:
        "repeating-linear-gradient(transparent 0 23px, var(--rf-rule) 23px 24px) 0 0 / 100% 24px content-box",
      border: "0",
      borderBottom: "1px solid var(--rf-control-edge)",
      borderRadius: "0",
      boxShadow: "none",
    },
  '.rf-textarea-field[data-surface] .rf-textarea[aria-invalid="true"]': {
    borderColor: "var(--rf-paper-error)",
    borderBottom: "2px solid var(--rf-paper-error)",
    boxShadow: "inset 2px 0 0 var(--rf-paper-error)",
  },
  ".rf-textarea-field[data-surface] .rf-textarea:is(:disabled, [readonly])": {
    borderBottomStyle: "dashed",
  },
  "@media (forced-colors: active)": {
    ".rf-textarea": {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
    },
    ".rf-textarea::placeholder": {
      color: "GrayText",
    },
    ".rf-textarea:disabled": {
      background: "Canvas",
      color: "GrayText",
      borderColor: "CanvasText",
      boxShadow: "none",
    },
    ".rf-textarea:focus-visible": {
      outlineColor: "Highlight",
    },
    '.rf-textarea-field[data-surface] .rf-textarea[aria-invalid="true"]': {
      borderColor: "CanvasText",
      borderStyle: "dashed",
      boxShadow: "none",
    },
  },
});
