// Text fields, number fields, codes, selects, comboboxes, days and colors.
import { signal, Text, View } from "nsh";
import {
  Calendar,
  ColorPicker,
  Combobox,
  DatePicker,
  type DateRange,
  Field,
  Input,
  NumberInput,
  OtpInput,
  Select,
  type SelectItem,
  Textarea,
} from "nsh-omarchy";
import { Section, Status } from "./Section";

const themes: SelectItem[] = [
  { value: "tokyo-night", label: "Tokyo Night", group: "Dark" },
  { value: "catppuccin", label: "Catppuccin", group: "Dark" },
  { value: "gruvbox", label: "Gruvbox", group: "Dark" },
  { value: "matte-black", label: "Matte Black", group: "Dark", disabled: true },
  { value: "flexoki-light", label: "Flexoki Light", group: "Light" },
  { value: "rose-pine-dawn", label: "Rosé Pine Dawn", group: "Light" },
];

const apps: SelectItem[] = [
  { value: "alacritty", label: "Alacritty" },
  { value: "chromium", label: "Chromium" },
  { value: "neovim", label: "Neovim" },
  { value: "obsidian", label: "Obsidian" },
  { value: "spotify", label: "Spotify" },
  { value: "typora", label: "Typora" },
];

const swatches = ["#f7768e", "#ff9e64", "#e0af68", "#9ece6a", "#7dcfff", "#7aa2f7", "#bb9af7"];

export function Fields() {
  const name = signal("");
  const submitted = signal("");
  const password = signal("hunter2");
  const notes = signal("");
  const gaps = signal<number | null>(10);
  const code = signal("");
  const verified = signal("");
  const theme = signal<string | null>("tokyo-night");
  const pinned = signal<string[]>(["neovim"]);
  const day = signal<string | null>(null);
  const trip = signal<DateRange | null>(null);
  const due = signal<string | null>("2026-11-19");
  const accent = signal("#7aa2f7");
  const short = () => password().length < 8;
  return (
    <View class="gap-6">
      <Section title="Input" description="A well with the control border; the accent border while editing.">
        <View class="w-80 gap-3">
          <Field label="Hostname" description="Return submits.">
            <Input
              placeholder="omarchy"
              value={name()}
              onChange={({ value }) => name.set(value)}
              onSubmit={({ value }) => submitted.set(value)}
            />
          </Field>
          <Field label="Password" required description="At least 8 characters." error={short() ? "Too short." : null}>
            <Input masked invalid={short()} value={password()} onChange={({ value }) => password.set(value)} />
          </Field>
        </View>
        <Status text={`Submitted: ${submitted() || "nothing"}`} />
      </Section>
      <Section title="Textarea" description="Several lines; Return types a new line.">
        <View class="w-96">
          <Field label="Notes">
            <Textarea rows={3} placeholder="Write something…" value={notes()} onChange={({ value }) => notes.set(value)} />
          </Field>
        </View>
      </Section>
      <Section title="NumberInput" description="Steppers, Up / Down, Page Up / Page Down.">
        <Field label="Window gaps">
          <NumberInput value={gaps()} min={0} max={40} suffix="px" onChange={({ value }) => gaps.set(value)} />
        </Field>
        <Status text={`Gaps: ${gaps() ?? "empty"}`} />
      </Section>
      <Section title="OtpInput" description="Digits only; paste or autofill fills every cell.">
        <Field label="Verification code">
          <OtpInput
            length={6}
            groups={2}
            value={code()}
            onChange={({ value }) => code.set(value)}
            onComplete={({ value }) => verified.set(value)}
          />
        </Field>
        <Status text={`Verified: ${verified() || "nothing"}`} />
      </Section>
      <Section title="Select" description="Typeahead, groups, a chosen row with the selected fill.">
        <Field label="Theme">
          <Select items={themes} value={theme()} onChange={({ value }) => theme.set(value)} />
        </Field>
        <Field label="Font">
          <Select items={themes} value={null} placeholder="Choose a font" size="small" disabled />
        </Field>
        <Status text={`Theme: ${theme() ?? "none"}`} />
      </Section>
      <Section title="Combobox" description="Typing filters; several values stay chosen.">
        <Field label="Pinned apps">
          <Combobox
            items={apps}
            value={pinned()}
            multiple
            placeholder="Search apps"
            onChange={({ value }) => pinned.set(value)}
          />
        </Field>
        <Status text={`Pinned: ${pinned().join(", ") || "none"}`} />
      </Section>
      <Section title="Calendar" description="Arrows move a day or a week, Page Up / Page Down a month.">
        <View class="flex-row flex-wrap gap-6">
          <View class="gap-2">
            <Calendar value={day()} onChange={({ value }) => day.set(value)} />
            <Status text={`Day: ${day() ?? "none"}`} />
          </View>
          <View class="gap-2">
            <Calendar range rangeValue={trip()} onRangeChange={({ value }) => trip.set(value)} />
            <Status text={`Trip: ${trip() ? `${trip()!.start} to ${trip()!.end}` : "none"}`} />
          </View>
        </View>
      </Section>
      <Section title="DatePicker" description="A trigger that opens a calendar; a pick or Escape closes it.">
        <Field label="Due date">
          <DatePicker value={due()} onChange={({ value }) => due.set(value)} />
        </Field>
        <Field label="Trip dates">
          <DatePicker range rangeValue={trip()} onRangeChange={({ value }) => trip.set(value)} />
        </Field>
        <Status text={`Due: ${due() ?? "none"}`} />
      </Section>
      <Section title="ColorPicker" description="Drag the area or the sliders, type a CSS color, or press a swatch.">
        <View class="flex-row items-start gap-4">
          <ColorPicker
            accessibilityLabel="Accent"
            value={accent()}
            alpha
            swatches={swatches}
            onChange={({ value }) => accent.set(value)}
          />
          <View class="h-8 w-8 border border-border" style={{ backgroundColor: accent() }} />
        </View>
        <Text class="text-[11px] text-text-secondary">{`Accent: ${accent()}`}</Text>
      </Section>
    </View>
  );
}
