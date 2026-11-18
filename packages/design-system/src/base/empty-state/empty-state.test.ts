import { componentTests } from "../../../tests/browser";

componentTests(
  "base-emptystate",
  {
    "opens the drop target action by keyboard": async (page) => {
      await page
        .getByRole("button", { name: /Drop images here/ })
        .press("Enter");
      await page.getByText("File picker requested").waitFor();
    },
  },
  "drop-target-button",
);
