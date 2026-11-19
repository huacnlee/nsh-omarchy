// display: real interactions through the gallery.
import { describe, expect, test } from "nsh/test";
import { gallery } from "./testing";

describe("display", () => {
  test("shows every piece", async () => {
    const app = await gallery("Display");
    for (const text of [
      "Heading",
      "Updates",
      "Workspace",
      "Outlined panel",
      "ONLINE",
      "OUTLINE",
      "esc",
      "super",
      "shift",
      "No windows",
      "Open launcher",
    ]) {
      expect(app.shows(`text:${text}`)).toBe(true);
    }
    expect(app.shows("text:GH")).toBe(true);
    // Omarchy.shortcutLabel runs outside the engine: it goes by the last
    // platform a component reported, else the one the app was built for.
    expect(app.shows("text:Save is ctrl-s here") || app.shows("text:Save is cmd-s here")).toBe(true);
    expect(app.shows("label:Encrypted")).toBe(true);
    expect(app.shows("label:super shift enter")).toBe(true);
  });

  test("markdown reports a pressed link", async () => {
    const app = await gallery("Display");
    expect(app.shows("text:Release notes")).toBe(true);
    expect(app.shows("text:Press a link")).toBe(true);
    await app.press("text:links");
    await app.waitFor("text:Link pressed: https://omarchy.org");
  });

  test("charts draw their summaries and legends", async () => {
    const app = await gallery("Display");
    expect(app.shows("label:Visitors, January to June")).toBe(true);
    expect(app.shows("label:Visitors by browser")).toBe(true);
    expect(app.shows("label:Bar chart of Desktop and Mobile, 6 points, from 73 to 305")).toBe(true);
    expect(app.shows("text:Mobile")).toBe(true);
    expect(app.shows("text:Chrome")).toBe(true);
    expect(app.shows("text:30%")).toBe(true);
  });
});
