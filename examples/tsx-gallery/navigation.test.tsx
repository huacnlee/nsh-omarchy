// navigation: real interactions through the gallery.
import { describe, expect, test } from "nsh/test";
import { NEW_KEY, gallery, tabTo, tabUntil } from "./testing";

describe("navigation", () => {
  test("tabs select by press and arrows", async () => {
    const app = await gallery("Navigation");
    await app.press("role:tab[Keybindings]");
    await app.waitFor("text:Panel: keys");
    await tabTo(app, "role:tab[Keybindings]");
    await app.keyboard.press("right");
    await app.waitFor("text:Panel: display");
    expect(app.enabled("role:tab[Beta]")).toBe(false);
    await app.press("role:tab[Grid]");
    await app.waitFor("text:View: grid");
  });

  test("the sidebar selects by press and arrows", async () => {
    const app = await gallery("Navigation");
    await app.press("role:tab[Starred]");
    await app.waitFor("text:Mailbox: starred");
    // Tab to the sidebar (one stop, at the current item), Down there.
    await tabUntil(app, "down", () => app.shows("text:Mailbox: sent"));
    expect(app.shows("text:Labels")).toBe(true);
    expect(app.enabled("role:tab[Archive]")).toBe(false);
  });

  test("a menu opens, chooses, toggles, closes", async () => {
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
    await app.press("role:button[File]");
    await app.press("text:Delete");
    await app.waitFor("text:Chose: Delete · wrap off");
  });

  test("a context menu opens on a right click", async () => {
    const app = await gallery("Navigation");
    await app.rightClick("text:Right-click here");
    await app.waitFor("text:Paste");
    await app.press("text:Paste");
    await app.waitFor("text:Chose: Paste · wrap on");
  });

  test("an accordion opens and closes items", async () => {
    const app = await gallery("Navigation");
    expect(app.shows("text:An opinionated Arch and Hyprland setup.")).toBe(true);
    await app.press("text:Can I change the theme?");
    await app.waitFor("text:Open: what, themes");
    await app.press("text:What is Omarchy?");
    await app.waitFor("text:Open: themes");
    expect(app.enabled("role:button[Locked item]")).toBe(false);
  });

  test("collapsibles open, controlled or not", async () => {
    const app = await gallery("Navigation");
    expect(app.shows("text:Compression: zstd, level 3")).toBe(false);
    await app.press("label:Advanced options");
    await app.waitFor("text:Advanced: open");
    await app.waitFor("text:Compression: zstd, level 3");
    await app.press("label:Release notes");
    await app.waitFor("text:0.1: the first release.");
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
});
