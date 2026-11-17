// The Omarchy look as Tailwind classes: every component of this library draws
// with these constants, never with its own colors or sizes. They name the
// roles of `themes/omarchy.json` (the app's `nsh.json` `"theme"`), so they
// follow the light (Flexoki) and dark (Tokyo Night) palettes and any theme
// change at run time. Mirrors `src/tokens.rs` of the Rust crate.

/** Joins class lists, dropping the empty ones: `cx(base, on && "x", props.class)`. */
export function cx(...parts: (string | false | null | undefined)[]): string {
  return parts.filter(Boolean).join(" ");
}

/** Type scale (points): 10 / 11 / 12 / 14 / 16 / 20, Omarchy's compact desktop text. */
export const text = {
  xs: "text-[10px]",
  sm: "text-[11px]",
  md: "text-xs",
  lg: "text-sm",
  xl: "text-base",
  "2xl": "text-xl",
} as const;

/** Text colors. */
export const ink = {
  /** Body text. */
  body: "text-foreground",
  /** Captions, hints, placeholders. */
  secondary: "text-text-secondary",
  /** Titles, selected rows, values. */
  bright: "text-bright",
  /** The single accent: focus, primary actions, the current item. */
  accent: "text-omarchy-accent",
  /** Text on an accent fill. */
  onAccent: "text-on-accent",
  danger: "text-danger",
  warning: "text-warning",
  success: "text-success",
  /** Disabled text: the foreground at 38%. */
  disabled: "text-foreground/38",
} as const;

/** Foreground-tinted interaction fills. */
export const fill = {
  /** A control at rest: 4%. */
  normal: "bg-foreground/4",
  /** Under the pointer: 8%. */
  hover: "hover:bg-foreground/8",
  /** With the keyboard focus: the hover wash. */
  focus: "focus-visible:bg-foreground/8",
  /** A list row the keys or the pointer are on (`focus:` of nsh/base rows). */
  highlight: "focus:bg-foreground/8",
  /** A press in progress: 22%. */
  pressed: "active:bg-foreground/22",
  /** Persistent selection, alone (no marker, no accent): 18%. */
  selected: "bg-foreground/18",
} as const;

/** Lines. */
export const line = {
  /** Surfaces' borders. */
  surface: "border-border",
  /** Dividers inside a surface: the foreground at 12%. */
  divider: "border-foreground/12",
  /** Controls' borders: the foreground at 40%. */
  control: "border-foreground/40",
  /** The visible keyboard focus. */
  focus: "focus-visible:border-omarchy-accent",
  /** The focus of a part nsh/base marks `focus:` (an input that edits). */
  focusPart: "focus:border-omarchy-accent",
} as const;

/** Surfaces. Square everywhere: Omarchy's radius is 0. */
export const surface = {
  /** The window. */
  page: "bg-background",
  /** Raised: panels, cards, sheets. */
  raised: "bg-surface border border-border",
  /** Popups: menus, popovers, tooltips, select lists. */
  popup: "bg-popover border border-border",
  /** Recessed: wells, tracks, code. */
  inset: "bg-inset",
  /** The backdrop behind modal surfaces. */
  scrim: "bg-background/72",
} as const;

/** Control heights: 24 / 28 / 32. */
export const height = { sm: "h-6", md: "h-7", lg: "h-8" } as const;
/** Square controls (icon buttons, pagination cells): 24 / 28 / 32. */
export const square = { sm: "size-6", md: "size-7", lg: "size-8" } as const;

/** A control size. */
export type Size = "sm" | "md" | "lg";

/**
 * Motion: color washes 90 ms, popups and thumbs 140 ms, sheets 220 ms,
 * ease-out; none while the user asks for less motion.
 */
export const motion = {
  fast: "transition-colors duration-90 ease-out motion-reduce:transition-none",
  base: "transition duration-140 ease-out motion-reduce:transition-none",
  slow: "transition duration-220 ease-out motion-reduce:transition-none",
  /** A popup's first frame: faded and 4pt lower, which `base` animates from. */
  popupStarting: "opacity-0 translate-y-1",
  /** A popup while it goes. */
  popupEnding: "opacity-0",
  /** The press scale of buttons. */
  press: "active:scale-97 motion-reduce:active:scale-100",
} as const;

/** The monospaced family: keycaps, code, tabular figures. */
export const mono = "font-mono";

/**
 * The resting look shared by pressable controls: a transparent 1pt border
 * that the hover and the keyboard focus turn to the accent, the hover wash,
 * the pressed fill, and none of them while disabled.
 */
export const pressable = cx(
  "border border-transparent",
  fill.hover,
  fill.focus,
  line.focus,
  fill.pressed,
  "hover:border-omarchy-accent",
  "disabled:bg-transparent disabled:border-transparent disabled:text-foreground/38",
  motion.fast,
);
