import { defineGlobalStyles } from "@pandacss/dev";

export const numberFieldStyles = defineGlobalStyles({
  ".rf-number-field": {
    display: "grid",
    minWidth: "0",
    gap: "3px",
  },
  '.rf-number-field[data-label-layout="inline"]': {
    gridTemplateColumns: "minmax(0, 1fr) auto",
    alignItems: "center",
    gap: "12px",
  },
  ".rf-number-controls": {
    display: "flex",
    alignItems: "center",
    minWidth: "0",
  },
  ".rf-number-box": {
    display: "flex",
    alignItems: "center",
    flex: "1",
    minWidth: "0",
    minHeight: "44px",
    border: "1px solid var(--rf-control-edge)",
    borderRadius: "2px",
    background: "var(--rf-paper)",
    color: "var(--rf-paper-ink)",
  },
  '.rf-number-field[data-label-layout="inline"] .rf-number-box': {
    width: "75px",
  },
  ".rf-number-field[data-stepper] .rf-number-box": {
    flex: "0 0 48px",
  },
  ".rf-number-box:has(input[aria-invalid=true])": {
    borderColor: "var(--rf-error)",
  },
  ".rf-number-box input": {
    width: "100%",
    minWidth: "0",
    minHeight: "42px",
    padding: "4px 7px",
    border: "0",
    background: "transparent",
    color: "inherit",
    fontVariantNumeric: "tabular-nums",
  },
  ".rf-number-field[data-stepper] input": {
    textAlign: "center",
  },
  ".rf-number-box > span": {
    flexShrink: "0",
    paddingInlineEnd: "8px",
    color: "var(--rf-paper-muted)",
    fontSize: "0.79rem",
    whiteSpace: "nowrap",
  },
  ".rf-number-box:focus-within": {
    outline: "3px solid var(--rf-focus)",
    outlineOffset: "2px",
  },
  ".rf-theme .rf-number-box input:focus-visible": {
    outline: "none",
  },
});
