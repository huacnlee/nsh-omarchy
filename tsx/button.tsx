// Omarchy actions: buttons, icon buttons, toggles and single-choice groups on
// `nsh/base`'s Button, Toggle and ToggleGroup (focus, keyboard, roles).
//
// Every variant rests transparent: Primary is an accent outline with accent
// text (fills mark state, not priority), Danger the same in `danger`, Default
// a control-colored outline and Ghost none. Hover and keyboard focus wash the
// fill and draw the accent border; a press deepens the fill and scales the
// control to 97%; `selected` keeps the selected fill. Disabled shows none of
// those, in 38% text.
import type { JSX } from "nsh";
import { Text, View } from "nsh";
import { Button as BaseButton, Toggle as BaseToggle, ToggleGroup as BaseToggleGroup } from "nsh/base";
import { cx, fill, height, ink, line, motion, type Size, square, text } from "./theme";

/** How a button presents its action. */
export type ButtonVariant = "default" | "primary" | "ghost" | "danger";

const variantClass: Record<ButtonVariant, string> = {
  default: "border-foreground/40 text-foreground hover:border-omarchy-accent",
  primary: "border-omarchy-accent text-omarchy-accent",
  ghost: "border-transparent text-foreground",
  danger: "border-danger text-danger",
};

const disabledBorder: Record<ButtonVariant, string> = {
  default: "disabled:border-foreground/12",
  primary: "disabled:border-foreground/12",
  ghost: "disabled:border-transparent",
  danger: "disabled:border-foreground/12",
};

const sizeClass: Record<Size, string> = {
  sm: cx(height.sm, "px-2 gap-1", text.sm),
  md: cx(height.md, "px-3 gap-2", text.md),
  lg: cx(height.lg, "px-4 gap-2", text.lg),
};

/**
 * The classes of a button of `variant` and `size`, for a control of your own
 * that should look like one (spread on `nsh/base`'s `Button` or a pressable
 * `View`).
 */
export function buttonClass(variant: ButtonVariant = "default", size: Size = "md", extra?: string): string {
  return cx(
    "flex-row shrink-0 self-start items-center justify-center border bg-transparent",
    sizeClass[size],
    variantClass[variant],
    fill.hover,
    fill.focus,
    line.focus,
    fill.pressed,
    "selected:bg-foreground/18",
    motion.base,
    motion.press,
    "disabled:bg-transparent disabled:text-foreground/38 disabled:scale-100",
    disabledBorder[variant],
    extra,
  );
}

/** Props shared by the pressable controls of this file. */
interface PressableProps {
  /** No presses, no hover, focus or press feedback; 38% text. */
  disabled?: boolean;
  /** What screen readers say when the visible content does not (an icon). */
  accessibilityLabel?: string;
  /** A tooltip (`<Tooltip label="…" />` from `./overlays`). */
  tooltip?: JSX.Element;
  /** Finds the control in tests. */
  testID?: string;
  /** Classes appended to the control's own. */
  class?: string;
}

export interface ButtonProps extends PressableProps {
  /** The visible label. */
  label?: string;
  /** A glyph before the label (`<Glyph name="plus" />`). */
  icon?: JSX.Element;
  /** Content after the label (a keycap, a badge), or the whole content without `label`. */
  children?: JSX.Element;
  /** Default: a control-colored outline. Primary: the accent outline. Ghost: no outline. Danger: destructive. */
  variant?: ButtonVariant;
  /** 24, 28 (default) or 32 points tall. */
  size?: Size;
  /** Keeps the selected fill: the current choice among buttons. */
  selected?: boolean;
  onPress?: () => void;
}

/** A labelled action: `<Button label="Save" variant="primary" onPress={save} />`. */
export function Button(props: ButtonProps) {
  return (
    <BaseButton
      class={buttonClass(props.variant, props.size, props.class)}
      disabled={props.disabled}
      selected={props.selected}
      accessibilityLabel={props.accessibilityLabel}
      tooltip={props.tooltip}
      testID={props.testID}
      onPress={props.onPress}
    >
      {props.icon}
      <If when={props.label}>
        <Text>{props.label}</Text>
      </If>
      {props.children}
    </BaseButton>
  );
}

export interface IconButtonProps extends PressableProps {
  /** What the button does, for screen readers: required, the icon says nothing. */
  accessibilityLabel: string;
  /** The glyph. */
  children: JSX.Element;
  /** Ghost by default: toolbars and dense rows. */
  variant?: ButtonVariant;
  size?: Size;
  selected?: boolean;
  onPress?: () => void;
}

/** A square button holding one glyph: `<IconButton accessibilityLabel="Close"><Glyph name="cross" /></IconButton>`. */
export function IconButton(props: IconButtonProps) {
  return (
    <BaseButton
      class={buttonClass(props.variant ?? "ghost", props.size, cx("px-0", square[props.size ?? "md"], props.class))}
      disabled={props.disabled}
      selected={props.selected}
      accessibilityLabel={props.accessibilityLabel}
      tooltip={props.tooltip}
      testID={props.testID}
      onPress={props.onPress}
    >
      {props.children}
    </BaseButton>
  );
}

