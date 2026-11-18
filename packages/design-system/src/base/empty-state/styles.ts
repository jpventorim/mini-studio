import { defineGlobalStyles } from "@pandacss/dev";

export const emptyStateStyles = defineGlobalStyles({
  ".rf-empty-state": {
    display: "grid",
    justifyItems: "center",
    gap: "4px",
    maxWidth: "100%",
    padding: "12px",
    color: "var(--rf-muted)",
    textAlign: "center",
    fontSize: "0.8125rem",
  },
  ".rf-empty-icon": { display: "flex", marginBottom: "4px" },
  ".rf-empty-title": {
    color: "var(--rf-ink)",
    font: "700 1.05rem / 1.2 var(--rf-font-display)",
  },
  '.rf-empty-state[data-variant="drop"]': {
    padding: "24px 10px",
    border: "1px dashed var(--rf-control-edge)",
    background: "transparent",
  },
  '.rf-empty-state[data-variant="drop"] .rf-empty-title': {
    fontFamily: "inherit",
    fontWeight: "600",
    fontSize: "0.9375rem",
    lineHeight: "1.5",
  },
  ".rf-theme button.rf-empty-state": {
    width: "100%",
    fontSize: "0.8125rem",
    cursor: "pointer",
  },
  ".rf-theme button.rf-empty-state:hover:not(:disabled)": {
    background: "var(--rf-paper-light)",
    color: "var(--rf-paper-muted)",
  },
  ".rf-theme button.rf-empty-state:hover:not(:disabled) .rf-empty-title": {
    color: "var(--rf-paper-ink)",
  },
  ".rf-theme button.rf-empty-state:disabled": { cursor: "wait" },
});
