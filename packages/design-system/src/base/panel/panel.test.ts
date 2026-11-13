import { expect } from "@rstest/core";
import { componentTests } from "../../../tests/browser";

componentTests("base-panel", {
  "renders a plain readable sheet without a frame or grain": async (page) => {
    const surface = await page.locator(".rf-panel").evaluate((element) => {
      const paper = getComputedStyle(element);
      return {
        paper: paper.backgroundColor,
        ink: paper.color,
        grain: paper.backgroundImage,
      };
    });
    expect(surface.paper).not.toBe("rgba(0, 0, 0, 0)");
    expect(surface.paper).not.toBe(surface.ink);
    expect(surface.grain).toBe("none");
  },
});
