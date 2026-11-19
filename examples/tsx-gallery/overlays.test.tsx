// overlays: real interactions through the gallery.
import { describe, expect, test } from "nsh/test";
import { gallery } from "./testing";

describe("overlays", () => {
  test("a dialog opens, edits, saves, and Escape closes it", async () => {
    const app = await gallery("Overlays");
    await app.press("role:button[Rename workspace]");
    await app.waitFor("label:Workspace name");
    await app.type("label:Workspace name", " main");
    await app.press("role:button[Save]");
    await app.waitFor("label:Workspace name", { state: "gone" });
    await app.waitFor("text:Last: Renamed to Workspace 1 main");
    await app.press("role:button[Rename workspace]");
    await app.waitFor("label:Workspace name");
    // The name field has the focus and keeps Escape (nsh/base): Tab leaves it.
    await app.keyboard.press("tab");
    await app.keyboard.press("escape");
    await app.waitFor("label:Workspace name", { state: "gone" });
  });

  test("an alert dialog confirms", async () => {
    const app = await gallery("Overlays");
    await app.press("role:button[Delete workspace]");
    await app.waitFor("text:Delete workspace?");
    await app.press("role:button[Cancel]");
    await app.waitFor("text:Delete workspace?", { state: "gone" });
    expect(app.shows("text:Last: nothing")).toBe(true);
    await app.press("role:button[Delete workspace]");
    await app.press("role:button[Delete]");
    await app.waitFor("text:Last: Deleted");
  });

  test("the native confirm answers", async () => {
    const app = await gallery("Overlays");
    app.onDialog(() => "Restart");
    await app.press("role:button[Native confirm]");
    await app.waitFor("text:Last: Restarted");
    expect(app.dialogs()[0]?.title).toBe("Restart Hyprland?");
  });

  test("a danger confirm is answered by its cancel", async () => {
    const app = await gallery("Overlays");
    app.onDialog(() => "Cancel");
    await app.press("role:button[Danger confirm]");
    await app.waitFor("text:Last: Files kept");
    expect(app.dialogs()[0]?.title).toBe("Delete 3 files?");
  });

  test("a sheet opens and its close button closes it", async () => {
    const app = await gallery("Overlays");
    await app.press("role:button[Open settings]");
    await app.waitFor("text:Quick settings");
    await app.press("role:switch[Do not disturb]");
    await app.press("label:Close");
    await app.waitFor("text:Quick settings", { state: "gone" });
    await app.press("role:button[Open settings]");
    await app.waitFor("text:Quick settings");
    await app.press("role:button[Done]");
    await app.waitFor("text:Quick settings", { state: "gone" });
  });

  test("a popover opens and Escape closes it", async () => {
    const app = await gallery("Overlays");
    await app.press("role:button[Details]");
    await app.waitFor("text:82%, about 5 hours left.");
    await app.keyboard.press("escape");
    await app.waitFor("text:82%, about 5 hours left.", { state: "gone" });
  });

  test("a tooltip and a hover card show on hover", async () => {
    const app = await gallery("Overlays");
    await app.hover("role:button[Hover me]");
    await app.waitFor("text:Lock screen");
    await app.hover("role:button[@dhh]");
    await app.waitFor("text:Creator of Omarchy.");
  });

  test("toasts show, run their action, time out, and errors stay", async () => {
    const app = await gallery("Overlays");
    await app.press("role:button[With action]");
    await app.waitFor("text:Window closed");
    await app.press("role:button[Undo]");
    await app.waitFor("text:Window closed", { state: "gone" });
    await app.waitFor("text:Last: Undone");
    await app.press("role:button[Error]");
    await app.press("role:button[Notify]");
    await app.waitFor("text:Screenshot saved");
    await app.wait(6500);
    await app.waitFor("text:Screenshot saved", { state: "gone" });
    expect(app.shows("text:Update failed")).toBe(true);
    await app.press("label:Dismiss notification");
    await app.waitFor("text:Update failed", { state: "gone" });
  }, 20000);

  test("dismissToast removes a toast by its id, and an id replaces its toast", async () => {
    const app = await gallery("Overlays");
    await app.press("role:button[With action]");
    await app.waitFor("text:Window closed");
    await app.press("role:button[Dismiss last]");
    await app.waitFor("text:Window closed", { state: "gone" });
    await app.press("role:button[Progress]");
    await app.waitFor("text:Step 2 of 2");
    await app.waitFor("text:Step 1 of 2", { state: "gone" });
    await app.wait(2500);
    await app.waitFor("text:Step 2 of 2", { state: "gone" });
  }, 20000);
});
