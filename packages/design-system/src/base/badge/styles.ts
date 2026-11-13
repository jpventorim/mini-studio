import { defineGlobalStyles } from "@pandacss/dev";

export const badgeStyles = defineGlobalStyles({
  ".rf-badge": {
    "--rf-ink": "var(--rf-paper-ink)",
    "--rf-muted": "var(--rf-paper-muted)",
    "--rf-focus": "var(--rf-paper-focus)",
    "--rf-error": "var(--rf-paper-error)",
    display: "inline-flex",
    alignItems: "center",
    verticalAlign: "middle",
    gap: "4px",
    minWidth: "0",
    maxWidth: "100%",
    overflowWrap: "anywhere",
    border: "0",
    borderRadius: "3px",
    background: "var(--rf-tint)",
    color: "var(--rf-accent)",
    padding: "3px 7px",
    fontSize: "0.75rem",
    fontWeight: "600",
    lineHeight: "1.4",
  },
  '.rf-badge[data-tone="gold"]': {
    color: "#5a481d",
  },
  "@media (forced-colors: active)": {
    '.rf-badge, .rf-badge[data-tone="gold"]': {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
      border: "1px solid CanvasText",
    },
  },
});
