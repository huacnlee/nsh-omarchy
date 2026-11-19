// Shared helpers of the gallery tests.
import { expect, open } from "nsh/test";
import type { App } from "nsh/test";
/** The menu's group label: shown while the menu is open. */
export const NEW_KEY = "text:Document";

/** Opens the gallery on the page `page` (its sidebar label). */
export async function gallery(page: string) {
  const app = await open("App.tsx", { width: 1200, height: 900, platform: "linux", clock: {} });
  if (page !== "Actions") await app.press(`id:page-${page}`);
  await app.waitFor(`text:${page}`);
  return app;
}

/**
 * Moves the keyboard focus from stop to stop (Tab), pressing `key` at each,
 * until `done`: a press gives no keyboard focus on a desktop, Tab does.
 */
export async function tabUntil(app: App, key: string, done: () => boolean) {
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

