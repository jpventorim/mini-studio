import { defineGlobalStyles } from "@pandacss/dev";

export const tooltipStyles = defineGlobalStyles({
  ".rf-tooltip": {
    position: "relative",
    maxWidth: "min(280px, calc(100vw - 32px), var(--available-width, 280px))",
    padding: "8px 12px",
    background: "#282d32",
    color: "#fffef8",
    border: "1px solid #282d32",
    borderRadius: "2px",
    fontSize: "0.875rem",
    lineHeight: "1.5",
    fontWeight: "500",
    whiteSpace: "normal",
    overflowWrap: "anywhere",
    boxShadow: "var(--rf-shadow-contact)",
    transition: "opacity 140ms ease-out",
  },
  ".rf-tooltip:is([data-starting-style], [data-ending-style])": {
    opacity: "0",
  },
  "@media (prefers-reduced-motion: reduce)": {
    ".rf-tooltip": {
      transition: "none",
    },
  },
  "@media (forced-colors: active)": {
    ".rf-tooltip": {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
    },
  },
});
