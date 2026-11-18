import { expect } from "@rstest/core";
import { componentTests } from "../../../tests/browser";

componentTests("base-numberfield", {
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
});
