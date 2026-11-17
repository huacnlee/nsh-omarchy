// nsh-omarchy's Rust exports, called from TSX in the headless runner `nsh
// test` links with the crate.
import { expect, open, test } from "nsh/test";

test("the Omarchy module applies palettes and tells TSX", async () => {
  const app = await open("./App.tsx");
  // `followSystem` applied a theme (the system's, or Tokyo Night) and the
  // listener heard it.
  await app.waitUntil(() => app.texts().some((t) => t.startsWith("Theme: ") && t !== "Theme: none yet"));
  await app.press("Flexoki Light");
  await app.waitFor("Theme: Flexoki Light (light)");
  await app.press("Tokyo Night");
  await app.waitFor("Theme: Tokyo Night (dark)");
});

test("the chart, the icon and Markdown render from TSX props", async () => {
  const app = await open("./App.tsx");
  // The chart's summary and the labeled icon are what screen readers say.
  await app.waitFor("label:Builds this week");
  expect(app.shows("label:Theme")).toBe(true);
  expect(app.shows("Mon")).toBe(true);
  expect(app.shows("Release notes")).toBe(true);
});
