import { defineGlobalStyles } from "@pandacss/dev";

export const selectableRowStyles = defineGlobalStyles({
  ".rf-selectable-row": {
    display: "grid",
    gridTemplateColumns: "minmax(0, 1fr) auto 15px",
    alignItems: "center",
    gap: "8px",
    width: "100%",
    minHeight: "52px",
    padding: "6px 8px",
    background: "transparent",
    border: "1px solid transparent",
    borderRadius: "2px",
    color: "var(--rf-ink)",
    textAlign: "start",
    cursor: "pointer",
  },
  ".rf-selectable-row[data-leading]": {
    gridTemplateColumns: "36px minmax(0, 1fr) auto 15px",
  },
  ".rf-selectable-row:hover:not(:disabled)": {
    background: "var(--rf-paper-light)",
    color: "var(--rf-paper-ink)",
  },
  '.rf-selectable-row[aria-pressed="true"]': {
    background: "color-mix(in srgb, var(--rf-paper-focus) 8%, var(--rf-paper))",
    borderColor:
      "color-mix(in srgb, var(--rf-paper-focus) 38%, var(--rf-paper))",
    color: "var(--rf-paper-ink)",
  },
  ".rf-selectable-row:disabled": {
    color: "var(--rf-muted)",
    cursor: "not-allowed",
  },
  ".rf-theme .rf-selectable-row:focus-visible": {
    outlineOffset: "-3px",
  },
  ".rf-selectable-row-leading": {
    display: "grid",
    placeItems: "center",
    height: "36px",
    border: "1px solid var(--rf-paper-edge)",
    background: "var(--rf-paper)",
    color: "var(--rf-paper-ink)",
  },
  ".rf-selectable-row-leading img": {
    display: "block",
    width: "100%",
    height: "100%",
    minHeight: "0",
    objectFit: "contain",
  },
  ".rf-selectable-row-label": {
    overflow: "hidden",
    textOverflow: "ellipsis",
    whiteSpace: "nowrap",
    fontSize: "0.875rem",
  },
  ".rf-selectable-row-detail": {
    color: "var(--rf-muted)",
    fontSize: "0.8125rem",
    fontVariantNumeric: "tabular-nums",
  },
  '.rf-selectable-row[aria-pressed="true"] .rf-selectable-row-detail, .rf-selectable-row:hover:not(:disabled) .rf-selectable-row-detail':
    {
      color: "var(--rf-paper-muted)",
    },
  ".rf-selectable-row-chevron": {
    gridColumn: "-2 / -1",
  },
  "@media (forced-colors: active)": {
    '.rf-selectable-row[aria-pressed="true"]': {
      borderColor: "Highlight",
    },
  },
});
