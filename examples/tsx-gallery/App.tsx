// The Omarchy TSX gallery: every component of `../../tsx`, one page per
// family, navigated from a sidebar, with a light / dark switch.
import { colorScheme, mount, Scroll, setColorScheme, signal, View } from "nsh";
import { Caption, Heading, Sidebar, Switch, Title, Toaster } from "../../tsx";
import { Actions } from "./pages/Actions";
import { Choices } from "./pages/Choices";
import { Display } from "./pages/Display";
import { Fields } from "./pages/Fields";
import { Navigation } from "./pages/Navigation";
import { Overlays } from "./pages/Overlays";

const pages = [
  { value: "actions", label: "Actions", title: "Actions", page: Actions },
  { value: "choices", label: "Choices", title: "Choices", page: Choices },
  { value: "fields", label: "Fields", title: "Fields", page: Fields },
  { value: "navigation", label: "Navigation", title: "Navigation", page: Navigation },
  { value: "overlays", label: "Overlays", title: "Overlays", page: Overlays },
  { value: "display", label: "Display", title: "Display", page: Display },
];

export function App() {
  const current = signal("actions");
  const shown = () => pages.find((p) => p.value === current()) ?? pages[0];
  return (
    <View class="flex-1 flex-row bg-background">
      <Sidebar
        accessibilityLabel="Components"
        value={current()}
        onChange={({ value }) => current.set(value)}
        sections={[{ title: "Components", items: pages.map((p) => ({ value: p.value, label: p.label })) }]}
        header={
          <View class="gap-0.5 px-3 pb-2">
            <Title>Omarchy</Title>
            <Caption>nsh-omarchy · TSX</Caption>
          </View>
        }
        footer={
          <View class="border-t border-foreground/12 px-2 pt-2">
            <Switch
              label="Dark"
              checked={colorScheme() === "dark"}
              onChange={({ checked }) => setColorScheme(checked ? "dark" : "light")}
            />
          </View>
        }
      />
      <Scroll class="flex-1" testID="page">
        <View class="gap-6 p-6">
          <Heading>{shown().title}</Heading>
          <Each of={pages} key={(p) => p.value}>
            {(p) => (
              <If when={current() === p().value}>
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
