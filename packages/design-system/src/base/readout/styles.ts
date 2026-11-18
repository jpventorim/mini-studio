import { defineGlobalStyles } from "@pandacss/dev";

export const readoutStyles = defineGlobalStyles({
  ".rf-readout": {
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    flexWrap: "wrap",
    gap: "4px 8px",
    padding: "8px 10px",
    background: "var(--rf-paper-light)",
  },
  ".rf-readout > div": { display: "grid" },
  ".rf-readout > div > span": { color: "var(--rf-muted)", fontSize: "0.75rem" },
  ".rf-readout strong": {
    fontSize: "0.875rem",
    fontVariantNumeric: "tabular-nums",
  },
});
