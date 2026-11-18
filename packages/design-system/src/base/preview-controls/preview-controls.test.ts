import { componentTests } from "../../../tests/browser";

componentTests("base-previewcontrols", {
  "adjusts zoom and fits again": async (page) => {
    await page.getByRole("button", { name: "Zoom in" }).click();
    await page.getByText("125%", { exact: true }).waitFor();
    await page.getByRole("button", { name: "Fit page" }).click();
    await page.getByText("100%", { exact: true }).waitFor();
  },
});
