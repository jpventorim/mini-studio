import { defineGlobalStyles } from "@pandacss/dev";

export const dimensionFieldsStyles = defineGlobalStyles({
  ".rf-dimension-fields": {
    display: "grid",
    gridTemplateColumns: "minmax(0, 1fr) 44px",
    gridTemplateRows: "auto auto 2px auto auto",
    gap: "3px 8px",
  },
  ".rf-dimension-fields > .rf-number-field": {
    gridColumn: "1",
    gridTemplateRows: "subgrid",
    gridRow: "1 / 3",
  },
  ".rf-dimension-fields > .rf-number-field + .rf-number-field": {
    gridRow: "4 / 6",
  },
  ".rf-dimension-link": {
    gridColumn: "2",
    gridRow: "2 / 6",
    alignSelf: "center",
  },
  '.rf-dimension-link.rf-button[aria-pressed="true"]': {
    color: "var(--rf-paper-focus)",
    background: "color-mix(in srgb, var(--rf-paper-focus) 9%, var(--rf-paper))",
  },
  "@media (forced-colors: active)": {
    '.rf-dimension-link.rf-button[aria-pressed="true"]': {
      color: "Highlight",
      borderColor: "Highlight",
    },
  },
});
