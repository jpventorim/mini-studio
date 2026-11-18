import { defineGlobalStyles } from "@pandacss/dev";

export const dimensionFieldsStyles = defineGlobalStyles({
  ".rf-dimension-fields": {
    display: "grid",
    gridTemplateColumns: "minmax(0, 1fr) 44px",
    gap: "8px",
  },
  ".rf-dimension-fields > .rf-number-field": {
    gridColumn: "1",
  },
  ".rf-dimension-link": {
    gridColumn: "2",
    gridRow: "1 / span 2",
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
