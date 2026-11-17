// Omarchy choices on `nsh/base`: Checkbox, Switch, RadioGroup / Radio, Slider
// and Progress. All controlled: the value in, the value asked for out.
//
// A labelled control is one row: hover and the keyboard focus wash it and
// outline it in the accent, a press darkens the wash. Checked fills the box
// or track with the selected fill and drops its border; disabled dims text,
// box and mark and takes no interaction styles.
import type { JSX } from "nsh";
import { context, Text, View } from "nsh";
import {
  Checkbox as BaseCheckbox,
  Progress as BaseProgress,
  Radio as BaseRadio,
  RadioGroup as BaseRadioGroup,
  Slider as BaseSlider,
  type SliderValue,
  Switch as BaseSwitch,
} from "nsh/base";
import { Glyph } from "./icons";
import { cx, fill, ink, line, motion, text } from "./theme";

/** The row a labelled choice draws: label beside its box, hugging both. */
const choiceRow = cx(
  "group flex-row self-start items-center gap-2 min-h-7 px-1 border border-transparent",
  text.md,
  ink.body,
  fill.hover,
  "hover:border-omarchy-accent",
  fill.focus,
  line.focus,
  fill.pressed,
  motion.fast,
  "disabled:bg-transparent disabled:border-transparent disabled:text-foreground/38",
);

/** A box or track: the control border on the resting fill, filled and borderless while checked. */
const box = cx(
  "shrink-0 items-center justify-center border border-foreground/40 bg-foreground/4",
  "checked:border-transparent checked:bg-foreground/18",
  "disabled:border-foreground/12 disabled:bg-foreground/4",
  motion.fast,
);

interface ChoiceProps {
  /** The visible label, which screen readers say. */
  label?: string;
  /** What screen readers say when there is no label. */
  accessibilityLabel?: string;
  disabled?: boolean;
  testID?: string;
  /** Classes appended to the row. */
  class?: string;
}

export interface CheckboxProps extends ChoiceProps {
  checked: boolean;
  /** Neither checked nor unchecked: a dash, and a press asks for checked. */
  indeterminate?: boolean;
  /** The value asked for: write it back to `checked`. */
  onChange?: (event: { checked: boolean }) => void;
}

/** A square box with a drawn check (a dash while indeterminate), and its label. */
export function Checkbox(props: CheckboxProps) {
  return (
    <BaseCheckbox
      class={cx(choiceRow, props.class)}
      checked={props.checked}
      indeterminate={props.indeterminate}
      disabled={props.disabled}
      accessibilityLabel={props.accessibilityLabel}
      testID={props.testID}
      onChange={props.onChange}
      indicatorClass={cx(box, "size-4 indeterminate:border-transparent indeterminate:bg-foreground/18")}
      indicator={
        <View class="size-3.5 items-center justify-center text-bright group-disabled:text-foreground/38">
          <Glyph name="check" class={cx("absolute opacity-0 group-checked:opacity-100 group-indeterminate:opacity-0", motion.fast)} />
          <Glyph name="dash" class={cx("absolute opacity-0 group-indeterminate:opacity-100", motion.fast)} />
        </View>
      }
    >
      <If when={props.label}>
        <Text>{props.label}</Text>
      </If>
    </BaseCheckbox>
  );
}

export interface SwitchProps extends ChoiceProps {
  checked: boolean;
  /** The value asked for: write it back to `checked`. */
  onChange?: (event: { checked: boolean }) => void;
}

/**
 * A square track whose square thumb slides from the start (off) to the end
 * (on) over 140 ms; on fills the track and brightens the thumb.
 */
export function Switch(props: SwitchProps) {
  return (
    <BaseSwitch
      class={cx(choiceRow, props.class)}
      checked={props.checked}
      disabled={props.disabled}
      accessibilityLabel={props.accessibilityLabel}
      testID={props.testID}
      onChange={props.onChange}
      trackClass={cx(box, "w-10.5 h-5.5 items-start p-0.5")}
      thumbClass={cx(
        "size-4 bg-text-secondary checked:bg-bright checked:translate-x-5 disabled:bg-foreground/38",
        motion.base,
      )}
    >
      <If when={props.label}>
        <Text>{props.label}</Text>
      </If>
    </BaseSwitch>
  );
}

/** What a `RadioGroup` gives its radios. */
interface RadioContext {
  value: () => string | null | undefined;
  choose: (value: string) => void;
  disabled: () => boolean;
}

const Radios = context<RadioContext | null>(null);

export interface RadioGroupProps {
  /** The checked radio's value (controlled), or none. */
  value: string | null;
  /** The value asked for: write it back to `value`. */
  onChange?: (event: { value: string }) => void;
  /** What screen readers call the set ("Plan"). */
  accessibilityLabel?: string;
  disabled?: boolean;
  /** Lays the radios out in a row. */
  horizontal?: boolean;
  /** The `Radio`s. */
  children?: JSX.Element;
  class?: string;
}

/**
 * A set of radios, one stop of Tab at the checked one: the arrows move around
 * the set and check the radio they land on, Space checks the focused one.
 */
