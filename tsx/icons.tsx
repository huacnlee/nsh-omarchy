// The few glyphs the controls need, drawn with views in the current text
// color (`bg-current`, `border-current`): they follow the theme and the state
// of the control around them (hover, selected, disabled) with no color of
// their own. Decorative: hidden from screen readers, so put the meaning in
// the control's label.
import { View } from "nsh";
import { cx } from "./theme";

/** A glyph's name. */
export type GlyphName =
  | "check"
  | "dash"
  | "dot"
  | "plus"
  | "minus"
  | "cross"
  | "chevron-down"
  | "chevron-up"
  | "chevron-left"
  | "chevron-right"
  | "menu";

/** The rotation that points a chevron (the corner of a box) its way. */
const chevronTurn: Record<string, string> = {
  "chevron-down": "rotate-45 -mt-1",
  "chevron-up": "-rotate-135 mt-1",
  "chevron-left": "rotate-135 ml-1",
  "chevron-right": "-rotate-45 -ml-1",
};

/**
 * A 12pt glyph in the current text color: `<Glyph name="check" />`. Tint it
 * with a text class on it or on the control around it.
 */
export function Glyph(props: { name: GlyphName; class?: string }) {
  const name = props.name;
  const box = () => cx("size-3 shrink-0 items-center justify-center", props.class);
  if (name === "check") {
    return (
      <View accessibilityElementsHidden class={box()}>
        <View class="-mt-0.5 h-2.25 w-1.25 rotate-45 border-r-2 border-b-2 border-current" />
      </View>
    );
  }
  if (name === "dash" || name === "minus") {
    return (
      <View accessibilityElementsHidden class={box()}>
        <View class={name === "dash" ? "h-0.5 w-2 bg-current" : "h-px w-2.5 bg-current"} />
      </View>
    );
  }
  if (name === "dot") {
    return (
      <View accessibilityElementsHidden class={box()}>
        <View class="size-1.5 bg-current" />
      </View>
    );
  }
  if (name === "plus") {
    return (
      <View accessibilityElementsHidden class={box()}>
        <View class="absolute h-px w-2.5 bg-current" />
        <View class="absolute h-2.5 w-px bg-current" />
      </View>
    );
  }
  if (name === "cross") {
    return (
      <View accessibilityElementsHidden class={box()}>
        <View class="absolute h-px w-3 rotate-45 bg-current" />
        <View class="absolute h-px w-3 -rotate-45 bg-current" />
      </View>
    );
  }
  if (name === "menu") {
    return (
      <View accessibilityElementsHidden class={cx(box(), "gap-0.5")}>
        <View class="h-px w-2.5 bg-current" />
        <View class="h-px w-2.5 bg-current" />
        <View class="h-px w-2.5 bg-current" />
      </View>
    );
  }
  return (
    <View accessibilityElementsHidden class={box()}>
      <View class={cx("size-1.5 border-r border-b border-current", chevronTurn[name])} />
    </View>
  );
}
