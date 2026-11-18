import { expect } from "@rstest/core";
import { componentTests } from "../../../tests/browser";

componentTests("base-selectablerow", {
  "selects a row by keyboard and exposes the selection": async (page) => {
    const guardian = page.getByRole("button", { name: /Guardian/ });
    await guardian.press("Space");
    expect(await guardian.getAttribute("aria-pressed")).toBe("true");
    expect(
      await page
        .getByRole("button", { name: /Scout/ })
        .getAttribute("aria-pressed"),
    ).toBe("false");
  },
});
