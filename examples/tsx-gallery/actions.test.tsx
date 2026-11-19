// actions: real interactions through the gallery.
import { describe, expect, test } from "nsh/test";
import { enterPage, gallery } from "./testing";

describe("actions", () => {
  test("buttons press, disabled ones do not", async () => {
    const app = await gallery("button");
    await app.press("role:button[Default]");
    await app.press("role:button[Primary]");
    await app.press("role:button[Delete]");
    await app.waitFor("text:Pressed 3 times");
    expect(app.enabled("role:button[Disabled]")).toBe(false);
  });

  test("an icon button presses by its label", async () => {
    const app = await gallery("icon-button");
    await app.press("label:Add");
    await app.waitFor("text:Added 1 times");
    expect(app.enabled("label:Next")).toBe(false);
  });

  test("a toggle switches", async () => {
    const app = await gallery("toggle");
    await app.press("role:button[Bold]");
    await app.waitFor("text:Bold on");
    expect(app.enabled("role:button[Locked]")).toBe(false);
  });

  test("a toggle group switches its toggles", async () => {
    const app = await gallery("toggle-group");
    await app.press("role:button[U]");
    await app.waitFor("text:Marks: italic, underline");
    await app.press("role:button[I]");
    await app.waitFor("text:Marks: underline");
  });

  test("the toggle group is one stop, the arrows move inside it", async () => {
    const app = await gallery("toggle-group");
    // The group is the page's first stop, entered at its pressed toggle (I).
    await enterPage(app);
    await app.keyboard.press("right");
    await app.keyboard.press("space");
    await app.waitFor("text:Marks: italic, underline");
  });

  test("a button group passes presses and disables its buttons", async () => {
    const app = await gallery("button-group");
    await app.press("role:button[Float]");
    await app.waitFor("text:Layout: float · zoom 100%");
    await app.press("label:Zoom in");
    await app.waitFor("text:Layout: float · zoom 110%");
    expect(app.enabled("role:button[Paste]")).toBe(false);
  });

  test("an app link reports its press", async () => {
    const app = await gallery("link");
    await app.press("role:link[Release notes]");
    await app.waitFor("text:Opened: /releases");
  });
});
