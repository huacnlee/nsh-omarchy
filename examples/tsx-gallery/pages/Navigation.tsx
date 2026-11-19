// Tabs, the sidebar, menus, accordions, collapsibles and pagination.
import { signal, Text, View } from "nsh";
import {
  Accordion,
  AccordionItem,
  Body,
  Button,
  Collapsible,
  ContextMenu,
  Menu,
  MenuItem,
  MenuLabel,
  MenuSeparator,
  Pagination,
  Sidebar,
  SidebarItem,
  SidebarSection,
  Tab,
  Tabs,
} from "nsh-omarchy";
import { Demos, type Family, Row, Section, Status } from "./Section";

function TabsPage() {
  const tab = signal("general");
  const view = signal("list");
  return (
    <Demos>
      <Section title="Underline" description="The arrows move and select; the indicator slides.">
        <Tabs accessibilityLabel="Settings" value={tab()} onChange={({ value }) => tab.set(value)}>
          <Tab value="general" label="General" />
          <Tab value="keys" label="Keybindings" icon="terminal" />
          <Tab value="display" label="Display" icon="sun" />
          <Tab value="beta" label="Beta" disabled />
        </Tabs>
        <Body>{`Panel: ${tab()}`}</Body>
      </Section>
      <Section title="Segmented" description="A compact switch between views; the selected fill slides.">
        <Tabs variant="segmented" size="small" accessibilityLabel="View" value={view()} onChange={({ value }) => view.set(value)}>
          <Tab value="list" label="List" />
          <Tab value="grid" label="Grid" />
          <Tab value="board" label="Board" />
        </Tabs>
        <Status text={`View: ${view()}`} />
      </Section>
    </Demos>
  );
}

function SidebarPage() {
  const mailbox = signal("inbox");
  return (
    <Demos>
      <Section title="Sidebar" description="Sections of items; the current one has the selected fill. Up and Down move.">
        <Row>
          <Sidebar
            class="h-[240px] border border-border"
            accessibilityLabel="Mailboxes"
            value={mailbox()}
            onChange={({ value }) => mailbox.set(value)}
            header={<Text class="px-3 font-semibold text-bright">Mail</Text>}
          >
            <SidebarSection title="Mailboxes">
              <SidebarItem value="inbox" label="Inbox" icon="inbox" suffix={<Text class="text-[11px] text-text-secondary">5</Text>} />
              <SidebarItem value="starred" label="Starred" icon="star" />
              <SidebarItem value="sent" label="Sent" icon="upload" />
            </SidebarSection>
            <SidebarSection title="Labels">
              <SidebarItem value="work" label="Work" />
              <SidebarItem value="archive" label="Archive" disabled />
            </SidebarSection>
          </Sidebar>
          <Status text={`Mailbox: ${mailbox()}`} />
        </Row>
      </Section>
    </Demos>
  );
}

function MenuPage() {
  const last = signal("nothing");
  const wrap = signal(true);
  return (
    <Demos>
      <Section title="Menu" description="Rows with shortcuts in monospace, a checkable row, a destructive row.">
        <Row>
          <Menu trigger={<Button label="File" icon="file" />}>
            <MenuLabel label="Document" />
            <MenuItem label="New" shortcut="mod-n" onSelect={() => last.set("New")} />
            <MenuItem label="Open…" shortcut="mod-o" onSelect={() => last.set("Open")} />
            <MenuItem label="Word wrap" checked={wrap()} onSelect={() => wrap.update((v) => !v)} />
            <MenuItem label="Print" disabled />
            <MenuSeparator />
            <MenuItem label="Delete" icon="trash" danger onSelect={() => last.set("Delete")} />
          </Menu>
        </Row>
        <Status text={`Chose: ${last()} · wrap ${wrap() ? "on" : "off"}`} />
      </Section>
    </Demos>
  );
}

function ContextMenuPage() {
  const last = signal("nothing");
  return (
    <Demos>
      <Section title="ContextMenu" description="The same rows, opened by a right click at the pointer.">
        <ContextMenu
          menu={
            <>
              <MenuItem label="Copy" icon="copy" shortcut="mod-c" onSelect={() => last.set("Copy")} />
              <MenuItem label="Paste" shortcut="mod-v" onSelect={() => last.set("Paste")} />
            </>
          }
        >
          <View class="h-16 w-64 items-center justify-center border border-dashed border-foreground/40">
            <Text class="text-xs text-text-secondary">Right-click here</Text>
          </View>
        </ContextMenu>
        <Status text={`Chose: ${last()}`} />
      </Section>
    </Demos>
  );
}

function AccordionPage() {
  const open = signal<string[]>(["what"]);
  return (
    <Demos>
      <Section title="Accordion" description="Items open under their headings; the chevron turns.">
        <Accordion value={open()} multiple onChange={({ value }) => open.set(value)} class="w-96">
          <AccordionItem value="what" title="What is Omarchy?">
            <Body>An opinionated Arch and Hyprland setup.</Body>
          </AccordionItem>
          <AccordionItem value="themes" title="Can I change the theme?">
            <Body>Every theme recolors the whole desktop.</Body>
          </AccordionItem>
          <AccordionItem value="locked" title="Locked item" disabled>
            <Body>Hidden.</Body>
          </AccordionItem>
        </Accordion>
        <Status text={`Open: ${open().join(", ") || "none"}`} />
      </Section>
    </Demos>
  );
}

function CollapsiblePage() {
  const advanced = signal(false);
  return (
    <Demos>
      <Section title="Collapsible" description="One region under a trigger row, controlled or not.">
        <Collapsible class="w-96" title="Advanced options" open={advanced()} onOpenChange={({ open }) => advanced.set(open)}>
          <Body>Compression: zstd, level 3</Body>
        </Collapsible>
        <Collapsible class="w-96" title="Release notes">
          <Body>0.1: the first release.</Body>
        </Collapsible>
        <Status text={`Advanced: ${advanced() ? "open" : "closed"}`} />
      </Section>
    </Demos>
  );
}

function PaginationPage() {
  const page = signal(1);
  return (
    <Demos>
      <Section title="Pagination" description="Square cells; the current page carries the selected fill.">
        <Pagination accessibilityLabel="Results" page={page()} totalPages={12} onChange={({ page: p }) => page.set(p)} />
        <Status text={`Page ${page()} of 12`} />
      </Section>
    </Demos>
  );
}

export const navigation: Family = {
  title: "Navigation",
  pages: [
    { value: "tabs", title: "Tabs", page: TabsPage },
    { value: "sidebar", title: "Sidebar", page: SidebarPage },
    { value: "menu", title: "Menu", page: MenuPage },
    { value: "context-menu", title: "ContextMenu", page: ContextMenuPage },
    { value: "accordion", title: "Accordion", page: AccordionPage },
    { value: "collapsible", title: "Collapsible", page: CollapsiblePage },
    { value: "pagination", title: "Pagination", page: PaginationPage },
  ],
};
