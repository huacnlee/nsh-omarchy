// The gallery's shell: a page per component, grouped by family in the sidebar.
import { describe, expect, test } from "nsh/test";
import { gallery } from "./testing";

describe("shell", () => {
  test("the sidebar groups the pages by family", async () => {
    const app = await gallery("button");
    for (const family of ["Actions", "Choices", "Fields", "Navigation", "Overlays", "Data", "Display", "Charts"]) {
      expect(app.shows(`text:${family}`)).toBe(true);
    }
    expect(app.shows("role:tab[IconButton]")).toBe(true);
    expect(app.shows("role:tab[PieChart]")).toBe(true);
  });

  test("a press changes the page, showing only that component", async () => {
    const app = await gallery("button");
    expect(app.shows("role:button[Primary]")).toBe(true);
    await app.press("id:page-switch");
    await app.waitFor("role:switch[Wi-Fi]");
    expect(app.shows("role:button[Primary]")).toBe(false);
    expect(app.shows("role:checkbox[Send notifications]")).toBe(false);
  });
});
