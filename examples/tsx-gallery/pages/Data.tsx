// Data: a table, a data table, a virtual list with a scrollbar, a tree,
// resizable panels.
import type { JSX, ListRef } from "nsh";
import { List, signal, Text, View } from "nsh";
import {
  DataTable,
  ListItem,
  ResizablePanel,
  ResizablePanelGroup,
  Scrollbar,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  Tree,
} from "nsh-omarchy";
import type { TableSort, TreeItem } from "nsh-omarchy";
import { Demos, type Family, Section, Status } from "./Section";

const NAMES = ["hyprland", "waybar", "alacritty", "nvim", "firefox", "mako", "pipewire", "walker"];

type Process = { id: string; pid: number; name: string; user: string; cpu: number };

function processes(n: number): Process[] {
  return Array.from({ length: n }, (_, i) => ({
    id: String(1200 + i * 7),
    pid: 1200 + i * 7,
    name: NAMES[i % NAMES.length],
    user: i % 3 === 0 ? "root" : "omarchy",
    cpu: ((i * 37) % 1000) / 10,
  }));
}

function StaticTable() {
  const few = processes(4);
  const selected = signal("1207");
  return (
    <View class="gap-2">
      <Table accessibilityLabel="Processes">
        <TableHeader>
          <TableRow>
            <TableHead title="PID" width={64} numeric />
            <TableHead title="Name" />
            <TableHead title="User" width={96} />
            <TableHead title="CPU %" width={72} numeric />
          </TableRow>
        </TableHeader>
        <TableBody>
          <Each of={few} key={(p) => p.id}>
            {(p) => (
              <TableRow
                accessibilityLabel={`${p().name} ${p().pid}`}
                selected={selected() === p().id}
                onPress={() => selected.set(p().id)}
              >
                <TableCell text={String(p().pid)} width={64} numeric />
                <TableCell text={p().name} />
                <TableCell text={p().user} width={96} />
                <TableCell text={p().cpu.toFixed(1)} width={72} numeric />
              </TableRow>
            )}
          </Each>
        </TableBody>
      </Table>
      <Status text={`Selected pid ${selected()}`} />
    </View>
  );
}

function BigTable() {
  const all = processes(300);
  const sort = signal<TableSort | null>(null);
  const selected = signal<string | null>(null);
  const rows = () => {
    const by = sort();
    if (!by) return all;
    const key = by.key as keyof Process;
    const sorted = [...all].sort((a, b) => (a[key] < b[key] ? -1 : a[key] > b[key] ? 1 : 0));
    return by.direction === "descending" ? sorted.reverse() : sorted;
  };
  return (
    <View class="gap-2">
      <DataTable
        class="h-[280px]"
        accessibilityLabel="All processes"
        columns={[
          { key: "pid", title: "PID", width: 80, numeric: true, sortable: true },
          { key: "name", title: "Name", width: 160, sortable: true },
          { key: "user", title: "User", width: 120, sortable: true },
          { key: "cpu", title: "CPU %", width: 96, numeric: true, sortable: true },
        ]}
        rows={rows()}
        sort={sort()}
        selected={selected()}
        onSortChange={({ sort: next }) => sort.set(next)}
        onSelectedChange={({ selected: id }) => selected.set(id)}
      />
      <Status
        text={`Sort ${sort() ? `${sort()!.key} ${sort()!.direction}` : "none"} · selected ${selected() ?? "none"}`}
      />
    </View>
  );
}

function VirtualList() {
  const items = Array.from({ length: 2000 }, (_, i) => ({ id: i + 1, name: NAMES[i % NAMES.length] }));
  const selected = signal(0);
  const target = signal<ListRef | undefined>(undefined);
  return (
    <View class="gap-2">
      <View class="relative h-[280px] w-[420px] border border-border bg-background">
        <List
          class="size-full"
          testID="list"
          each={items}
          keyExtractor={(item) => String(item.id)}
          ref={(node) => target.set(node)}
        >
          {(item) => (
            <ListItem
              accessibilityLabel={`Item ${item().id}`}
              selected={selected() === item().id}
              disabled={item().id === 2}
              onPress={() => selected.set(item().id)}
            >
              <Text class="font-mono text-text-secondary">{String(item().id).padStart(4, "0")}</Text>
              <Text class="flex-1">{`Item ${item().id}`}</Text>
              <Text class="text-[11px] text-text-secondary">{item().name}</Text>
            </ListItem>
          )}
        </List>
        {target() ? <Scrollbar class="absolute top-0 right-0 bottom-0" target={target()!} /> : null}
      </View>
      <Status text={selected() ? `Selected item ${selected()}` : "Selected none"} />
    </View>
  );
}

