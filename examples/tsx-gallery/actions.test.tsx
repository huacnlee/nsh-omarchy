// actions: real interactions through the gallery.
import { describe, expect, test } from "nsh/test";
import { gallery, tabTo } from "./testing";

describe("actions", () => {
  test("buttons press, disabled ones do not", async () => {
    const app = await gallery("Actions");
    await app.press("role:button[Default]");
    await app.press("role:button[Primary]");
    await app.press("role:button[Delete]");
    await app.press("label:Add");
    await app.waitFor("text:Pressed 4 times");
    expect(app.enabled("role:button[Disabled]")).toBe(false);
    expect(app.enabled("label:Next")).toBe(false);
  });

  test("a toggle and a toggle group switch", async () => {
    const app = await gallery("Actions");
    await app.press("role:button[Bold]");
    await app.waitFor("text:Bold on · marks: italic");
    await app.press("role:button[U]");
    await app.waitFor("text:Bold on · marks: italic, underline");
    await app.press("role:button[I]");
    await app.waitFor("text:Bold on · marks: underline");
    expect(app.enabled("role:button[Locked]")).toBe(false);
  });

  test("the toggle group is one stop, the arrows move inside it", async () => {
    const app = await gallery("Actions");
    // From the Favorite toggle (Locked is disabled), the group is the next
    // stop, entered at its pressed toggle (I).
    await tabTo(app, "role:button[Favorite]");
    await app.keyboard.press("tab");
    await app.keyboard.press("right");
    await app.keyboard.press("space");
    await app.waitFor("text:Bold off · marks: italic, underline");
  });

  test("a button group passes presses and disables its buttons", async () => {
    const app = await gallery("Actions");
    await app.press("role:button[Float]");
    await app.waitFor("text:Layout: float · zoom 100%");
    await app.press("label:Zoom in");
    await app.waitFor("text:Layout: float · zoom 110%");
    expect(app.enabled("role:button[Paste]")).toBe(false);
  });

  test("an app link reports its press", async () => {
    const app = await gallery("Actions");
    await app.press("role:link[Release notes]");
    await app.waitFor("text:Opened: /releases");
  });
});
