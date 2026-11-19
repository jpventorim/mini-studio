import { expect } from "@rstest/core";
import { componentTests } from "../../../tests/browser";

componentTests("base-numberfield", {
  "moves through allowed sizes and rejects intermediate values": async (
    page,
  ) => {
    const size = page.getByRole("textbox", { name: "Size", exact: true });
    await size.fill("0.5");
    for (const value of [0.75, 1, 2, 3, 4, 5, 5]) {
      await size.press("ArrowUp");
      expect(await size.inputValue()).toBe(String(value));
    }
    for (const value of [4, 3, 2, 1, 0.75, 0.5, 0.5]) {
      await size.press("ArrowDown");
      expect(await size.inputValue()).toBe(String(value));
    }
    for (const value of ["0.25", "1.5", "6"]) {
      await size.fill(value);
      expect(await size.getAttribute("aria-invalid")).toBe("true");
      await size.press("Enter");
      expect(await size.inputValue()).toBe("0.5");
    }
  },
  "steps a bounded quantity and preserves only valid input": async (page) => {
    const quantity = page.getByRole("textbox", {
      name: "Quantity",
      exact: true,
    });
    await page.getByRole("button", { name: "Decrease quantity" }).click();
    expect(await quantity.inputValue()).toBe("1");
    expect(
      await page
        .getByRole("button", { name: "Decrease quantity" })
        .isDisabled(),
    ).toBe(true);
    await page.getByRole("button", { name: "Increase quantity" }).click();
    await quantity.fill("10");
    expect(await quantity.getAttribute("aria-invalid")).toBe("true");
    await quantity.press("Enter");
    expect(await quantity.inputValue()).toBe("2");
    const width = page.getByRole("textbox", { name: "Width" });
    await width.fill("42,5");
    await width.press("Enter");
    expect(await width.inputValue()).toBe("42.5");
  },
  "steps with arrow keys and respects the field limits": async (page) => {
    const quantity = page.getByRole("textbox", {
      name: "Quantity",
      exact: true,
    });
    await quantity.press("ArrowUp");
    expect(await quantity.inputValue()).toBe("3");
    await quantity.fill("5");
    await quantity.press("ArrowUp");
    expect(await quantity.inputValue()).toBe("5");
    const width = page.getByRole("textbox", { name: "Width" });
    await width.fill("2");
    await width.press("ArrowUp");
    expect(await width.inputValue()).toBe("2.5");
    await width.press("ArrowDown");
    expect(await width.inputValue()).toBe("2");
    await width.fill("0.1");
    await width.press("ArrowDown");
    expect(await width.inputValue()).toBe("0.1");
    await width.fill("199.75");
    await width.press("ArrowUp");
    expect(await width.inputValue()).toBe("200");
    expect(
      await page.getByRole("button", { name: "Increase width" }).count(),
    ).toBe(0);
  },
});
