// The Rust exports of nsh-omarchy as a mixed app's TSX sees them: the
// `Omarchy` theme module, a chart, `Markdown` and `Icon`, imported from the
// crate's specifier. `nsh run App.tsx` runs it; `nsh run` runs the Rust
// gallery (nsh.json "root").
import { mount, Scroll, signal, Text, View } from "nsh";
import { Button } from "nsh/base";
import { BarChart, type BuiltInTheme, Icon, Markdown, Omarchy, type ThemeInfo } from "nsh-omarchy";

const NOTES = `## Release notes

Omarchy's palettes reach TSX as **tokens**. Read the [manual](https://learn.omacom.io/2/the-omarchy-manual).`;

/** A press target in Omarchy's look, from the theme's tokens. */
function ThemeButton(props: { title: string; onPress: () => void }) {
  return (
    <Button
      onPress={props.onPress}
      class="px-3 py-1.5 border border-border bg-secondary active:bg-accent focus-visible:border-ring"
    >
      <Text class="text-sm text-foreground">{props.title}</Text>
    </Button>
  );
}

function App() {
  // Rust owns the theme; this signal mirrors it from `onChange`.
  const theme = signal<ThemeInfo | null>(Omarchy.current());
  Omarchy.onChange((t) => theme.set(t));
  const linked = signal("No link pressed");
  const apply = (name: BuiltInTheme) => () => Omarchy.applyTheme(name);
  Omarchy.followSystem();

  return (
    <Scroll class="flex-1 bg-background">
      <View class="gap-4 p-6">
        <View class="flex-row items-center gap-2">
          <Icon name="sun" label="Theme" class="text-primary" />
          <Text class="text-lg font-semibold text-foreground">
            {() => (theme() ? `Theme: ${theme()!.name} (${theme()!.appearance})` : "Theme: none yet")}
          </Text>
        </View>
        <View class="flex-row gap-2">
          <ThemeButton title="Tokyo Night" onPress={apply("tokyo-night")} />
          <ThemeButton title="Flexoki Light" onPress={apply("flexoki-light")} />
          <ThemeButton title="Follow system" onPress={() => Omarchy.followSystem()} />
        </View>
        <BarChart
          data={[
            { label: "Mon", value: 12 },
            { label: "Tue", value: 18 },
            { label: "Wed", value: 15 },
          ]}
          name="Builds"
          unit=" builds"
          accessibilityLabel="Builds this week"
        />
        <Markdown source={NOTES} onLinkPress={(e) => linked.set(`Link: ${e.value}`)} />
        <Text class="text-sm text-muted-foreground">{() => linked()}</Text>
      </View>
    </Scroll>
  );
}

mount(App);
