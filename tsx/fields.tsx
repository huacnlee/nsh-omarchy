// Omarchy fields on `nsh/base`: Input, Textarea, NumberInput, Select and
// Combobox. A field is a resting-fill well with the control border; while it
// edits (or its list is open) the border turns to the accent and the well
// takes the hover wash. Placeholders are secondary text, the caret is the
// accent. All controlled, and none rewrites what is typed, so an input
// method's composition is left alone.
import type { InputRef, JSX } from "nsh";
import { Text, View } from "nsh";
import {
  Combobox as BaseCombobox,
  Input as BaseInput,
  NumberInput as BaseNumberInput,
  Select as BaseSelect,
  type SelectItem,
} from "nsh/base";
import { Glyph } from "./icons";
import { cx, fill, height, ink, line, motion, type Size, surface, text } from "./theme";

export type { SelectItem };

const sizeText: Record<Size, string> = { sm: text.sm, md: text.md, lg: text.lg };

/** The well every field draws. */
const well = cx(
  "border border-foreground/40 bg-foreground/4",
  ink.body,
  "placeholder:text-text-secondary caret-omarchy-accent selection:bg-foreground/35",
  "hover:border-foreground/60",
  "focus:border-omarchy-accent focus:bg-foreground/8",
  motion.fast,
);

/** A field the user's checks rejected: the danger border. */
const invalidWell = "border-danger hover:border-danger";

interface FieldProps {
  /** Shown while empty. */
  placeholder?: string;
  /** What screen readers call the field when no visible label names it. */
  accessibilityLabel?: string;
  /** Selectable, not editable. */
  readonly?: boolean;
  /** Draws the danger border. */
  invalid?: boolean;
  testID?: string;
  class?: string;
}

export interface InputProps extends FieldProps {
  value: string;
  /** Every change: write it back to `value`. */
  onChange?: (event: { value: string }) => void;
  /** Return in the field. */
  onSubmit?: (event: { text: string }) => void;
  /** Hides what is typed (a password). */
  masked?: boolean;
  /** 24, 28 (default) or 32 points tall. */
  size?: Size;
  autoFocus?: boolean;
  /** Receives the field's ref, to focus it: `ref={(node) => (field = node)}`. */
  ref?: (node: InputRef) => void;
}

/** A one-line text field. */
export function Input(props: InputProps) {
  return (
    <BaseInput
      ref={props.ref}
      class={cx(height[props.size ?? "md"], "px-2", sizeText[props.size ?? "md"], well, props.invalid && invalidWell, props.class)}
      value={props.value}
      placeholder={props.placeholder}
      masked={props.masked}
      readonly={props.readonly}
      autoFocus={props.autoFocus}
      accessibilityLabel={props.accessibilityLabel}
      testID={props.testID}
      onChange={props.onChange}
      onSubmit={props.onSubmit}
    />
  );
}

export interface TextareaProps extends FieldProps {
  value: string;
  onChange?: (event: { value: string }) => void;
  /** Visible lines before it scrolls (default 4). */
  rows?: number;
}

/** A text field of several lines; Return types a new line. */
export function Textarea(props: TextareaProps) {
  return (
    <BaseInput
      multiline
      class={cx("px-2 py-1.5", text.md, well, props.invalid && invalidWell, props.class)}
      style={{ minHeight: 8 + 16 * (props.rows ?? 4) }}
      value={props.value}
      placeholder={props.placeholder}
      readonly={props.readonly}
      accessibilityLabel={props.accessibilityLabel}
      testID={props.testID}
      onChange={props.onChange}
    />
  );
}

/** The − / + buttons of a number field. */
const stepper = cx(
  "w-7 items-center justify-center",
  ink.body,
  fill.hover,
  fill.pressed,
  motion.fast,
  "disabled:bg-transparent disabled:text-foreground/38",
);

