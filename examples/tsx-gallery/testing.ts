// Shared helpers of the gallery tests.
import { expect, open } from "nsh/test";
import type { App } from "nsh/test";
import { pages } from "./pages";

/** The menu's group label: shown while the menu is open. */
export const NEW_KEY = "text:Document";

/**
 * The gallery's own Tab stops before a page: the Appearance tabs and the
 * sidebar. Arrows there change the theme or the page, so the helpers below
 * pass them before pressing keys.
 */
const CHROME_STOPS = 2;

/** Opens the gallery on the page whose sidebar value is `value`. */
export async function gallery(value: string) {
  const page = pages.find((p) => p.value === value);
  if (!page) throw new Error(`no gallery page ${value}`);
  const app = await open("App.tsx", { width: 1200, height: 900, platform: "linux", clock: {} });
  if (value !== pages[0].value) await app.press(`id:page-${value}`);
  await app.waitFor(`text:${page.title}`);
  return app;
}

/** Tabs to the page's first stop, past the gallery's own. */
export async function enterPage(app: App) {
  for (let stop = 0; stop <= CHROME_STOPS; stop++) await app.keyboard.press("tab");
}

/**
 * Moves the keyboard focus from stop to stop of the page (Tab), pressing
 * `key` at each, until `done`: a press gives no keyboard focus on a desktop,
 * Tab does.
 */
export async function tabUntil(app: App, key: string, done: () => boolean) {
  for (let stop = 0; stop < CHROME_STOPS; stop++) await app.keyboard.press("tab");
  for (let stops = 0; !done(); stops++) {
    expect(stops).toBeLessThan(60);
    await app.keyboard.press("tab");
    await app.keyboard.press(key);
    await app.wait(20);
  }
}

/**
 * Tabs until the keyboard focus is on what `selector` names: its
 * `focus-visible:` fill shows (the library's controls all draw one).
 */
export async function tabTo(app: App, selector: string) {
  const resting = app.viewStyle(selector)?.backgroundColor;
  for (let stops = 0; app.viewStyle(selector)?.backgroundColor === resting; stops++) {
    expect(stops).toBeLessThan(60);
    await app.keyboard.press("tab");
    await app.wait(20);
  }
}
