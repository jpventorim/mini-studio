import { defineGlobalStyles } from "@pandacss/dev";

export const noticeStyles = defineGlobalStyles({
  ".rf-notice": {
    display: "flex",
    alignItems: "center",
    gap: "12px",
    minWidth: "0",
    fontSize: "0.875rem",
  },
  '.rf-notice[data-variant="floating"]': {
    padding: "6px 12px",
    border: "1px solid var(--rf-paper-edge)",
    borderRadius: "4px",
    background: "var(--rf-paper)",
    color: "var(--rf-paper-muted)",
  },
  '.rf-notice[data-notice-tone="error"]': { color: "var(--rf-error)" },
  '.rf-notice[data-notice-tone="error"][data-variant="floating"]': {
    color: "var(--rf-paper-error)",
  },
  ".rf-notice-text": { overflowWrap: "anywhere", whiteSpace: "pre-line" },
});
