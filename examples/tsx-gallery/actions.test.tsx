// actions: real interactions through the gallery.
import { describe, expect, test } from "nsh/test";
import { gallery, tabTo } from "./testing";

describe("actions", () => {
  test("buttons press, disabled ones do not", async () => {
    const app = await gallery("Actions");
    await app.press("role:button[Default]");
    await app.press("role:button[Primary]");
    await app.press("role:button[Delete]");
    await app.waitFor("text:Pressed 3 times");
    expect(app.enabled("role:button[Disabled]")).toBe(false);
    expect(app.shows("text:Pressed 3 times")).toBe(true);
    expect(app.enabled("label:Next")).toBe(false);
  });

  test("a toggle and a toggle group switch", async () => {
    const app = await gallery("Actions");
    await app.press("role:button[Bold]");
    await app.waitFor("text:Bold on · marks: italic");
    await app.press("text:U");
    await app.waitFor("text:Bold on · marks: italic, underline");
    await app.press("text:I");
    await app.waitFor("text:Bold on · marks: underline");
  });

  test("a button group chooses one, skips the disabled segment", async () => {
    const app = await gallery("Actions");
    await app.press("text:Float");
    await app.waitFor("text:Layout: float");
    expect(app.enabled("text:Full")).toBe(false);
    // From the Bold toggle, the toggle group is one stop on (entered at its
    // pressed toggle), the button group the next (at its chosen segment).
    await tabTo(app, "role:button[Bold]");
    await app.keyboard.press("tab");
    await app.keyboard.press("tab");
    await app.keyboard.press("right");
    await app.keyboard.press("space");
    await app.waitFor("text:Layout: stack");
  });
});
