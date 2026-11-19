// The gallery's host on the web (nsh.json "web.host"). Embedded in the
// website, it sends the site's theme to the app, now and on every change.
const appearance = { "tokyo-night": "dark", "flexoki-light": "light" };

export function createHost(player) {
  let root = null;
  try {
    if (typeof window !== "undefined" && window.parent !== window) root = window.parent.document.documentElement;
  } catch {
    // Another origin embeds the gallery: it keeps its own appearance.
  }
  if (root) {
    const send = () => {
      const theme = appearance[root.dataset.theme];
      if (theme) player.hostEvent("theme", { theme });
    };
    send();
    new MutationObserver(send).observe(root, { attributes: true, attributeFilter: ["data-theme"] });
  }
  return {
    call(name) {
      throw new Error(`no host function ${name}`);
    },
  };
}
