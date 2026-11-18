// Paper surfaces and the shared 24px graph ruling.
export const materialTokens = {
  "--rf-grid-step": "24px",
  "--rf-grid-lines":
    "linear-gradient(var(--rf-grid) 1px, transparent 1px), linear-gradient(90deg, var(--rf-grid) 1px, transparent 1px)",
  "--rf-canvas-texture":
    "linear-gradient(var(--rf-grid) 1px, transparent 1px) 0 0 / var(--rf-grid-step) var(--rf-grid-step), linear-gradient(90deg, var(--rf-grid) 1px, transparent 1px) 0 0 / var(--rf-grid-step) var(--rf-grid-step), var(--rf-canvas)",
  "--rf-paper-texture": "var(--rf-paper)",
};
