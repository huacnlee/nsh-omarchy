// Buttons, icon buttons, toggles, groups and links.
import { signal } from "nsh";
import { Button, ButtonGroup, IconButton, Link, Toggle, ToggleGroup } from "nsh-omarchy";
import { Demos, type Family, Row, Section, Status } from "./Section";

function ButtonPage() {
  const presses = signal(0);
  const press = () => presses.update((n) => n + 1);
  return (
    <Demos>
      <Section title="Variants" description="Default, primary, ghost and danger.">
        <Row>
          <Button label="Default" onPress={press} />
          <Button label="Primary" variant="primary" onPress={press} />
          <Button label="Ghost" variant="ghost" onPress={press} />
          <Button label="Delete" variant="danger" onPress={press} />
          <Button label="Disabled" disabled />
        </Row>
        <Status text={`Pressed ${presses()} times`} />
      </Section>
      <Section title="Sizes and states" description="24, 28 and 32 points tall; an icon, loading, selected.">
        <Row>
          <Button label="Small" size="small" />
          <Button label="Medium" size="medium" />
          <Button label="Large" size="large" />
        </Row>
        <Row>
          <Button label="New workspace" variant="primary" icon="plus" />
          <Button label="Saving" loading />
          <Button label="Selected" selected />
        </Row>
      </Section>
    </Demos>
  );
}

function IconButtonPage() {
  const presses = signal(0);
  return (
    <Demos>
      <Section title="IconButton" description="A square glyph button; its label is for screen readers.">
        <Row>
          <IconButton icon="plus" label="Add" onPress={() => presses.update((n) => n + 1)} />
          <IconButton icon="menu" label="Menu" variant="default" />
          <IconButton icon="x" label="Close" variant="danger" size="small" />
          <IconButton icon="chevron-right" label="Next" size="large" disabled />
        </Row>
        <Status text={`Added ${presses()} times`} />
      </Section>
    </Demos>
  );
}

function TogglePage() {
  const bold = signal(false);
  return (
    <Demos>
      <Section title="Toggle" description="On or off, with a label and an optional glyph.">
        <Row>
          <Toggle label="Bold" checked={bold()} onChange={({ checked }) => bold.set(checked)} />
          <Toggle label="Favorite" icon="star" />
          <Toggle label="Locked" checked disabled />
        </Row>
        <Status text={`Bold ${bold() ? "on" : "off"}`} />
      </Section>
    </Demos>
  );
}

function ToggleGroupPage() {
  const marks = signal<string[]>(["italic"]);
  return (
    <Demos>
      <Section title="ToggleGroup" description="One Tab stop; the arrows move between toggles.">
        <ToggleGroup value={marks()} onChange={({ value }) => marks.set(value)}>
          <Toggle value="bold" label="B" />
          <Toggle value="italic" label="I" />
          <Toggle value="underline" label="U" />
        </ToggleGroup>
        <Status text={`Marks: ${marks().join(", ") || "none"}`} />
      </Section>
    </Demos>
  );
}

function ButtonGroupPage() {
  const layout = signal("tile");
  const zoom = signal(100);
  return (
    <Demos>
      <Section title="ButtonGroup" description="Buttons joined edge to edge; each is a stop of Tab.">
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
      </Section>
    </Demos>
  );
}

function LinkPage() {
  const opened = signal("nothing yet");
  return (
    <Demos>
      <Section title="Link" description="Underlined accent text; the system opens a URL, the app a path of its own.">
        <Row>
          <Link label="Omarchy manual" href="https://omarchy.org/manual" />
          <Link label="Release notes" href="/releases" openWith="app" onPress={() => opened.set("/releases")} />
        </Row>
        <Status text={`Opened: ${opened()}`} />
      </Section>
    </Demos>
  );
}

export const actions: Family = {
  title: "Actions",
  pages: [
    { value: "button", title: "Button", page: ButtonPage },
    { value: "icon-button", title: "IconButton", page: IconButtonPage },
    { value: "toggle", title: "Toggle", page: TogglePage },
    { value: "toggle-group", title: "ToggleGroup", page: ToggleGroupPage },
    { value: "button-group", title: "ButtonGroup", page: ButtonGroupPage },
    { value: "link", title: "Link", page: LinkPage },
  ],
};
