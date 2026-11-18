//! Text and number fields: [`text_input`] (one line), [`textarea`] (several
//! lines) and [`number_field`] (a number with − / + steppers).
//!
//! All are `nsh_base`'s native text field (typing, selection, clipboard,
//! input methods) dressed in Omarchy's control look: a `normal_fill` well
//! with a `control_border`, the accent border and a `hover_fill` wash while
//! it edits, `secondary` placeholder, accent caret, `disabled` text. They
//! are controlled: `.value(..)` plus `.on_change(..)` writing the owner's
//! state. They never rewrite what is typed, so an input method's
//! composition (pinyin, kana) is left alone.
//!
//! The names avoid `nsh_base::input` / `nsh_base::number_input`, so both
//! crates' preludes can be imported together.

use nsh_base::{Input, NumberInputElement, number_input};
use nsh_core::element::StyleRefinement;
use nsh_core::prelude::*;

use crate::tokens;

/// The minimum height of a [`textarea`]: about four lines of `TEXT_MD`.
const TEXTAREA_MIN_HEIGHT: f32 = 80.0;

/// Tailwind classes for the states of `nsh_base` parts that only classes
/// reach (a part's typed style has no `hover:` / `focus:`). Each is one
/// token of [`crate::tokens`] written as the theme roles it is made of, so
/// it follows the theme as the token does.
pub(crate) use crate::tokens::classes as state_classes;

/// The text styles every Omarchy field shares.
fn field_text<E: Styled>(field: E) -> E {
    field
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
}

/// The frame every Omarchy field shares (a text field's own, a number
/// field's row): the well, its border and their states.
fn field_frame<E: Interactive>(field: E) -> E {
    field
        .min_w(px(0.0))
        .rounded(px(tokens::RADIUS))
        .border_1()
        .border_color(move || tokens::control_border())
        .bg(move || tokens::normal_fill())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .disabled_style(|s| {
            s.text_color(move || tokens::disabled())
                .bg(move || tokens::normal_fill())
                .border_color(move || tokens::divider())
        })
}

/// The native field's own look: colors only a resolved `Hsla` reaches.
fn field_colors(field: Input) -> Input {
    field
        .placeholder_color(move || color(tokens::secondary()))
        .caret_color(move || color(tokens::accent()))
        .selection_color(move || color(tokens::text_selection()))
}

fn styled_field(field: Input) -> Input {
    field_frame(field_text(field_colors(field)))
        .px(px(tokens::SPACE_2))
        .focus(|s| {
            s.bg(move || tokens::hover_fill())
                .border_color(move || tokens::focus_border())
        })
}

/// A single-line text field, `CONTROL_MD` tall. Give it a
/// `.label(..)` (what screen readers call it) when no visible label names it.
///
/// ```ignore
/// let name = signal(String::new());
/// text_input()
///     .value(move || name.get())
///     .on_change(move |text: &str| name.set(text.to_owned()))
///     .placeholder("Workspace name")
///     .label("Workspace name")
/// ```
pub fn text_input() -> Input {
    styled_field(nsh_base::input()).h(px(tokens::CONTROL_MD))
}

/// A multi-line text field that grows with its text from about four lines.
/// Return adds a line.
pub fn textarea() -> Input {
    styled_field(nsh_base::input())
        .multiline(true)
        .min_h(px(TEXTAREA_MIN_HEIGHT))
        .py(px(tokens::SPACE_1))
}

/// A stepper button's glyph.
fn stepper_glyph(name: super::icon::IconName, label: &'static str) -> impl IntoElement {
    view()
        .label(label)
        .child(super::icon::indicator(name, 14.0))
}

/// A number field: − before the value, + after it, `CONTROL_MD` tall.
/// Up / Down step it, Page Up / Page Down ten steps (they repeat while
/// held); typing is held to `min` / `max` when the field is left or Return
/// is pressed. Controlled: `.value(..)` (an `Option<f64>`) plus
/// `.on_change(move |v: &Option<f64>| ..)`; `.min`, `.max`, `.step`,
/// `.disabled` and `.accessibility_label` as `nsh_base::number_input`.
pub fn number_field() -> NumberInputElement {
    use state_classes::*;
    let button_class: SharedString = format!("{HOVER} {PRESSED} {DISABLED}").into();
    let button_style = |s: StyleRefinement| {
        s.w(px(tokens::CONTROL_MD))
            .h_full()
            .items_center()
            .justify_center()
            .transition_colors()
            .duration(tokens::DURATION_FAST)
            .cursor_pointer()
    };
    let field = number_input()
        .flex_row()
        .items_stretch()
        .h(px(tokens::CONTROL_MD))
        .min_w(px(0.0))
        .rounded(px(tokens::RADIUS))
        .border_1()
        .border_color(move || tokens::control_border())
        .bg(move || tokens::normal_fill())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .focus_within(|s| {
            s.bg(move || tokens::hover_fill())
                .border_color(move || tokens::focus_border())
        })
        .disabled_style(|s| {
            s.text_color(move || tokens::disabled())
                .border_color(move || tokens::divider())
        });
    field_text(field)
        .input_class(PLACEHOLDER)
        .input_style(|s: StyleRefinement| {
            s.flex_1()
                .min_w(px(0.0))
                .px(px(tokens::SPACE_2))
                .text_center()
                .text_size(px(tokens::TEXT_MD))
                .text_color(move || tokens::foreground())
                .tabular_nums()
        })
        .decrement_button_class(button_class.clone())
        .decrement_button_style(move |s: StyleRefinement| {
            button_style(s)
                .border_r_1()
                .border_color(move || tokens::divider())
        })
        .decrement_button(stepper_glyph(super::icon::IconName::Minus, "Decrease"))
        .increment_button_class(button_class)
        .increment_button_style(move |s: StyleRefinement| {
            button_style(s)
                .border_l_1()
                .border_color(move || tokens::divider())
        })
        .increment_button(stepper_glyph(super::icon::IconName::Plus, "Increase"))
}

#[cfg(test)]
mod tests {
    use super::state_classes::*;
    use nsh_core::tailwind::{Theme, parse};

    #[test]
    fn every_state_class_names_a_known_utility_and_token() {
        let mut theme = Theme::default();
        theme.set_tokens(&crate::Theme::tokyo_night().to_tokens());
        for class in [
            HOVER,
            PRESSED,
            HIGHLIGHT,
            FOCUS_FILL,
            CLEAR_BORDER,
            FOCUS_VISIBLE_BORDER,
            CONTROL_BORDER,
            FOCUS_BORDER,
            DISABLED,
            PLACEHOLDER,
        ] {
            let list = parse(&theme, class);
            assert!(list.unknown().is_empty(), "{class}: {:?}", list.unknown());
            assert!(!list.is_empty(), "{class} applies nothing");
        }
    }
}
