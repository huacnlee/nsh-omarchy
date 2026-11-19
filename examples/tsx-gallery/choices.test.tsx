// choices: real interactions through the gallery.
import { describe, expect, test } from "nsh/test";
import { gallery, tabUntil, tabTo } from "./testing";

describe("choices", () => {
  test("checkboxes check, an indeterminate one becomes checked", async () => {
    const app = await gallery("Choices");
    await app.press("role:checkbox[Sync on metered networks]");
    await app.waitFor("text:Notifications on · sync on");
    await app.press("role:checkbox[Send notifications]");
    await app.waitFor("text:Notifications off · sync on");
    await app.press("role:checkbox[Select all]");
    await app.waitFor("text:All: checked");
    expect(app.enabled("role:checkbox[Managed by policy]")).toBe(false);
  });

  test("an uncontrolled checkbox keeps its value", async () => {
    const app = await gallery("Choices");
    await app.press("role:checkbox[Keeps its own value]");
    await app.waitFor("text:Kept: on");
    // It drew its own value: the next press asks for the opposite.
    await app.press("role:checkbox[Keeps its own value]");
    await app.waitFor("text:Kept: off");
  });

  test("switches switch", async () => {
    const app = await gallery("Choices");
    await app.press("role:switch[Bluetooth]");
    await app.waitFor("text:Wi-Fi on · Bluetooth on");
    await app.press("role:switch[Wi-Fi]");
    await app.waitFor("text:Wi-Fi off · Bluetooth on");
    expect(app.enabled("role:switch[Airplane mode]")).toBe(false);
  });

  test("radios check by press and by arrows", async () => {
    const app = await gallery("Choices");
    await app.press("role:radio[Team]");
    await app.waitFor("text:Plan: team");
    await tabUntil(app, "up", () => app.shows("text:Plan: pro"));
    expect(app.enabled("role:radio[Enterprise]")).toBe(false);
  });

  test("a slider steps with the keys", async () => {
    const app = await gallery("Choices");
    await tabTo(app, "label:Volume");
    await app.keyboard.press("right");
    await app.waitFor("text:41");
    await app.keyboard.press("end");
    await app.waitFor("text:100");
    await app.keyboard.press("home");
    await app.waitFor("text:0");
  });

  test("progress shows its value", async () => {
    const app = await gallery("Choices");
    expect(app.shows("text:64%")).toBe(true);
    expect(app.shows("text:92%")).toBe(true);
    await tabTo(app, "label:Download progress");
    await app.keyboard.press("end");
    await app.waitFor("text:100%");
  });
});
