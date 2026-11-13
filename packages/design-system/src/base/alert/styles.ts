import { defineGlobalStyles } from "@pandacss/dev";

export const alertStyles = defineGlobalStyles({
  ".rf-alert": {
    "--rf-ink": "var(--rf-paper-ink)",
    "--rf-muted": "var(--rf-paper-muted)",
    "--rf-focus": "var(--rf-paper-focus)",
    "--rf-error": "var(--rf-paper-error)",
    display: "flex",
    alignItems: "flex-start",
    gap: "12px",
    padding: "16px 20px",
    color: "var(--rf-paper-ink)",
    background: "var(--rf-paper)",
    fontSize: "0.9375rem",
    lineHeight: "1.6",
  },
  ".rf-alert-icon": {
    display: "grid",
    placeItems: "center",
    flexShrink: "0",
    width: "20px",
    height: "24px",
    color: "var(--rf-accent)",
  },
  '.rf-alert[data-tone="gold"] .rf-alert-icon': {
    color: "#5a481d",
  },
  ".rf-alert-icon svg": {
    width: "20px",
    height: "20px",
    strokeWidth: "1.75",
  },
  ".rf-alert-copy": {
    minWidth: "0",
    flex: "1",
    overflowWrap: "anywhere",
  },
  ".rf-alert-title": {
    display: "block",
    fontSize: "0.9375rem",
    lineHeight: "1.6",
    fontWeight: "650",
  },
  ".rf-alert-description": {
    paddingTop: "2px",
    maxWidth: "65ch",
  },
  ".rf-alert-description > :first-child": {
    marginTop: "0",
  },
  ".rf-alert-description > :last-child": {
    marginBottom: "0",
  },
  "@media (max-width: 480px)": {
    ".rf-alert": {
      padding: "16px",
      gap: "10px",
    },
  },
  "@media (forced-colors: active)": {
    ".rf-alert": {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
    },
    ".rf-alert .rf-alert-icon": {
      color: "CanvasText",
    },
  },
});