export interface NumberInputProps extends FieldProps {
  /** The number, or `null` for an empty field (controlled). */
  value: number | null;
  onChange?: (event: { value: number | null }) => void;
  min?: number;
  max?: number;
  /** Buttons, Up / Down (default 1); Page Up / Page Down go ten. */
  step?: number;
  disabled?: boolean;
  /** A unit after the number ("px", "%"). */
  suffix?: string;
}

/** A number with − / + steppers on either side. */
export function NumberInput(props: NumberInputProps) {
  return (
    <BaseNumberInput
      class={cx(
        "flex-row items-stretch self-start w-36 border border-foreground/40 bg-foreground/4",
        height.md,
        "hover:border-foreground/60 focus-within:border-omarchy-accent focus-within:bg-foreground/8",
        "disabled:border-foreground/12",
        props.invalid && invalidWell,
        motion.fast,
        props.class,
      )}
      value={props.value}
      min={props.min}
      max={props.max}
      step={props.step}
      disabled={props.disabled}
      readonly={props.readonly}
      placeholder={props.placeholder}
      accessibilityLabel={props.accessibilityLabel}
      testID={props.testID}
      onChange={props.onChange}
      inputClass={cx(
        "flex-1 min-w-0 px-1 text-center tabular-nums",
        text.md,
        ink.body,
        "placeholder:text-text-secondary caret-omarchy-accent disabled:text-foreground/38",
      )}
      decrementButtonClass={cx(stepper, "border-r border-foreground/12")}
      decrementButton={<Glyph name="minus" />}
      incrementButtonClass={cx(stepper, "border-l border-foreground/12")}
      incrementButton={<Glyph name="plus" />}
      suffix={props.suffix ? <Text class={cx("self-center pr-1", text.sm, ink.secondary)}>{props.suffix}</Text> : undefined}
    />
  );
}

/** A popup list's surface and entrance (selects, comboboxes). */
const listPopup = cx(surface.popup, "p-1", motion.base);

/** A row of a popup list: the hover / keyboard wash; chosen, the selected fill and bright text (no marker). */
const listItem = cx(
  "group h-7 flex-row items-center gap-2 px-2",
  text.md,
  ink.body,
  fill.highlight,
  fill.hover,
  "selected:bg-foreground/18 selected:text-bright",
  "disabled:text-foreground/38",
  motion.fast,
);

/** A group's heading in a popup list. */
const listGroup = cx("px-2 pt-2 pb-1", text.xs, ink.secondary);

/** The trigger of a select: the field's well, its border the accent while open. */
const triggerClass = cx(
  "flex-row items-center justify-between gap-2 px-2 border border-foreground/40 bg-foreground/4",
  ink.body,
  "hover:border-foreground/60",
  line.focus,
  fill.focus,
  "aria-expanded:border-omarchy-accent aria-expanded:bg-foreground/8",
  "disabled:border-foreground/12 disabled:text-foreground/38",
  motion.fast,
);

export interface SelectProps {
  items: SelectItem[];
  /** The chosen item's value, `null` for none (controlled). */
  value: string | null;
  /** The value asked for: write it back to `value`. */
  onChange?: (event: { value: string | null }) => void;
  /** What the trigger says while nothing is chosen. */
  placeholder?: string;
  /** What the choice is about, for screen readers. */
  accessibilityLabel: string;
  /** A search field above the items. */
  searchable?: boolean;
  disabled?: boolean;
  invalid?: boolean;
  size?: Size;
  testID?: string;
  class?: string;
}

/**
 * A trigger that opens a list: Up / Down / Home / End and typeahead move,
 * Return or Space choose, Escape and a press outside close, the keyboard
 * goes back to the trigger.
 */
