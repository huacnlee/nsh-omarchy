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

  test("a field shows its error in place of its description", async () => {
    const app = await gallery("Fields");
    expect(app.shows("text:Too short.")).toBe(true);
    expect(app.shows("text:At least 8 characters.")).toBe(false);
    await app.type("label:Password", "!!");
    await app.waitFor("text:At least 8 characters.");
    expect(app.shows("text:Too short.")).toBe(false);
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

  test("an otp input completes", async () => {
    const app = await gallery("Fields");
    await app.type("label:Verification code", "12a345");
    expect(app.shows("text:Verified: nothing")).toBe(true);
    await app.type("label:Verification code", "6");
    await app.waitFor("text:Verified: 123456");
  });

  test("a select opens, chooses, closes on Escape", async () => {
    const app = await gallery("Fields");
    await app.press("role:combobox[Theme]");
    await app.waitFor("role:option[Gruvbox]");
    await app.press("role:option[Gruvbox]");
    await app.waitFor("text:Theme: gruvbox");
    await app.waitFor("role:option[Catppuccin]", { state: "gone" });
    await app.press("role:combobox[Theme]");
    await app.waitFor("role:option[Catppuccin]");
    await app.keyboard.press("escape");
    await app.waitFor("role:option[Catppuccin]", { state: "gone" });
    expect(app.shows("text:Theme: gruvbox")).toBe(true);
  });

  test("a combobox filters and adds", async () => {
    const app = await gallery("Fields");
    await app.type("label:Pinned apps", "spo");
    await app.waitFor("role:option[Spotify]");
    expect(app.shows("role:option[Chromium]")).toBe(false);
    await app.press("role:option[Spotify]");
    await app.waitFor("text:Pinned: neovim, spotify");
  });

  test("a calendar picks a day and a range", async () => {
    const app = await gallery("Fields");
    const today = new Date();
    const iso = (d: Date) =>
      `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
    const month = (n: number) => iso(new Date(today.getFullYear(), today.getMonth(), n));
    await app.press(`label:${month(14)} >> nth=0`);
    await app.waitFor(`text:Day: ${month(14)}`);
    await app.press(`label:${month(5)} >> nth=1`);
    await app.press(`label:${month(9)} >> nth=1`);
    await app.waitFor(`text:Trip: ${month(5)} to ${month(9)}`);
  });

  test("a date picker shows its value, opens and picks", async () => {
    const app = await gallery("Fields");
    expect(app.shows("text:Nov 19, 2026")).toBe(true);
    expect(app.shows("text:Choose dates")).toBe(true);
    await app.press("role:combobox[Due date]");
    // The picker's calendar comes after the page's two.
    await app.waitFor("label:2026-11-23 >> nth=2");
    await app.press("label:2026-11-23 >> nth=2");
    await app.waitFor("text:Due: 2026-11-23");
    await app.waitFor("text:Nov 23, 2026");
    await app.waitFor("label:2026-11-23 >> nth=2", { state: "gone" });
  });

  test("a color picker takes a typed color", async () => {
    const app = await gallery("Fields");
    await app.clear("label:Hex color");
    await app.type("label:Hex color", "#ff0000");
    await app.submit("label:Hex color");
    await app.waitFor("text:Accent: #ff0000ff");
  });
});
