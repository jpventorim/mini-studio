import { defineGlobalStyles } from "@pandacss/dev";

export const avatarStyles = defineGlobalStyles({
  ".rf-avatar": {
    "--rf-ink": "var(--rf-paper-ink)",
    "--rf-muted": "var(--rf-paper-muted)",
    "--rf-focus": "var(--rf-paper-focus)",
    "--rf-error": "var(--rf-paper-error)",
    display: "inline-flex",
    verticalAlign: "middle",
    flex: "0 0 auto",
    width: "48px",
    height: "48px",
    alignItems: "center",
    justifyContent: "center",
    borderRadius: "50%",
    overflow: "hidden",
    color: "#fffef8",
    background: "var(--rf-sapphire)",
  },
  '.rf-avatar[data-size="sm"]': {
    width: "32px",
    height: "32px",
  },
  '.rf-avatar[data-size="lg"]': {
    width: "64px",
    height: "64px",
  },
  '.rf-avatar[data-size="sm"] .rf-avatar-fallback': {
    fontSize: "12px",
  },
  '.rf-avatar[data-size="lg"] .rf-avatar-fallback': {
    fontSize: "23px",
  },
  ".rf-avatar-image": {
    width: "100%",
    height: "100%",
    objectFit: "cover",
    display: "block",
  },
  ".rf-avatar-fallback": {
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    width: "100%",
    height: "100%",
    fontFamily: "var(--rf-font-body)",
    fontSize: "17px",
    fontWeight: "600",
    lineHeight: "1",
  },
  "@media (forced-colors: active)": {
    ".rf-avatar": {
      background: "Canvas",
      color: "CanvasText",
      borderColor: "CanvasText",
      boxShadow: "none",
      border: "1px solid CanvasText",
    },
  },
});