export function RadioGroup(props: RadioGroupProps) {
  const ctx: RadioContext = {
    value: () => props.value,
    choose: (value) => props.onChange?.({ value }),
    disabled: () => props.disabled ?? false,
  };
  return (
    <Radios.Provider value={ctx}>
      <BaseRadioGroup
        class={cx(props.horizontal ? "flex-row flex-wrap gap-3" : "gap-1", props.class)}
        accessibilityLabel={props.accessibilityLabel}
      >
        {props.children}
      </BaseRadioGroup>
    </Radios.Provider>
  );
}

export interface RadioProps extends ChoiceProps {
  /** The app's id of the option. */
  value: string;
}

/** One option of a `RadioGroup`: a square box with a square dot while checked. */
export function Radio(props: RadioProps) {
  const group = Radios();
  return (
    <BaseRadio
      class={cx(choiceRow, props.class)}
      checked={group?.value() === props.value}
      disabled={props.disabled || group?.disabled()}
      accessibilityLabel={props.accessibilityLabel}
      testID={props.testID}
      onChange={() => group?.choose(props.value)}
    >
      <View class={cx(box, "size-4 group-checked:border-transparent group-checked:bg-foreground/18 group-disabled:border-foreground/12")}>
        <View class={cx("size-2 bg-bright opacity-0 group-checked:opacity-100 group-disabled:bg-foreground/38", motion.fast)} />
      </View>
      <If when={props.label}>
        <Text>{props.label}</Text>
      </If>
    </BaseRadio>
  );
}

export interface SliderProps {
  /** A number, or `[start, end]` for a range with two thumbs (controlled). */
  value: SliderValue;
  /** Every value the user moves through: write it back to `value`. */
  onChange?: (event: { value: SliderValue }) => void;
  /** The value the user's change ended at. */
  onChangeEnd?: (event: { value: SliderValue }) => void;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  /** What screen readers say for the thumb: what the value is. */
  accessibilityLabel: string;
  /** Shows the value after the track, in tabular figures. */
  showValue?: boolean;
  /** Formats the shown value (default: the number). */
  format?: (value: number) => string;
  testID?: string;
  class?: string;
}

/**
 * An inset rail with an accent fill up to a square thumb. Keys on the
 * focused thumb: Left/Down and Right/Up step, PageUp/PageDown jump, Home/End
 * go to the ends; a press on the rail moves the nearer thumb there.
 */
export function Slider(props: SliderProps) {
  const shown = () => {
    const f = props.format ?? String;
    const v = props.value;
    return Array.isArray(v) ? `${f(v[0])} – ${f(v[1])}` : f(v);
  };
  return (
    <BaseSlider
      class={cx("flex-row items-center gap-3", text.md, ink.body, "disabled:text-foreground/38", props.class)}
      value={props.value}
      min={props.min}
      max={props.max}
      step={props.step}
      disabled={props.disabled}
      accessibilityLabel={props.accessibilityLabel}
      testID={props.testID}
      onChange={props.onChange}
      onChangeEnd={props.onChangeEnd}
      trackClass="h-7 flex-1 justify-center px-2"
      indicatorClass="h-1 bg-inset"
      rangeClass="bg-omarchy-accent disabled:bg-foreground/38"
      thumbClass={cx(
        "size-4 border-2 border-omarchy-accent bg-background",
        "hover:bg-selection active:bg-omarchy-accent focus-visible:bg-omarchy-accent focus-visible:border-bright",
        "disabled:border-foreground/38 disabled:bg-background",
        motion.fast,
      )}
    >
      <If when={props.showValue}>
        <Text class={cx("min-w-16 text-right", ink.bright, "tabular-nums")}>{shown()}</Text>
      </If>
    </BaseSlider>
  );
}

export interface ProgressProps {
  /** 0 to 100. */
  value?: number;
  /** The extent of the work is unknown: a third of the track pulses. */
  indeterminate?: boolean;
  /** A label above the bar, which screen readers say. */
  label?: string;
  /** Shows the percentage beside the label. */
  showValue?: boolean;
  accessibilityLabel?: string;
  /** The fill's color: the accent (default), or a status. */
  tone?: "accent" | "success" | "warning" | "danger";
  testID?: string;
  class?: string;
}

const progressTone = {
  accent: "bg-omarchy-accent",
  success: "bg-success",
  warning: "bg-warning",
  danger: "bg-danger",
};

/** A thin inset track with a fill that glides to each value. */
export function Progress(props: ProgressProps) {
  return (
    <View class={cx("gap-1", text.sm, ink.secondary, props.class)}>
      <If when={props.label || props.showValue}>
        <View class="flex-row justify-between">
          <Text>{props.label ?? ""}</Text>
          <If when={props.showValue && !props.indeterminate}>
            <Text class={cx(ink.bright, "tabular-nums")}>{`${Math.round(props.value ?? 0)}%`}</Text>
          </If>
        </View>
      </If>
      <BaseProgress
        class="w-full"
        value={props.value}
        indeterminate={props.indeterminate}
        accessibilityLabel={props.accessibilityLabel ?? props.label}
        testID={props.testID}
        trackClass="w-full h-1.5 bg-inset"
        indicatorClass={cx(
          "h-1.5",
          progressTone[props.tone ?? "accent"],
          props.indeterminate ? "w-1/3 animate-pulse" : motion.base,
        )}
      />
    </View>
  );
}
