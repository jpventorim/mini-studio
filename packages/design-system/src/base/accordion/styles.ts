import { defineGlobalStyles } from "@pandacss/dev";

export const accordionStyles = defineGlobalStyles({
  ".rf-accordion": {
    "--rf-ink": "var(--rf-paper-ink)",
    "--rf-muted": "var(--rf-paper-muted)",
    "--rf-focus": "var(--rf-paper-focus)",
    "--rf-error": "var(--rf-paper-error)",
    display: "grid",
    minWidth: "0",
    border: "1px solid var(--rf-paper-edge)",
    background: "var(--rf-paper)",
    color: "var(--rf-paper-ink)",
  },
  ".rf-accordion-item": {
    minWidth: "0",
    borderBottom: "1px solid var(--rf-paper-edge)",
  },
  ".rf-accordion-item:last-child": {
    borderBottom: "0",
  },
  ".rf-accordion-header": {
    margin: "0",
    font: "inherit",
  },
  ".rf-accordion-trigger": {
    display: "flex",
    gap: "16px",
    width: "100%",
    minHeight: "52px",
    alignItems: "center",
    justifyContent: "space-between",
    padding: "12px 20px",
    background: "transparent",
    border: "0",
    color: "var(--rf-paper-ink)",
    fontSize: "15px",
    fontWeight: "650",
    lineHeight: "1.5",
    textAlign: "start",
    cursor: "pointer",
  },
  ".rf-accordion-trigger:hover:not([data-disabled])": {
    background: "var(--rf-paper-light)",
  },
  ".rf-accordion-trigger > span": {
    minWidth: "0",
    overflowWrap: "anywhere",
  },
  ".rf-accordion-indicator": {
    display: "grid",
    flex: "0 0 24px",
    width: "24px",
    height: "24px",
    placeItems: "center",
    color: "var(--rf-paper-muted)",
  },
  ".rf-accordion-indicator svg": {
    transition: "transform 160ms",
  },
  ".rf-accordion-trigger[data-panel-open] .rf-accordion-indicator svg": {
    transform: "rotate(180deg)",
  },
  ".rf-accordion-trigger:focus-visible": {
    outline: "3px solid var(--rf-paper-focus)",
    outlineOffset: "-4px",
  },
  ".rf-accordion-trigger[data-disabled]": {
    cursor: "not-allowed",
    color: "var(--rf-paper-muted)",
    textDecoration: "line-through",
  },
  ".rf-accordion-panel": {
    height: "var(--accordion-panel-height)",
    overflow: "hidden",
    transition: "height 180ms var(--rf-ease)",
  },
  ".rf-accordion-panel:is([data-starting-style], [data-ending-style])": {
    height: "0",
  },
  ".rf-accordion-panel > div": {
    padding: "0 20px 20px",
    color: "var(--rf-paper-ink)",
    lineHeight: "1.6",
    overflowWrap: "anywhere",
  },
  ".rf-accordion-panel > div > :first-child": {
    marginTop: "0",
  },
  ".rf-accordion-panel > div > :last-child": {
    marginBottom: "0",
  },
  "@media (prefers-reduced-motion: reduce)": {
    ".rf-accordion-panel, .rf-accordion-indicator svg": {
      transition: "none",
    },
  },
  "@media (forced-colors: active)": {
    ".rf-accordion, .rf-accordion-item, .rf-accordion-trigger, .rf-accordion-panel > div":
      {
        background: "Canvas",
        color: "CanvasText",
        borderColor: "CanvasText",
        boxShadow: "none",
      },
    ".rf-accordion-indicator": {
      color: "CanvasText",
    },
    ".rf-accordion-trigger:focus-visible": {
      outlineColor: "Highlight",
    },
    ".rf-accordion-trigger[data-disabled]": {
      color: "GrayText",
    },
  },
});
