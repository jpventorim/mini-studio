import { expect } from "@rstest/core";
import { componentTests } from "../../../tests/browser";

componentTests("base-previewcontrols", {
  "pages forward, adjusts zoom, and fits again": async (page) => {
    expect(
      await page.getByRole("button", { name: "Previous page" }).isDisabled(),
    ).toBe(true);
    await page.getByRole("button", { name: "Next page" }).click();
    await page.getByText("Page 2 of 3", { exact: true }).waitFor();
    await page.getByRole("button", { name: "Zoom in" }).click();
    await page.getByText("125%", { exact: true }).waitFor();
    await page.getByRole("button", { name: "Fit page" }).click();
    await page.getByText("100%", { exact: true }).waitFor();
  },
});
