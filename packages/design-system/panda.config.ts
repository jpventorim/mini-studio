import { defineConfig } from "@pandacss/dev";
import { globalCss, keyframes } from "./src/theme";

export default defineConfig({
  preflight: false,
  include: ["./src/**/*.{ts,tsx}", "./stories/**/*.tsx"],
  exclude: ["./src/**/*.test.ts"],
  outdir: "styled-system",
  globalCss,
  theme: {
    extend: {
      tokens: {
        colors: {
          ink: { value: "#282d32" },
          muted: { value: "#58616b" },
          paper: { value: "#fffef8" },
          paperLight: { value: "#f6f4ec" },
          border: { value: "#c6c9c7" },
          gold: { value: "#e7c66b" },
          ruby: { value: "#a13742" },
          emerald: { value: "#2f684f" },
          sapphire: { value: "#285a88" },
          focus: { value: "#285a88" },
        },
        fonts: {
          body: { value: '"Segoe UI", ui-sans-serif, system-ui, sans-serif' },
          display: { value: 'Georgia, "Times New Roman", serif' },
        },
      },
      keyframes,
    },
  },
});
