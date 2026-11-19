// The gallery's shell: its page list changes the page.
import { describe, expect, test } from "nsh/test";
import { gallery } from "./testing";

describe("shell", () => {
  test("a press changes the page", async () => {
    const app = await gallery("Actions");
    await app.press("id:page-Data");
    await app.waitFor("text:Data");
    expect(app.shows("id:page")).toBe(true);
  });
});
