// Real interactions with every family of the library, through the gallery.
import { describe, expect, open, test } from "nsh/test";
import type { App } from "nsh/test";

/** Opens the gallery on the page `page` (its sidebar label). */
async function gallery(page: string) {
  const app = await open("App.tsx", { width: 1200, height: 900, platform: "linux", clock: {} });
  if (page !== "Actions") await app.press(`role:tab[${page}]`);
  await app.waitFor(`text:${page}`);
  return app;
}

/**
 * Moves the keyboard focus from stop to stop (Tab), pressing `key` at each,
 * until `done`: a press gives no keyboard focus on a desktop, Tab does.
 */
async function tabUntil(app: App, key: string, done: () => boolean) {
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
async function tabTo(app: App, selector: string) {
  const resting = app.viewStyle(selector)?.backgroundColor;
  for (let stops = 0; app.viewStyle(selector)?.backgroundColor === resting; stops++) {
    expect(stops).toBeLessThan(60);
    await app.keyboard.press("tab");
    await app.wait(20);
  }
}

describe("sidebar", () => {
  test("a press and the arrows change the page", async () => {
    const app = await gallery("Actions");
    expect(app.shows("text:Pressed 0 times")).toBe(true);
    await app.press("role:tab[Choices]");
    await app.waitFor("text:Send notifications");
    expect(app.shows("text:Pressed 0 times")).toBe(false);
    await tabUntil(app, "down", () => app.shows("text:Hostname"));
    await app.keyboard.press("end");
    await app.waitFor("text:Typography");
    await app.keyboard.press("home");
    await app.waitFor("text:Pressed 0 times");
  });

  test("the dark switch changes the color scheme", async () => {
    const app = await gallery("Actions");
    const light = app.viewStyle("role:tab[Actions]");
    await app.press("role:switch[Dark]");
    await app.waitUntil(() => JSON.stringify(app.viewStyle("role:tab[Actions]")) !== JSON.stringify(light));
    await app.press("role:switch[Dark]");
    await app.waitUntil(() => JSON.stringify(app.viewStyle("role:tab[Actions]")) === JSON.stringify(light));
  });
});

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
    await tabTo(app, "label:Download progress");
    await app.keyboard.press("end");
    await app.waitFor("text:100%");
    await app.waitFor("text:100%");
  });
});

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
    await app.waitFor("text:Ctrl+N");
    await app.press("text:Word wrap");
    await app.waitFor("text:Chose: nothing · wrap off");
    expect(app.shows("text:Ctrl+N")).toBe(true);
    await app.press("text:Open…");
    await app.waitFor("text:Chose: Open · wrap off");
    await app.waitFor("text:Ctrl+N", { state: "gone" });
    await app.press("role:button[File]");
    await app.waitFor("text:Ctrl+N");
    await app.keyboard.press("escape");
    await app.waitFor("text:Ctrl+N", { state: "gone" });
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
    await app.press("label:Dismiss");
    await app.waitFor("text:Update failed", { state: "gone" });
  }, 20000);
});

describe("display", () => {
  test("shows every piece", async () => {
    const app = await gallery("Display");
    for (const text of ["Heading", "Updates", "Inset panel", "ONLINE", "Esc", "Super", "No windows", "Open launcher"]) {
      expect(app.shows(`text:${text}`)).toBe(true);
    }
    expect(app.shows("text:GH")).toBe(true);
  });
});
