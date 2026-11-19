// display and charts: what each page shows, and the ones that respond.
import { describe, expect, test } from "nsh/test";
import { gallery } from "./testing";

describe("display", () => {
  const shows: [string, string[]][] = [
    ["typography", ["text:Heading", "text:Caption for hints and metadata.", "text:omarchy-theme-set tokyo-night"]],
    ["card", ["text:Updates", "text:3 packages can be upgraded.", "role:button[Update]"]],
    ["panel", ["text:Workspace", "text:Outlined panel"]],
    ["badge", ["text:ONLINE", "text:OUTLINE"]],
    ["keycap", ["text:esc", "text:super", "text:shift", "label:super shift enter"]],
    ["separator", ["text:Above the line", "text:Right"]],
    ["avatar", ["text:GH"]],
    ["icon", ["label:Encrypted"]],
    ["empty-state", ["text:No windows", "role:button[Open launcher]"]],
  ];
  for (const [page, selectors] of shows) {
    test(`${page} shows its pieces`, async () => {
      const app = await gallery(page);
      for (const selector of selectors) expect(app.shows(selector)).toBe(true);
    });
  }

  test("a shortcut label follows the platform", async () => {
    const app = await gallery("keycap");
    // Omarchy.shortcutLabel runs outside the engine: it goes by the last
    // platform a component reported, else the one the app was built for.
    expect(app.shows("text:Save is ctrl-s here") || app.shows("text:Save is cmd-s here")).toBe(true);
  });

  test("markdown reports a pressed link", async () => {
    const app = await gallery("markdown");
    expect(app.shows("text:Release notes")).toBe(true);
    expect(app.shows("text:Press a link")).toBe(true);
    await app.press("text:links");
    await app.waitFor("text:Link pressed: https://omarchy.org");
  });
});

describe("charts", () => {
  test("a line chart names its series", async () => {
    const app = await gallery("line-chart");
    expect(app.shows("label:Visitors, January to June")).toBe(true);
    expect(app.shows("text:Mobile")).toBe(true);
  });

  test("a bar chart summarizes its points", async () => {
    const app = await gallery("bar-chart");
    expect(app.shows("label:Bar chart of Desktop and Mobile, 6 points, from 73 to 305")).toBe(true);
  });

  test("an area chart draws", async () => {
    const app = await gallery("area-chart");
    expect(app.shows("text:Desktop")).toBe(true);
  });

  test("a pie chart shows its shares", async () => {
    const app = await gallery("pie-chart");
    expect(app.shows("label:Visitors by browser")).toBe(true);
    expect(app.shows("text:Chrome")).toBe(true);
    expect(app.shows("text:30%")).toBe(true);
  });
});