export function Select(props: SelectProps) {
  const chosen = () => props.items.find((item) => item.value === props.value)?.title;
  return (
    <BaseSelect
      class={cx("self-start min-w-40", props.class)}
      items={props.items}
      value={props.value}
      placeholder={props.placeholder}
      searchable={props.searchable}
      disabled={props.disabled}
      accessibilityLabel={props.accessibilityLabel}
      testID={props.testID}
      onChange={props.onChange}
      cover={false}
      offset={2}
      popupClass={listPopup}
      popupStartingClass={motion.popupStarting}
      popupEndingClass={motion.popupEnding}
      listClass="max-h-72"
      itemClass={listItem}
      groupClass={listGroup}
      searchClass={cx("h-7 mb-1 px-2 border-b border-foreground/12", text.md, ink.body, "placeholder:text-text-secondary caret-omarchy-accent")}
      empty={<Text class={cx("px-2 py-1.5", text.md, ink.secondary)}>No matches</Text>}
      trigger={
        <View
          class={cx(
            triggerClass,
            height[props.size ?? "md"],
            sizeText[props.size ?? "md"],
            props.invalid && invalidWell,
            props.disabled && "border-foreground/12 bg-transparent text-foreground/38",
          )}
        >
          <Text class={props.disabled ? undefined : chosen() ? ink.body : ink.secondary} style={{ numberOfLines: 1 }}>
            {chosen() ?? props.placeholder ?? ""}
          </Text>
          <Glyph name="chevron-down" class={props.disabled ? undefined : ink.secondary} />
        </View>
      }
    />
  );
}

export interface ComboboxProps {
  items: SelectItem[];
  /** The selected values (controlled). */
  value: string[];
  onChange?: (event: { value: string[] }) => void;
  /** Several values: a choice toggles one and the list stays open. */
  multiple?: boolean;
  placeholder?: string;
  accessibilityLabel: string;
  disabled?: boolean;
  testID?: string;
  /** Finds the text field in tests. */
  inputTestID?: string;
  class?: string;
}

/** A field whose typing filters the list. */
export function Combobox(props: ComboboxProps) {
  return (
    <BaseCombobox
      class={cx(
        "flex-row items-center self-start min-w-56 border border-foreground/40 bg-foreground/4",
        height.md,
        "hover:border-foreground/60 focus-within:border-omarchy-accent focus-within:bg-foreground/8",
        "aria-expanded:border-omarchy-accent disabled:border-foreground/12",
        motion.fast,
        props.class,
      )}
      items={props.items}
      value={props.value}
      multiple={props.multiple}
      placeholder={props.placeholder}
      disabled={props.disabled}
      accessibilityLabel={props.accessibilityLabel}
      testID={props.testID}
      inputTestID={props.inputTestID}
      onChange={props.onChange}
      offset={2}
      popupClass={listPopup}
      popupStartingClass={motion.popupStarting}
      popupEndingClass={motion.popupEnding}
      listClass="max-h-72"
      itemClass={listItem}
      groupClass={listGroup}
      inputClass={cx("flex-1 min-w-0 px-2", text.md, ink.body, "placeholder:text-text-secondary caret-omarchy-accent disabled:text-foreground/38")}
      triggerButton={
        <View class={cx("w-7 self-stretch items-center justify-center", ink.secondary, fill.hover, fill.pressed, motion.fast)}>
          <Glyph name="chevron-down" />
        </View>
      }
      empty={<Text class={cx("px-2 py-1.5", text.md, ink.secondary)}>No matches</Text>}
      renderItem={(entry) => (
        <View class="flex-1 flex-row items-center justify-between">
          <Text>{entry.title}</Text>
          <Glyph name="check" class="opacity-0 group-selected:opacity-100" />
        </View>
      )}
    />
  );
}

/** A field's label above it and an optional hint or error below. */
export function Field(props: { label: string; hint?: string; error?: string; children: JSX.Element; class?: string }) {
  return (
    <View class={cx("gap-1", props.class)}>
      <Text class={cx(text.sm, ink.secondary)}>{props.label}</Text>
      {props.children}
      <If when={props.error}>
        <Text class={cx(text.sm, ink.danger)}>{props.error}</Text>
      </If>
      <If when={!props.error && props.hint}>
        <Text class={cx(text.sm, ink.secondary)}>{props.hint}</Text>
      </If>
    </View>
  );
}
