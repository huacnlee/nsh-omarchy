// data: real interactions through the gallery.
import { describe, expect, test } from "nsh/test";
import { gallery } from "./testing";

describe("data", () => {
  test("a table row selects by press", async () => {
    const app = await gallery("table");
    await app.waitFor("text:Selected pid 1207");
    await app.press("label:alacritty 1214");
    await app.waitFor("text:Selected pid 1214");
    // The selected row keeps the selected fill; the others lose it.
    const fill = app.viewStyle("label:alacritty 1214")?.backgroundColor;
    expect(fill).toBeDefined();
    expect(app.viewStyle("label:waybar 1207")?.backgroundColor).not.toBe(fill);
  });

  test("a data table sorts and selects rows by id", async () => {
    const app = await gallery("data-table");
    await app.waitFor("text:Sort none · selected none");
    await app.press("text:1228");
    await app.waitFor("text:Sort none · selected 1228");
    await app.press("id:sort:cpu");
    await app.waitFor("text:Sort cpu ascending · selected 1228");
    await app.press("id:sort:cpu");
    await app.waitFor("text:Sort cpu descending · selected 1228");
    await app.press("id:sort:cpu");
    await app.waitFor("text:Sort cpu ascending · selected 1228");
  });

  test("a virtual list selects by id; a disabled item takes no press", async () => {
    const app = await gallery("list");
    await app.press("label:Item 3");
    await app.waitFor("text:Selected item 3");
    expect(app.enabled("label:Item 2")).toBe(false);
    await app.scroll("id:list", 2800);
    await app.waitFor("label:Item 105");
    await app.press("label:Item 105");
    await app.waitFor("text:Selected item 105");
  });

  test("a tree folds and selects", async () => {
    const app = await gallery("tree");
    await app.waitFor("role:treeitem[hypr]");
    await app.press("role:treeitem[themes]");
    await app.waitFor("text:Selected themes · expanded config themes");
    await app.waitFor("role:treeitem[tokyo-night]");
    await app.press("role:treeitem[config]");
    await app.waitFor("text:Selected config · expanded themes");
  });

  test("resizable panels report their sizes", async () => {
    const app = await gallery("resizable");
    await app.waitFor("text:Sizes as laid out");
    await app.drag("label:Resize panels horizontally", { x: 40, y: 0 });
    await app.waitFor("text:/Sizes 240 /");
  });
});
