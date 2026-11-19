// The Omarchy TSX gallery: every component of nsh-omarchy (installed with
// `nsh add nsh-omarchy`), one page each, grouped by family in the sidebar.
import { host, mount, Scroll, signal, Text, View } from "nsh";
import { Caption, Omarchy, Sidebar, SidebarItem, SidebarSection, Tab, Tabs, Toaster } from "nsh-omarchy";
import { families, pages } from "./pages";

/** Draws with the Omarchy system theme, or one of the built-in palettes. */
function pickTheme(value: string) {
  if (value === "light") Omarchy.applyTheme("flexoki-light");
  else if (value === "dark") Omarchy.applyTheme("tokyo-night");
  else Omarchy.followSystem();
}

export function App() {
  const theme = signal("system");
  pickTheme(theme());
  // On the website, the page's host (host.js) sends the site's theme.
  host.on("theme", (payload) => {
    const value = (payload as { theme?: string } | null)?.theme;
    if (value !== "light" && value !== "dark") return;
    theme.set(value);
    pickTheme(value);
  });
  const current = signal(pages[0].value);
  return (
    <View class="flex-1 flex-row bg-background">
      <Sidebar
        accessibilityLabel="Components"
        value={current()}
        onChange={({ value }) => current.set(value)}
        header={
          <View class="gap-1 px-2 pt-2 pb-1">
            <Caption>Appearance</Caption>
            <Tabs
              value={theme()}
              variant="segmented"
              size="small"
              accessibilityLabel="Appearance"
              onChange={({ value }) => {
                theme.set(value);
                pickTheme(value);
              }}
            >
              <Tab value="system" label="System" />
              <Tab value="light" label="Light" />
              <Tab value="dark" label="Dark" />
            </Tabs>
          </View>
        }
      >
        <Each of={families} key={(f) => f.title}>
          {(family) => (
            <SidebarSection title={family().title}>
              <Each of={family().pages} key={(p) => p.value}>
                {(p) => <SidebarItem testID={`page-${p().value}`} value={p().value} label={p().title} />}
              </Each>
            </SidebarSection>
          )}
        </Each>
      </Sidebar>
      <Scroll class="flex-1" testID="page">
        <View class="gap-6 p-6">
          <Each of={pages} key={(p) => p.value}>
            {(p) => (
              <If when={current() === p().value}>
                <Text class="text-2xl font-semibold text-bright">{p().title}</Text>
                {(() => {
                  const Page = p().page;
                  return <Page />;
                })()}
              </If>
            )}
          </Each>
        </View>
      </Scroll>
      <Toaster />
    </View>
  );
}

mount(() => <App />);
