// navigation: real interactions through the gallery.
import { describe, expect, test } from "nsh/test";
import { NEW_KEY, gallery, tabTo } from "./testing";

describe("navigation", () => {
  test("tabs select by press and arrows", async () => {
    const app = await gallery("Navigation");
    await app.press("role:tab[Keybindings]");
    await app.waitFor("text:Panel: keys");
    await tabTo(app, "role:tab[Keybindings]");
    await app.keyboard.press("right");
    await app.waitFor("text:Panel: display");
    await app.press("role:tab[Grid]");
    await app.waitFor("text:View: grid");
  });

  test("a dropdown menu opens, chooses, toggles, closes", async () => {
    const app = await gallery("Navigation");
    await app.press("role:button[File]");
    await app.waitFor(NEW_KEY);
    await app.press("text:Word wrap");
    await app.waitFor("text:Chose: nothing · wrap off");
    expect(app.shows(NEW_KEY)).toBe(true);
    await app.press("text:Open…");
    await app.waitFor("text:Chose: Open · wrap off");
    await app.waitFor(NEW_KEY, { state: "gone" });
    await app.press("role:button[File]");
    await app.waitFor(NEW_KEY);
    await app.keyboard.press("escape");
    await app.waitFor(NEW_KEY, { state: "gone" });
  });

  test("a context menu opens on a right click", async () => {
    const app = await gallery("Navigation");
    await app.rightClick("text:Right-click here");
    await app.waitFor("text:Paste");
    await app.press("text:Paste");
    await app.waitFor("text:Chose: Paste · wrap on");
  });

  test("an accordion opens and closes sections", async () => {
    const app = await gallery("Navigation");
    expect(app.shows("text:An opinionated Arch and Hyprland setup.")).toBe(true);
    await app.press("text:Can I change the theme?");
    await app.waitFor("text:Open: what, themes");
    await app.press("text:What is Omarchy?");
    await app.waitFor("text:Open: themes");
    expect(app.enabled("text:Locked section")).toBe(false);
  });

  test("pagination moves between pages", async () => {
    const app = await gallery("Navigation");
    expect(app.enabled("label:Previous page")).toBe(false);
    await app.press("label:Next page");
    await app.waitFor("text:Page 2 of 12");
    await app.press("text:12");
    await app.waitFor("text:Page 12 of 12");
    expect(app.enabled("label:Next page")).toBe(false);
  });

  test("a link opens its URL", async () => {
    const app = await gallery("Navigation");
    await app.press("text:omarchy.org");
    expect(app.openedUrls()).toEqual(["https://omarchy.org"]);
  });
});
