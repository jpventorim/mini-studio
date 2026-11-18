import { expect } from "@rstest/core";
import { componentTests } from "../../../tests/browser";

componentTests("base-dimensionfields", {
  "stacks the fields and centers the link beside them": async (page) => {
    const layout = await page
      .locator(".rf-dimension-fields")
      .evaluate((element) => {
        const [width, height, link] = [...element.children].map((child) =>
          child.getBoundingClientRect(),
        );
        return {
          sameWidth: width.width === height.width && width.x === height.x,
          below: height.top > width.bottom,
          beside: link.left > width.right,
          centered:
            Math.abs(
              link.top + link.height / 2 - (width.top + height.bottom) / 2,
            ) < 1,
        };
      });
    expect(layout).toEqual({
      sameWidth: true,
      below: true,
      beside: true,
      centered: true,
    });
  },
  "toggles linked sizing by keyboard and preserves independent dimensions when unlinked":
    async (page) => {
      const width = page.getByRole("textbox", { name: "Width", exact: true });
      const height = page.getByRole("textbox", { name: "Height", exact: true });
      const lock = page.getByRole("button", { name: "Lock aspect ratio" });
      expect(await lock.getAttribute("aria-pressed")).toBe("true");
      await width.fill("30");
      expect(await height.inputValue()).toBe("40");
      await lock.press("Space");
      expect(await lock.getAttribute("aria-pressed")).toBe("false");
      await height.fill("50");
      expect(await width.inputValue()).toBe("30");
      await lock.press("Enter");
      expect(await height.inputValue()).toBe("40");
    },
});
