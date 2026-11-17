// Text fields, number fields, selects and comboboxes.
import { signal, View } from "nsh";
import { Combobox, Field, Input, NumberInput, Select, type SelectItem, Textarea } from "../../../tsx";
import { Section, Status } from "./Section";

const themes: SelectItem[] = [
  { value: "tokyo-night", title: "Tokyo Night", group: "Dark" },
  { value: "catppuccin", title: "Catppuccin", group: "Dark" },
  { value: "gruvbox", title: "Gruvbox", group: "Dark" },
  { value: "matte-black", title: "Matte Black", group: "Dark", disabled: true },
  { value: "flexoki-light", title: "Flexoki Light", group: "Light" },
  { value: "rose-pine-dawn", title: "Rosé Pine Dawn", group: "Light" },
];

const apps: SelectItem[] = [
  { value: "alacritty", title: "Alacritty" },
  { value: "chromium", title: "Chromium" },
  { value: "neovim", title: "Neovim" },
  { value: "obsidian", title: "Obsidian" },
  { value: "spotify", title: "Spotify" },
  { value: "typora", title: "Typora" },
];

export function Fields() {
  const name = signal("");
  const submitted = signal("");
  const notes = signal("");
  const gaps = signal<number | null>(10);
  const theme = signal<string | null>("tokyo-night");
  const pinned = signal<string[]>(["neovim"]);
  return (
    <View class="gap-6">
      <Section title="Input" description="A well with the control border; the accent border while editing.">
        <Field label="Hostname" hint="Return submits." class="w-80">
          <Input
            accessibilityLabel="Hostname"
            placeholder="omarchy"
            value={name()}
            onChange={({ value }) => name.set(value)}
            onSubmit={({ text }) => submitted.set(text)}
          />
        </Field>
        <Field label="Password" error="At least 8 characters." class="w-80">
          <Input accessibilityLabel="Password" masked invalid value="hunter2" />
        </Field>
        <Status text={`Submitted: ${submitted() || "nothing"}`} />
      </Section>
      <Section title="Textarea" description="Several lines; Return types a new line.">
        <Field label="Notes" class="w-96">
          <Textarea accessibilityLabel="Notes" placeholder="Write something…" value={notes()} onChange={({ value }) => notes.set(value)} />
        </Field>
      </Section>
      <Section title="NumberInput" description="Steppers, Up / Down, Page Up / Page Down.">
        <Field label="Window gaps">
          <NumberInput
            accessibilityLabel="Window gaps"
            value={gaps()}
            min={0}
            max={40}
            suffix="px"
            onChange={({ value }) => gaps.set(value)}
          />
        </Field>
        <Status text={`Gaps: ${gaps() ?? "empty"}`} />
      </Section>
      <Section title="Select" description="Typeahead, groups, a chosen row with the selected fill.">
        <Field label="Theme">
          <Select accessibilityLabel="Theme" items={themes} value={theme()} onChange={({ value }) => theme.set(value)} />
        </Field>
        <Field label="Disabled">
          <Select accessibilityLabel="Font" items={themes} value={null} placeholder="Choose a font" disabled />
        </Field>
        <Status text={`Theme: ${theme() ?? "none"}`} />
      </Section>
      <Section title="Combobox" description="Typing filters; several values stay chosen.">
        <Field label="Pinned apps">
          <Combobox
            accessibilityLabel="Pinned apps"
            inputTestID="apps-input"
            items={apps}
            value={pinned()}
            multiple
            placeholder="Search apps"
            onChange={({ value }) => pinned.set(value)}
          />
        </Field>
        <Status text={`Pinned: ${pinned().join(", ") || "none"}`} />
      </Section>
    </View>
  );
}
