// Text fields, number fields, codes, selects, comboboxes, days and colors.
import { signal, View } from "nsh";
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
import { Demos, type Family, Section, Status } from "./Section";

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

function InputPage() {
  const name = signal("");
  const submitted = signal("");
  const password = signal("hunter2");
  const short = () => password().length < 8;
  return (
    <Demos>
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
        </View>
        <Status text={`Submitted: ${submitted() || "nothing"}`} />
      </Section>
      <Section title="Field" description="A label, a description, and an error in its place.">
        <View class="w-80 gap-3">
          <Field label="Password" required description="At least 8 characters." error={short() ? "Too short." : null}>
            <Input masked invalid={short()} value={password()} onChange={({ value }) => password.set(value)} />
          </Field>
        </View>
      </Section>
    </Demos>
  );
}

function TextareaPage() {
  const notes = signal("");
  return (
    <Demos>
      <Section title="Textarea" description="Several lines; Return types a new line.">
        <View class="w-96">
          <Field label="Notes">
            <Textarea rows={3} placeholder="Write something…" value={notes()} onChange={({ value }) => notes.set(value)} />
          </Field>
        </View>
      </Section>
    </Demos>
  );
}

function NumberInputPage() {
  const gaps = signal<number | null>(10);
  return (
    <Demos>
      <Section title="NumberInput" description="Steppers, Up / Down, Page Up / Page Down.">
        <Field label="Window gaps">
          <NumberInput value={gaps()} min={0} max={40} suffix="px" onChange={({ value }) => gaps.set(value)} />
        </Field>
        <Status text={`Gaps: ${gaps() ?? "empty"}`} />
      </Section>
    </Demos>
  );
}

function OtpInputPage() {
  const code = signal("");
  const verified = signal("");
  return (
    <Demos>
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
    </Demos>
  );
}

function SelectPage() {
  const theme = signal<string | null>("tokyo-night");
  return (
    <Demos>
      <Section title="Select" description="Typeahead, groups, a chosen row with the selected fill.">
        <Field label="Theme">
          <Select items={themes} value={theme()} onChange={({ value }) => theme.set(value)} />
        </Field>
        <Status text={`Theme: ${theme() ?? "none"}`} />
      </Section>
      <Section title="Small and disabled">
        <Field label="Font">
          <Select items={themes} value={null} placeholder="Choose a font" size="small" disabled />
        </Field>
      </Section>
    </Demos>
  );
}

function ComboboxPage() {
  const pinned = signal<string[]>(["neovim"]);
  return (
    <Demos>
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
    </Demos>
  );
}

function CalendarPage() {
  const day = signal<string | null>(null);
  const trip = signal<DateRange | null>(null);
  return (
    <Demos>
      <Section title="Day" description="Arrows move a day or a week, Page Up / Page Down a month.">
        <Calendar value={day()} onChange={({ value }) => day.set(value)} />
        <Status text={`Day: ${day() ?? "none"}`} />
      </Section>
      <Section title="Range" description="The first press starts the range, the second ends it.">
        <Calendar range rangeValue={trip()} onRangeChange={({ value }) => trip.set(value)} />
        <Status text={`Trip: ${trip() ? `${trip()!.start} to ${trip()!.end}` : "none"}`} />
      </Section>
    </Demos>
  );
}

function DatePickerPage() {
  const due = signal<string | null>("2026-11-19");
  const trip = signal<DateRange | null>(null);
  return (
    <Demos>
      <Section title="DatePicker" description="A trigger that opens a calendar; a pick or Escape closes it.">
        <Field label="Due date">
          <DatePicker value={due()} onChange={({ value }) => due.set(value)} />
        </Field>
        <Status text={`Due: ${due() ?? "none"}`} />
      </Section>
      <Section title="Range">
        <Field label="Trip dates">
          <DatePicker range rangeValue={trip()} onRangeChange={({ value }) => trip.set(value)} />
        </Field>
      </Section>
    </Demos>
  );
}

function ColorPickerPage() {
  const accent = signal("#7aa2f7");
  return (
    <Demos>
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
        <Status text={`Accent: ${accent()}`} />
      </Section>
    </Demos>
  );
}

export const fields: Family = {
  title: "Fields",
  pages: [
    { value: "input", title: "Input", page: InputPage },
    { value: "textarea", title: "Textarea", page: TextareaPage },
    { value: "number-input", title: "NumberInput", page: NumberInputPage },
    { value: "otp-input", title: "OtpInput", page: OtpInputPage },
    { value: "select", title: "Select", page: SelectPage },
    { value: "combobox", title: "Combobox", page: ComboboxPage },
    { value: "calendar", title: "Calendar", page: CalendarPage },
    { value: "date-picker", title: "DatePicker", page: DatePickerPage },
    { value: "color-picker", title: "ColorPicker", page: ColorPickerPage },
  ],
};
