// Tabs, menus, accordions, pagination and links (the sidebar is the app's own).
import { signal, Text, View } from "nsh";
import {
  Accordion,
  AccordionSection,
  Body,
  Button,
  ContextMenu,
  DropdownMenu,
  Glyph,
  Link,
  MenuItem,
  MenuLabel,
  MenuSeparator,
  Pagination,
  Tabs,
} from "../../../tsx";
import { Row, Section, Status } from "./Section";

const tabItems = [
  { value: "general", label: "General" },
  { value: "keys", label: "Keybindings" },
  { value: "display", label: "Display" },
  { value: "beta", label: "Beta", disabled: true },
];

export function Navigation() {
  const tab = signal("general");
  const view = signal("list");
  const last = signal("nothing");
  const wrap = signal(true);
  const open = signal<string[]>(["what"]);
  const page = signal(1);
  return (
    <View class="gap-6">
      <Section title="Tabs" description="Plain and segmented; the arrows move and select, the selected fill slides.">
        <Tabs accessibilityLabel="Settings" items={tabItems} value={tab()} onChange={({ value }) => tab.set(value)} />
        <Body>{`Panel: ${tab()}`}</Body>
        <Tabs
          variant="segmented"
          accessibilityLabel="View"
          items={[
            { value: "list", label: "List" },
            { value: "grid", label: "Grid" },
            { value: "board", label: "Board" },
          ]}
          value={view()}
          onChange={({ value }) => view.set(value)}
        />
        <Status text={`View: ${view()}`} />
      </Section>
      <Section title="DropdownMenu and ContextMenu" description="Rows with shortcuts in monospace, checkable rows, a destructive row.">
        <Row>
          <DropdownMenu trigger={<Button label="File"><Glyph name="chevron-down" /></Button>}>
            <MenuLabel label="Document" />
            <MenuItem label="New" shortcut="Ctrl+N" onSelect={() => last.set("New")} />
            <MenuItem label="Open…" shortcut="Ctrl+O" onSelect={() => last.set("Open")} />
            <MenuItem label="Word wrap" checked={wrap()} closeOnSelect={false} onSelect={() => wrap.update((v) => !v)} />
            <MenuItem label="Print" disabled />
            <MenuSeparator />
            <MenuItem label="Delete" danger onSelect={() => last.set("Delete")} />
          </DropdownMenu>
          <ContextMenu
            class="h-16 w-64 items-center justify-center border border-dashed border-foreground/40"
            menu={
              <>
                <MenuItem label="Copy" shortcut="Ctrl+C" onSelect={() => last.set("Copy")} />
                <MenuItem label="Paste" shortcut="Ctrl+V" onSelect={() => last.set("Paste")} />
              </>
            }
          >
            <Text class="text-xs text-text-secondary">Right-click here</Text>
          </ContextMenu>
        </Row>
        <Status text={`Chose: ${last()} · wrap ${wrap() ? "on" : "off"}`} />
      </Section>
      <Section title="Accordion" description="Sections open under their headings; the chevron turns.">
        <Accordion value={open()} multiple onChange={({ value }) => open.set(value)} class="w-96">
          <AccordionSection value="what" title="What is Omarchy?">
            <Body>An opinionated Arch and Hyprland setup.</Body>
          </AccordionSection>
          <AccordionSection value="themes" title="Can I change the theme?">
            <Body>Every theme recolors the whole desktop.</Body>
          </AccordionSection>
          <AccordionSection value="locked" title="Locked section" disabled>
            <Body>Hidden.</Body>
          </AccordionSection>
        </Accordion>
        <Status text={`Open: ${open().join(", ") || "none"}`} />
      </Section>
      <Section title="Pagination" description="Square cells; the current page carries the selected fill.">
        <Pagination accessibilityLabel="Results" page={page()} totalPages={12} onChange={({ page: p }) => page.set(p)} />
        <Status text={`Page ${page()} of 12`} />
      </Section>
      <Section title="Link">
        <Row>
          <Link label="omarchy.org" href="https://omarchy.org" />
          <Link label="Disabled link" href="https://omarchy.org" disabled />
        </Row>
      </Section>
    </View>
  );
}
