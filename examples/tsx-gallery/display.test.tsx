// display: real interactions through the gallery.
import { describe, expect, test } from "nsh/test";
import { gallery } from "./testing";

describe("display", () => {
  test("shows every piece", async () => {
    const app = await gallery("Display");
    for (const text of ["Heading", "Updates", "Inset panel", "ONLINE", "esc", "super", "No windows", "Open launcher"]) {
      expect(app.shows(`text:${text}`)).toBe(true);
    }
    expect(app.shows("text:GH")).toBe(true);
  });
});
