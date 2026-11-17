// Pictures of every page in light and dark, for a person to look at. Run on a
// real platform, which draws the app, into `shots/` (a headless run draws
// nothing, so there these tests return at once):
//   mkdir -p shots && nsh test --target macos Screenshots.test.tsx
// (screenshot() does not create the directory).
import { open, test } from "nsh/test";

const dir = "shots";
const pages = ["Actions", "Choices", "Fields", "Navigation", "Overlays", "Display"];

for (const scheme of ["light", "dark"] as const) {
  test(`every page in ${scheme}`, async () => {
    const app = await open("App.tsx", { width: 1200, height: 900, platform: "linux", colorScheme: scheme });
    // Headless, a screenshot rejects: nothing to look at.
    if (!(await app.screenshot().then(() => true, () => false))) return;
    for (const page of pages) {
      await app.press(`role:tab[${page}]`);
      await app.wait(300);
      await app.screenshot(`${dir}/${scheme}-${page.toLowerCase()}.png`);
      // The rest of a long page.
      await app.scroll("id:page", 800);
      await app.wait(300);
      await app.screenshot(`${dir}/${scheme}-${page.toLowerCase()}-2.png`);
    }
    const shot = (name: string) => app.screenshot(`${dir}/${scheme}-open-${name}.png`);
    await app.press("role:tab[Overlays]");
    await app.press("role:button[Rename workspace]");
    await app.wait(400);
    await shot("dialog");
    await app.press("role:button[Cancel]");
    await app.press("role:button[Open settings]");
    await app.wait(400);
    await shot("sheet");
    await app.press("role:button[Done]");
    await app.press("role:button[Details]");
    await app.press("role:button[Notify]");
    await app.press("role:button[With action]");
    await app.press("role:button[Error]");
    await app.wait(400);
    await shot("toasts");
    await app.press("role:tab[Navigation]");
    await app.press("role:button[File]");
    await app.wait(400);
    await shot("menu");
    await app.keyboard.press("escape");
    await app.press("role:tab[Fields]");
    await app.scroll("id:page", 800);
    await app.press("label:Theme");
    await app.wait(400);
    await shot("select");
  }, 60000);
}
