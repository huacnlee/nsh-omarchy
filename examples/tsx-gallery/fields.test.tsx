// fields: real interactions through the gallery.
import { describe, expect, test } from "nsh/test";
import { gallery } from "./testing";

describe("fields", () => {
  test("an input types and submits", async () => {
    const app = await gallery("Fields");
    await app.type("label:Hostname", "arch");
    expect(app.value("label:Hostname")).toBe("arch");
    await app.submit("label:Hostname");
    await app.waitFor("text:Submitted: arch");
  });

  test("a textarea takes several lines", async () => {
    const app = await gallery("Fields");
    await app.type("label:Notes", "one\ntwo");
    expect(app.value("label:Notes")).toBe("one\ntwo");
  });

  test("a number input steps and clamps", async () => {
    const app = await gallery("Fields");
    await app.type("label:Window gaps", "");
    await app.keyboard.press("up");
    await app.waitFor("text:Gaps: 11");
    await app.clear("label:Window gaps");
    await app.type("label:Window gaps", "99");
    await app.type("label:Hostname", "");
    await app.waitFor("text:Gaps: 40");
  });

  test("a select opens, chooses, closes on Escape", async () => {
    const app = await gallery("Fields");
    await app.press("label:Theme");
    await app.waitFor("role:option[Gruvbox]");
    await app.press("role:option[Gruvbox]");
    await app.waitFor("text:Theme: gruvbox");
    await app.waitFor("role:option[Catppuccin]", { state: "gone" });
    await app.press("label:Theme");
    await app.waitFor("role:option[Catppuccin]");
    await app.keyboard.press("escape");
    await app.waitFor("role:option[Catppuccin]", { state: "gone" });
    expect(app.shows("text:Theme: gruvbox")).toBe(true);
  });

  test("a combobox filters and adds", async () => {
    const app = await gallery("Fields");
    await app.type("id:apps-input", "spo");
    await app.waitFor("role:option[Spotify]");
    expect(app.shows("role:option[Chromium]")).toBe(false);
    await app.press("role:option[Spotify]");
    await app.waitFor("text:Pinned: neovim, spotify");
  });
});
