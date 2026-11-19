// Buttons, icon buttons, toggles, groups and links.
import type { JSX } from "nsh";
import { signal, Text, View } from "nsh";
import { Button, ButtonGroup, IconButton, Link, Toggle, ToggleGroup } from "nsh-omarchy";

/** A demo block: a title, a line on what it shows, and the demo on a card. */
function Block(props: { title: string; description: string; children: JSX.Element }) {
  return (
    <View class="gap-2">
      <View class="gap-0.5">
        <Text class="text-[14px] font-semibold text-bright">{props.title}</Text>
        <Text class="text-[11px] text-text-secondary">{props.description}</Text>
      </View>
      <View class="gap-3 border border-border bg-surface p-4">{props.children}</View>
    </View>
  );
}

/** A wrapping row of demos. */
function Row(props: { children: JSX.Element }) {
  return <View class="flex-row flex-wrap items-center gap-2">{props.children}</View>;
}

/** What the last action did, for the reader (and the tests). */
function Status(props: { text: string }) {
  return <Text class="text-[11px] text-text-secondary">{props.text}</Text>;
}

export function Actions() {
  const presses = signal(0);
  const bold = signal(false);
  const marks = signal<string[]>(["italic"]);
  const layout = signal("tile");
  const zoom = signal(100);
  const opened = signal("nothing yet");
  const press = () => presses.update((n) => n + 1);
  return (
    <View class="gap-6">
      <Block title="Button" description="Default, primary, ghost and danger; 24, 28 and 32 points tall.">
        <Row>
          <Button label="Default" onPress={press} />
          <Button label="Primary" variant="primary" onPress={press} />
          <Button label="Ghost" variant="ghost" onPress={press} />
          <Button label="Delete" variant="danger" onPress={press} />
          <Button label="Disabled" disabled />
        </Row>
        <Row>
          <Button label="Small" size="small" />
          <Button label="Medium" size="medium" />
          <Button label="Large" size="large" />
          <Button label="New workspace" variant="primary" icon="plus" />
          <Button label="Saving" loading />
          <Button label="Selected" selected />
        </Row>
        <Status text={`Pressed ${presses()} times`} />
      </Block>
      <Block title="IconButton" description="A square glyph button; its label is for screen readers.">
        <Row>
          <IconButton icon="plus" label="Add" onPress={press} />
          <IconButton icon="menu" label="Menu" variant="default" />
          <IconButton icon="x" label="Close" variant="danger" size="small" />
          <IconButton icon="chevron-right" label="Next" size="large" disabled />
        </Row>
      </Block>
      <Block title="Toggle and ToggleGroup" description="On or off; a group is one Tab stop, the arrows move between toggles.">
        <Row>
          <Toggle label="Bold" checked={bold()} onChange={({ checked }) => bold.set(checked)} />
          <Toggle label="Favorite" icon="star" />
          <Toggle label="Locked" checked disabled />
        </Row>
        <ToggleGroup value={marks()} onChange={({ value }) => marks.set(value)}>
          <Toggle value="bold" label="B" />
          <Toggle value="italic" label="I" />
          <Toggle value="underline" label="U" />
        </ToggleGroup>
        <Status text={`Bold ${bold() ? "on" : "off"} · marks: ${marks().join(", ") || "none"}`} />
      </Block>
      <Block title="ButtonGroup" description="Buttons joined edge to edge; each is a stop of Tab.">
        <ButtonGroup>
          <Button label="Tile" selected={layout() === "tile"} onPress={() => layout.set("tile")} />
          <Button label="Float" selected={layout() === "float"} onPress={() => layout.set("float")} />
          <Button label="Stack" selected={layout() === "stack"} onPress={() => layout.set("stack")} />
        </ButtonGroup>
        <Row>
          <ButtonGroup size="small">
            <IconButton icon="minus" label="Zoom out" variant="default" onPress={() => zoom.update((z) => z - 10)} />
            <Button label="Reset zoom" onPress={() => zoom.set(100)} />
            <IconButton icon="plus" label="Zoom in" variant="default" onPress={() => zoom.update((z) => z + 10)} />
          </ButtonGroup>
          <ButtonGroup disabled>
            <Button label="Cut" />
            <Button label="Paste" />
          </ButtonGroup>
        </Row>
        <Status text={`Layout: ${layout()} · zoom ${zoom()}%`} />
      </Block>
      <Block title="Link" description="Underlined accent text; the system opens a URL, the app a path of its own.">
        <Row>
          <Link label="Omarchy manual" href="https://omarchy.org/manual" />
          <Link label="Release notes" href="/releases" openWith="app" onPress={() => opened.set("/releases")} />
        </Row>
        <Status text={`Opened: ${opened()}`} />
      </Block>
    </View>
  );
}
