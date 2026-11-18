// Buttons, icon buttons, toggles and single-choice groups.
import { signal, View } from "nsh";
import { Button, ButtonGroup, Glyph, IconButton, Shortcut, Toggle, ToggleGroup, Tooltip } from "../../../tsx";
import { Row, Section, Status } from "./Section";

export function Actions() {
  const presses = signal(0);
  const bold = signal(false);
  const marks = signal<string[]>(["italic"]);
  const layout = signal("tile");
  return (
    <View class="gap-6">
      <Section title="Button" description="Default, primary, ghost and danger; 24, 28 and 32 points tall.">
        <Row>
          <Button label="Default" onPress={() => presses.update((n) => n + 1)} />
          <Button label="Primary" variant="primary" onPress={() => presses.update((n) => n + 1)} />
          <Button label="Ghost" variant="ghost" onPress={() => presses.update((n) => n + 1)} />
          <Button label="Delete" variant="danger" onPress={() => presses.update((n) => n + 1)} />
          <Button label="Disabled" disabled />
        </Row>
        <Row>
          <Button label="Small" size="sm" />
          <Button label="Medium" size="md" />
          <Button label="Large" size="lg" />
          <Button label="New workspace" variant="primary" icon={<Glyph name="plus" />} />
          <Button label="Launcher">
            <Shortcut keys={["Super", "Space"]} />
          </Button>
          <Button label="Selected" selected />
        </Row>
        <Status text={`Pressed ${presses()} times`} />
      </Section>
      <Section title="IconButton" description="A square glyph button; its label is for screen readers, its tooltip for the pointer.">
        <Row>
          <IconButton accessibilityLabel="Add" tooltip={<Tooltip label="Add" shortcut="mod-n" />}>
            <Glyph name="plus" />
          </IconButton>
          <IconButton accessibilityLabel="Menu" variant="default">
            <Glyph name="menu" />
          </IconButton>
          <IconButton accessibilityLabel="Close" variant="danger" size="sm">
            <Glyph name="cross" />
          </IconButton>
          <IconButton accessibilityLabel="Next" size="lg" disabled>
            <Glyph name="chevron-right" />
          </IconButton>
        </Row>
      </Section>
      <Section title="Toggle and ToggleGroup" description="On or off; a group is one Tab stop, the arrows move between toggles.">
        <Row>
          <Toggle label="Bold" pressed={bold()} onChange={({ pressed }) => bold.set(pressed)} />
          <Toggle label="Locked" pressed disabled />
        </Row>
        <ToggleGroup
          items={[
            { value: "bold", label: "B" },
            { value: "italic", label: "I" },
            { value: "underline", label: "U" },
          ]}
          value={marks()}
          onChange={({ value }) => marks.set(value)}
        />
        <Status text={`Bold ${bold() ? "on" : "off"} · marks: ${marks().join(", ") || "none"}`} />
      </Section>
      <Section title="ButtonGroup" description="A setting with one value: arrows move the focus, Space or Return choose.">
        <ButtonGroup
          items={[
            { value: "tile", label: "Tile" },
            { value: "float", label: "Float" },
            { value: "stack", label: "Stack" },
            { value: "full", label: "Full", disabled: true },
          ]}
          value={layout()}
          onChange={({ value }) => layout.set(value)}
        />
        <Status text={`Layout: ${layout()}`} />
      </Section>
    </View>
  );
}
