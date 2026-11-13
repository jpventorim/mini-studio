import { defineGlobalStyles } from "@pandacss/dev";

export const tabsStyles = defineGlobalStyles({
  ".rf-tabs": {
    minWidth: "0",
    background: "var(--rf-paper)",
    paddingInline: "20px",
  },
  ".rf-tabs-list": {
    position: "relative",
    display: "flex",
    gap: "24px",
    minWidth: "0",
    overflowX: "auto",
    borderBottom: "1px solid var(--rf-paper-edge)",
    scrollbarWidth: "thin",
  },
  ".rf-tab": {
    flex: "0 0 auto",
    minHeight: "48px",
    minWidth: "44px",
    padding: "12px 2px 10px",
    border: "0",
    borderBottom: "2px solid transparent",
    background: "transparent",
    color: "var(--rf-paper-muted)",
    fontSize: "14px",
    fontWeight: "500",
    lineHeight: "1.5",
    whiteSpace: "nowrap",
    cursor: "pointer",
  },
  ".rf-tab:hover:not([data-disabled], [data-active])": {
    color: "var(--rf-paper-ink)",
  },
  ".rf-tab[data-active]": {
    color: "var(--rf-paper-ink)",
    fontWeight: "650",
  },
  ".rf-tab-indicator": {
    position: "absolute",
    left: "0",
    bottom: "0",
    width: "var(--active-tab-width)",
    height: "2px",
    background: "var(--rf-sapphire)",
    transform: "translateX(var(--active-tab-left))",
    transition:
      "transform var(--rf-motion-control) var(--rf-ease), width var(--rf-motion-control) var(--rf-ease), height var(--rf-motion-control) var(--rf-ease)",
    pointerEvents: "none",
  },
  ".rf-tabs-list .rf-tab:focus-visible": {
    outline: "3px solid var(--rf-paper-focus)",
    outlineOffset: "-4px",
  },
  ".rf-tab:is([data-disabled], :disabled)": {
    color: "var(--rf-paper-muted)",
    cursor: "not-allowed",
    textDecoration: "line-through",
  },
  ".rf-tab-panel": {
    paddingBlock: "20px",
    color: "var(--rf-paper-ink)",
    minWidth: "0",
    overflowWrap: "anywhere",
  },
  ".rf-tab-panel[hidden]": {
    display: "none",
  },
  '.rf-tabs[data-orientation="vertical"]': {
    display: "grid",
    gridTemplateColumns: "auto minmax(0, 1fr)",
    columnGap: "20px",
  },
  '.rf-tabs[data-orientation="vertical"] .rf-tabs-list': {
    flexDirection: "column",
    alignItems: "stretch",
    gap: "0",
    borderBottom: "0",
    borderInlineEnd: "1px solid var(--rf-paper-edge)",
    paddingBlock: "8px",
  },
  '.rf-tabs[data-orientation="vertical"] .rf-tab': {
    textAlign: "start",
    paddingInline: "8px 16px",
    borderBottom: "0",
    borderInlineEnd: "2px solid transparent",
  },
  '.rf-tabs[data-orientation="vertical"] .rf-tab-indicator': {
    left: "auto",
    insetInlineEnd: "0",
    top: "0",
    bottom: "auto",
    width: "2px",
    height: "var(--active-tab-height)",
    transform: "translateY(var(--active-tab-top))",
  },
  "@media (prefers-reduced-motion: reduce)": {
    ".rf-tab-indicator": { transition: "none" },
  },
  "@media (forced-colors: active)": {
    ".rf-tabs, .rf-tab": {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
    },
    ".rf-tabs-list": {
      borderColor: "CanvasText",
    },
    ".rf-tabs .rf-tab[data-active]": {
      textDecoration: "underline",
      textUnderlineOffset: "4px",
    },
    ".rf-tab-indicator": {
      background: "Highlight",
      forcedColorAdjust: "none",
    },
    ".rf-tabs-list .rf-tab:focus-visible": {
      outlineColor: "Highlight",
    },
    ".rf-tab:is([data-disabled], :disabled)": {
      color: "GrayText",
    },
  },
});
