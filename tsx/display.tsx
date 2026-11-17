// Omarchy surfaces and static pieces: Card, Panel, Separator, Badge, Keycap,
// Shortcut, EmptyState, Avatar, and the type scale (Heading, Title, Body,
// Caption, Code). Square, flat, one border; no shadows, no gradients.
import type { JSX } from "nsh";
import { Text, View } from "nsh";
import { Avatar as BaseAvatar } from "nsh/base";
import { cx, ink, surface, text } from "./theme";

export interface CardProps {
  title?: string;
  description?: string;
  children?: JSX.Element;
  /** Actions under the body, right-aligned. */
  footer?: JSX.Element;
  testID?: string;
  class?: string;
}

/** A raised surface with an optional title, description and footer. */
export function Card(props: CardProps) {
  return (
    <View class={cx(surface.raised, "gap-3 p-4", props.class)} testID={props.testID}>
      <If when={props.title || props.description}>
        <View class="gap-1">
          <If when={props.title}>
            <Text role="header" class={cx(text.lg, ink.bright, "font-semibold")}>
              {props.title}
            </Text>
          </If>
          <If when={props.description}>
            <Text class={cx(text.md, ink.secondary)}>{props.description}</Text>
          </If>
        </View>
      </If>
      {props.children}
      <If when={props.footer}>
        <View class="flex-row justify-end gap-2 border-t border-foreground/12 pt-3">{props.footer}</View>
      </If>
    </View>
  );
}

/** A plain surface: `raised` (default) or `inset` (a recessed well). */
export function Panel(props: { variant?: "raised" | "inset"; children?: JSX.Element; class?: string }) {
  return <View class={cx(props.variant === "inset" ? surface.inset : surface.raised, "p-3", props.class)}>{props.children}</View>;
}

/** A 1pt divider: horizontal (default) or vertical. */
export function Separator(props: { orientation?: "horizontal" | "vertical"; class?: string }) {
  return (
    <View
      accessibilityElementsHidden
      class={cx(props.orientation === "vertical" ? "w-px self-stretch" : "h-px self-stretch", "bg-foreground/12", props.class)}
    />
  );
}

/** A badge's meaning. */
export type BadgeTone = "neutral" | "accent" | "success" | "warning" | "danger";

const badgeTone: Record<BadgeTone, string> = {
  neutral: "border-foreground/40 text-foreground",
  accent: "border-omarchy-accent text-omarchy-accent",
  success: "border-success text-success",
  warning: "border-warning text-warning",
  danger: "border-danger text-danger",
};

const badgeSolid: Record<BadgeTone, string> = {
  neutral: "border-transparent bg-foreground/18 text-bright",
  accent: "border-transparent bg-omarchy-accent text-on-accent",
  success: "border-transparent bg-success text-on-accent",
  warning: "border-transparent bg-warning text-on-accent",
  danger: "border-transparent bg-danger text-on-accent",
};

/** A short status or count: outlined in its tone, or `solid`. */
export function Badge(props: { label: string; tone?: BadgeTone; solid?: boolean; class?: string }) {
  return (
    <View
      class={cx(
        "self-start h-5 px-1.5 items-center justify-center border",
        (props.solid ? badgeSolid : badgeTone)[props.tone ?? "neutral"],
        props.class,
      )}
    >
      <Text class={cx(text.xs, "font-semibold uppercase tracking-wide")}>{props.label}</Text>
    </View>
  );
}

/** One key, in monospace on a resting fill: `<Keycap label="Ctrl" />`. */
export function Keycap(props: { label: string; class?: string }) {
  return (
    <View
      role="keyboardkey"
      class={cx("h-5 min-w-5 px-1 items-center justify-center border border-foreground/40 bg-foreground/4", props.class)}
    >
      <Text class={cx("font-mono", text.xs, ink.bright)}>{props.label}</Text>
    </View>
  );
}

/** A key combination: `<Shortcut keys={["Super", "Shift", "Enter"]} />`. */
export function Shortcut(props: { keys: string[]; class?: string }) {
  return (
    <View class={cx("flex-row self-start items-center gap-1", props.class)} accessible accessibilityLabel={props.keys.join(" ")}>
      <Each of={props.keys}>{(key) => <Keycap label={key()} />}</Each>
    </View>
  );
}

/** What a view shows while it has nothing: a title, a hint, an action. */
export function EmptyState(props: { title: string; description?: string; action?: JSX.Element; class?: string }) {
  return (
    <View class={cx("items-center justify-center gap-2 border border-dashed border-foreground/40 p-8", props.class)}>
      <Text class={cx(text.lg, ink.bright, "font-semibold")}>{props.title}</Text>
      <If when={props.description}>
        <Text class={cx(text.md, ink.secondary, "text-center")}>{props.description}</Text>
      </If>
      {props.action}
    </View>
  );
}

const avatarSize = {
  sm: cx("size-6", text.xs),
  md: cx("size-8", text.sm),
  lg: cx("size-10", text.lg),
};

/** A square avatar: the picture, or the person's initials while it loads, when it fails and without one. */
export function Avatar(props: { name: string; src?: string; size?: "sm" | "md" | "lg"; class?: string }) {
  return (
    <BaseAvatar
      class={cx("shrink-0 overflow-hidden", avatarSize[props.size ?? "md"], props.class)}
      name={props.name}
      src={props.src}
      imageClass="transition-opacity duration-140 motion-reduce:transition-none"
      fallbackClass="items-center justify-center bg-selection text-bright font-semibold"
    />
  );
}

// ---- Typography -----------------------------------------------------------

interface TypeProps {
  children: JSX.Element;
  class?: string;
}

/** A page's heading: 20pt, bright, a screen reader's header. */
export function Heading(props: TypeProps) {
  return (
    <Text role="header" class={cx(text["2xl"], ink.bright, "font-semibold", props.class)}>
      {props.children}
    </Text>
  );
}

/** A section's title: 14pt, bright. */
export function Title(props: TypeProps) {
  return (
    <Text role="header" class={cx(text.lg, ink.bright, "font-semibold", props.class)}>
      {props.children}
    </Text>
  );
}

/** Body text: 12pt. */
export function Body(props: TypeProps) {
  return <Text class={cx(text.md, ink.body, props.class)}>{props.children}</Text>;
}

/** Captions and hints: 11pt, secondary. */
export function Caption(props: TypeProps) {
  return <Text class={cx(text.sm, ink.secondary, props.class)}>{props.children}</Text>;
}

/** Inline code: monospace on the inset surface. */
export function Code(props: TypeProps) {
  return <Text class={cx("font-mono px-1 bg-inset", text.sm, ink.bright, props.class)}>{props.children}</Text>;
}