const toggleSize: Record<Size, string> = {
  sm: cx(height.sm, "px-2 gap-1", text.sm),
  md: cx(height.md, "px-3 gap-2", text.md),
  lg: cx(height.lg, "px-4 gap-2", text.lg),
};

/** The classes of a toggle: a border outline, the selected fill and bright text while pressed. */
function toggleClass(size: Size = "md", extra?: string): string {
  return cx(
    "flex-row shrink-0 self-start items-center justify-center border border-border bg-transparent",
    ink.body,
    toggleSize[size],
    fill.hover,
    fill.focus,
    line.focus,
    "active:border-bright",
    "aria-pressed:bg-foreground/18 aria-pressed:text-bright",
    motion.fast,
    "disabled:bg-transparent disabled:border-foreground/12 disabled:text-foreground/38",
    extra,
  );
}

export interface ToggleProps extends PressableProps {
  /** On or off (controlled). */
  pressed: boolean;
  /** The value asked for: write it back to `pressed`. */
  onChange?: (event: { pressed: boolean }) => void;
  label?: string;
  icon?: JSX.Element;
  children?: JSX.Element;
  size?: Size;
}

/** A button that is on or off: `<Toggle label="Bold" pressed={bold()} onChange={({ pressed }) => bold.set(pressed)} />`. */
export function Toggle(props: ToggleProps) {
  return (
    <BaseToggle
      class={toggleClass(props.size, props.class)}
      pressed={props.pressed}
      disabled={props.disabled}
      accessibilityLabel={props.accessibilityLabel}
      tooltip={props.tooltip}
      testID={props.testID}
      onChange={props.onChange}
    >
      {props.icon}
      <If when={props.label}>
        <Text>{props.label}</Text>
      </If>
      {props.children}
    </BaseToggle>
  );
}

/** One option of a `ToggleGroup` or a `ButtonGroup`. */
export interface GroupItem {
  /** The app's id of the option, never its label. */
  value: string;
  label: string;
  disabled?: boolean;
}

export interface ToggleGroupProps {
  items: GroupItem[];
  /** The values that are on (controlled). */
  value: string[];
  /** The values as they would be: write them back to `value`. */
  onChange?: (event: { value: string[] }) => void;
  /** Several on at once (default); `false`: at most one. */
  multiple?: boolean;
  disabled?: boolean;
  size?: Size;
  class?: string;
}

/**
 * A row of toggles, one stop of Tab: the arrows move between them, Space and
 * Return turn the focused one on or off.
 */
export function ToggleGroup(props: ToggleGroupProps) {
  const toggle = (value: string, on: boolean) => {
    const multiple = props.multiple ?? true;
    const current = props.value;
    const next = on ? (multiple ? [...current.filter((v) => v !== value), value] : [value]) : current.filter((v) => v !== value);
    props.onChange?.({ value: next });
  };
  return (
    <BaseToggleGroup class={cx("flex-row self-start gap-1", props.class)} disabled={props.disabled}>
      <Each of={props.items} key={(item) => item.value}>
        {(item) => (
          <BaseToggle
            class={toggleClass(props.size)}
            pressed={props.value.includes(item().value)}
            disabled={item().disabled}
            onChange={({ pressed }) => toggle(item().value, pressed)}
          >
            <Text>{item().label}</Text>
          </BaseToggle>
        )}
      </Each>
    </BaseToggleGroup>
  );
}

export interface ButtonGroupProps {
  items: GroupItem[];
  /** The chosen option's value (controlled). */
  value: string;
  /** The option asked for: write it back to `value`. */
  onChange?: (event: { value: string }) => void;
  disabled?: boolean;
  size?: Size;
  class?: string;
}

const segmentSize: Record<Size, string> = {
  sm: cx(height.sm, "px-3", text.sm),
  md: cx(height.md, "px-4", text.md),
  lg: cx(height.lg, "px-5", text.lg),
};

/**
 * A setting with one value: equal segments in one square frame, one stop of
 * Tab. The arrows move the focus between segments without choosing; Space and
 * Return choose. The chosen segment keeps the selected fill and bright text.
 */
export function ButtonGroup(props: ButtonGroupProps) {
  return (
    <BaseToggleGroup class={cx("flex-row self-start border border-border", props.class)} disabled={props.disabled}>
      <Each of={props.items} key={(item) => item.value}>
        {(item, index) => (
          <BaseToggle
            class={cx(
              "min-w-16 shrink-0 items-center justify-center border border-transparent",
              segmentSize[props.size ?? "md"],
              index() > 0 && "border-l-foreground/12",
              ink.body,
              fill.hover,
              fill.focus,
              line.focus,
              fill.pressed,
              "aria-pressed:bg-foreground/18 aria-pressed:text-bright",
              motion.fast,
              "disabled:bg-transparent disabled:text-foreground/38",
            )}
            pressed={props.value === item().value}
            disabled={item().disabled}
            onChange={({ pressed }) => {
              if (pressed) props.onChange?.({ value: item().value });
            }}
          >
            <Text>{item().label}</Text>
          </BaseToggle>
        )}
      </Each>
    </BaseToggleGroup>
  );
}