const FILES: TreeItem[] = [
  {
    id: "config",
    label: "config",
    icon: "folder",
    children: [
      {
        id: "hypr",
        label: "hypr",
        icon: "folder",
        children: [
          { id: "hyprland.conf", label: "hyprland.conf", icon: "file" },
          { id: "bindings.conf", label: "bindings.conf", icon: "file" },
        ],
      },
      { id: "alacritty.toml", label: "alacritty.toml", icon: "file" },
    ],
  },
  {
    id: "themes",
    label: "themes",
    icon: "folder",
    children: [
      { id: "tokyo-night", label: "tokyo-night", icon: "folder" },
      { id: "gruvbox", label: "gruvbox", icon: "folder", disabled: true },
    ],
  },
  { id: "README.md", label: "README.md", icon: "file" },
];

function FileTree() {
  const expanded = signal<string[]>(["config"]);
  const selected = signal<string | null>("hypr");
  return (
    <View class="gap-2">
      <View class="h-[260px] w-[320px] border border-border bg-background">
        <Tree
          class="size-full"
          accessibilityLabel="Files"
          items={FILES}
          expanded={expanded()}
          selected={selected()}
          onExpandedChange={({ expanded: next }) => expanded.set(next)}
          onSelectedChange={({ selected: id }) => selected.set(id)}
        />
      </View>
      <Status text={`Selected ${selected() ?? "none"} · expanded ${expanded().join(" ")}`} />
    </View>
  );
}

function Pane(props: { title: string }) {
  return (
    <View class="size-full gap-1 bg-surface p-3">
      <Text class="font-semibold text-bright">{props.title}</Text>
      <Text class="text-[11px] text-text-secondary">Drag the line beside me.</Text>
    </View>
  );
}

function Panels() {
  const sizes = signal<number[]>([]);
  return (
    <View class="gap-2">
      <View class="h-[280px] w-full border border-border">
        <ResizablePanelGroup onResize={({ sizes: next }) => sizes.set(next)}>
          <ResizablePanel size={200}>
            <Pane title="Sidebar" />
          </ResizablePanel>
          <ResizablePanel>
            <ResizablePanelGroup layout="vertical">
              <ResizablePanel>
                <Pane title="Editor" />
              </ResizablePanel>
              <ResizablePanel size={100}>
                <Pane title="Terminal" />
              </ResizablePanel>
            </ResizablePanelGroup>
          </ResizablePanel>
        </ResizablePanelGroup>
      </View>
      <Status
        text={sizes().length ? `Sizes ${sizes().map((s) => s.toFixed(0)).join(" ")}` : "Sizes as laid out"}
      />
    </View>
  );
}

/** A page of one demo under one section. */
function page(title: string, description: string, Demo: () => JSX.Element) {
  return () => (
    <Demos>
      <Section title={title} description={description}>
        <Demo />
      </Section>
    </Demos>
  );
}

export const data: Family = {
  title: "Data",
  pages: [
    { value: "table", title: "Table", page: page("Table", "Rows of cells; a press selects a row.", StaticTable) },
    {
      value: "data-table",
      title: "DataTable",
      page: page("300 rows, sortable", "Virtualized rows from columns and data; the headers sort.", BigTable),
    },
    {
      value: "list",
      title: "ListItem",
      page: page("2000 rows, with a Scrollbar", "ListItem rows in a virtual List; a Scrollbar beside it.", VirtualList),
    },
    { value: "tree", title: "Tree", page: page("Tree", "Folders fold with Left and Right; Up and Down move.", FileTree) },
    {
      value: "resizable",
      title: "ResizablePanelGroup",
      page: page("Resizable panels", "Drag a divider, or focus it and use the arrows.", Panels),
    },
  ],
};
