// Pictures of every page in light and dark, for a person to look at. Run on a
// real platform, which draws the app, into `shots/` (a headless run draws
// nothing, so there these tests return at once):
//   mkdir -p shots && nsh test --target macos Screenshots.test.tsx
// (screenshot() does not create the directory).
import { open, test } from "nsh/test";
import { pages } from "./pages";

const dir = "shots";

for (const scheme of ["light", "dark"] as const) {
  test(`every page in ${scheme}`, async () => {
    const app = await open("App.tsx", { width: 1200, height: 900, platform: "linux", colorScheme: scheme });
    // Headless, a screenshot rejects: nothing to look at.
    if (!(await app.screenshot().then(() => true, () => false))) return;
    await app.press(scheme === "light" ? "text:Light" : "text:Dark");
    for (const page of pages) {
      await app.press(`id:page-${page.value}`);
      await app.wait(300);
      await app.screenshot(`${dir}/${scheme}-${page.value}.png`);
    }
    const shot = (name: string) => app.screenshot(`${dir}/${scheme}-open-${name}.png`);
    await app.press("id:page-dialog");
    await app.press("role:button[Rename workspace]");
    await app.wait(400);
    await shot("dialog");
    await app.press("role:button[Cancel]");
    await app.press("id:page-sheet");
    await app.press("role:button[Open settings]");
    await app.wait(400);
    await shot("sheet");
    await app.press("role:button[Done]");
    await app.press("id:page-toast");
    await app.press("role:button[Notify]");
    await app.press("role:button[With action]");
    await app.press("role:button[Error]");
    await app.wait(400);
    await shot("toasts");
    await app.press("id:page-menu");
    await app.press("role:button[File]");
    await app.wait(400);
    await shot("menu");
    await app.keyboard.press("escape");
    await app.press("id:page-select");
    await app.press("label:Theme");
    await app.wait(400);
    await shot("select");
  }, 120000);
}
