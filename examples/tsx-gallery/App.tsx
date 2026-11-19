// The Omarchy TSX gallery: every component of nsh-omarchy (installed with
// `nsh add nsh-omarchy`), one page per family.
import { mount, Scroll, signal, Text, View } from "nsh";
import { Button, Omarchy, Toaster } from "nsh-omarchy";
import { Actions } from "./pages/Actions";
import { Choices } from "./pages/Choices";
import { Data } from "./pages/Data";
import { Display } from "./pages/Display";
import { Fields } from "./pages/Fields";
import { Navigation } from "./pages/Navigation";
import { Overlays } from "./pages/Overlays";

const pages = [
  { value: "actions", title: "Actions", page: Actions },
  { value: "choices", title: "Choices", page: Choices },
  { value: "fields", title: "Fields", page: Fields },
  { value: "navigation", title: "Navigation", page: Navigation },
  { value: "overlays", title: "Overlays", page: Overlays },
  { value: "data", title: "Data", page: Data },
  { value: "display", title: "Display", page: Display },
];

export function App() {
  Omarchy.applyTheme("tokyo-night");
  const current = signal("actions");
  return (
    <View class="flex-1 flex-row bg-background">
      <View class="w-[200px] gap-1 border-r border-foreground/12 p-2">
        <Each of={pages} key={(p) => p.value}>
          {(p) => (
            <Button
              testID={`page-${p().title}`}
              label={p().title}
              variant="ghost"
              selected={current() === p().value}
              onPress={() => current.set(p().value)}
            />
          )}
        </Each>
      </View>
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
