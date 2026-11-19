//! Omarchy buttons: content-sized actions with quiet hover, focus and
//! pressed washes, after gpui-omarchy's `button` and `icon_button`.
//!
//! Every variant rests transparent. Primary is an accent outline with accent
//! text (fills mark state, not priority), Danger the same in `danger()`,
//! Default a control-colored outline and Ghost no outline at all. Hover and
//! keyboard focus wash the fill with `hover_fill()`; focus also draws the
//! `focus_border()`; a press deepens the fill to `pressed_fill()` and scales
//! the button by `PRESS_SCALE`; `selected` keeps `selected_fill()`. A
//! disabled button draws `disabled()` text and none of those states.
//!
//! The constructors return `nsh_base`'s [`ButtonElement`], so callers keep
//! its builder API: `.on_press(..)`, `.disabled(..)`, `.selected(..)`,
//! `.accessibility_label(..)` and every style method.

use nsh_base::{ButtonElement, button as base_button};
use nsh_core::prelude::*;

use crate::tokens;

/// How a button presents its action.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PropEnum)]
#[non_exhaustive]
pub enum ButtonVariant {
    /// A control-colored outline with body text: most actions.
    #[default]
    Default,
    /// The accent outline with accent text: the action a view leads with.
    Primary,
    /// No outline until focused: toolbars, dense rows, secondary actions.
    Ghost,
    /// The danger outline with danger text: destructive actions.
    Danger,
}

/// A control height: 24, 28 or 32 points.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PropEnum)]
#[non_exhaustive]
pub enum ControlSize {
    /// 24 points tall.
    Small,
    /// 28 points tall.
    #[default]
    Medium,
    /// 32 points tall.
    Large,
}

impl ControlSize {
    /// The control's height in points.
    pub(crate) fn height(self) -> f32 {
        match self {
            ControlSize::Small => tokens::CONTROL_SM,
            ControlSize::Medium => tokens::CONTROL_MD,
            ControlSize::Large => tokens::CONTROL_LG,
        }
    }

    /// The horizontal padding of a labelled control.
    pub(crate) fn padding(self) -> f32 {
        match self {
            ControlSize::Small => tokens::SPACE_2,
            ControlSize::Medium => tokens::SPACE_3,
            ControlSize::Large => tokens::SPACE_4,
        }
    }

    /// The label's text size.
    pub(crate) fn text_size(self) -> f32 {
        match self {
            ControlSize::Small => tokens::TEXT_SM,
            ControlSize::Medium => tokens::TEXT_MD,
            ControlSize::Large => tokens::TEXT_LG,
        }
    }

    /// The gap between an icon and a label.
    pub(crate) fn gap(self) -> f32 {
        match self {
            ControlSize::Small => tokens::SPACE_1,
            ControlSize::Medium | ControlSize::Large => tokens::SPACE_2,
        }
    }
}

/// A transition's length, or none while the user asks for less motion.
/// Read it in a binding: `.duration(move || actions_duration(..))`.
pub(crate) fn actions_duration(ms: f32) -> f32 {
    if reduce_motion() { 0.0 } else { ms }
}

pub(crate) fn rest_border(variant: ButtonVariant) -> AdaptiveColor {
    match variant {
        ButtonVariant::Default => tokens::control_border(),
        ButtonVariant::Primary => tokens::accent(),
        ButtonVariant::Ghost => AdaptiveColor::TRANSPARENT,
        ButtonVariant::Danger => tokens::danger(),
    }
}

pub(crate) fn hover_border(variant: ButtonVariant) -> AdaptiveColor {
    match variant {
        ButtonVariant::Default => tokens::focus_border(),
        other => rest_border(other),
    }
}

pub(crate) fn disabled_border(variant: ButtonVariant) -> AdaptiveColor {
    match variant {
        ButtonVariant::Ghost => AdaptiveColor::TRANSPARENT,
        _ => tokens::divider(),
    }
}

pub(crate) fn label_color(variant: ButtonVariant) -> AdaptiveColor {
    match variant {
        ButtonVariant::Default | ButtonVariant::Ghost => tokens::foreground(),
        ButtonVariant::Primary => tokens::accent(),
        ButtonVariant::Danger => tokens::danger(),
    }
}

/// A styled button with no content: add the label, an icon or both with
/// `.child(..)`. Give an icon-only button an `.accessibility_label(..)`.
pub fn button_style(variant: ButtonVariant, size: ControlSize) -> ButtonElement {
    button_style_with(move || variant, move || size)
}

/// [`button_style`] whose variant and size follow `variant()` and `size()`
/// (props that may change).
/// The button look on `$el` (a builder of `nsh_base`'s button, or a view:
/// the TSX buttons draw it on a view of their own).
macro_rules! button_look {
    ($el:expr, $variant:expr, $size:expr) => {{
        let variant = $variant;
        let size = $size;
        $el
        .flex_row()
        .flex_shrink_0()
        .self_start()
        .items_center()
        .justify_center()
        .h(move || px(size().height()))
        .px(move || px(size().padding()))
        .gap(move || px(size().gap()))
        .rounded(px($crate::tokens::RADIUS))
        .border(px($crate::tokens::BORDER))
        .border_color(move || $crate::components::button::rest_border(variant()))
        .bg(AdaptiveColor::TRANSPARENT)
        .text_color(move || $crate::components::button::label_color(variant()))
        .text_size(move || px(size().text_size()))
        .cursor_pointer()
        .transition()
        .duration(move || $crate::components::button::actions_duration($crate::tokens::DURATION_FAST))
        // The `selected:` variant (the element's own `.selected(..)` is the
        // prop that sets the state).
        .map(|b| Styled::selected(b, |s| s.bg(move || $crate::tokens::selected_fill())))
        .hover(|s| {
            s.bg(move || $crate::tokens::hover_fill())
                .border_color(move || $crate::components::button::hover_border(variant()))
        })
        .focus_visible(|s| {
            s.bg(move || $crate::tokens::hover_fill())
                .border_color(move || $crate::tokens::focus_border())
        })
        .active(|s| {
            s.bg(move || $crate::tokens::pressed_fill())
                .scale($crate::tokens::PRESS_SCALE)
        })
        // `disabled:` ranks above hover, focus and press: it puts back the
        // resting look, so a disabled button shows none of them.
        .disabled_style(|s| {
            s.bg(AdaptiveColor::TRANSPARENT)
                .border_color(move || $crate::components::button::disabled_border(variant()))
                .text_color(move || $crate::tokens::disabled())
                .scale(1.0)
                .cursor(Cursor::Default)
        })
    }};
}
pub(crate) use button_look;

pub(crate) fn button_style_with(
    variant: impl Fn() -> ButtonVariant + Copy + 'static,
    size: impl Fn() -> ControlSize + Copy + 'static,
) -> ButtonElement {
    button_look!(base_button(), variant, size)
}

/// A labelled button of `variant` and `size`.
pub fn button_variant(
    label: impl IntoReactive<SharedString>,
    variant: ButtonVariant,
    size: ControlSize,
) -> ButtonElement {
    button_style(variant, size).child(text(label))
}

/// A labelled default button, medium size: `button("Save").on_press(..)`.
pub fn button(label: impl IntoReactive<SharedString>) -> ButtonElement {
    button_variant(label, ButtonVariant::Default, ControlSize::Medium)
}

/// A labelled primary button (accent outline, accent text).
pub fn button_primary(label: impl IntoReactive<SharedString>) -> ButtonElement {
    button_variant(label, ButtonVariant::Primary, ControlSize::Medium)
}

/// A labelled ghost button (no outline at rest).
pub fn button_ghost(label: impl IntoReactive<SharedString>) -> ButtonElement {
    button_variant(label, ButtonVariant::Ghost, ControlSize::Medium)
}

/// A labelled destructive button.
pub fn button_danger(label: impl IntoReactive<SharedString>) -> ButtonElement {
    button_variant(label, ButtonVariant::Danger, ControlSize::Medium)
}

/// A button with an icon before its label.
pub fn button_with_icon(
    icon: impl IntoElement,
    label: impl IntoReactive<SharedString>,
    variant: ButtonVariant,
    size: ControlSize,
) -> ButtonElement {
    button_style(variant, size).child(icon).child(text(label))
}

/// A square, icon-only button. `label` names the action for screen readers
/// and test selectors (`label:Add workspace`).
pub fn icon_button(
    icon: impl IntoElement,
    label: impl IntoReactive<SharedString>,
    variant: ButtonVariant,
    size: ControlSize,
) -> ButtonElement {
    let label: Prop<SharedString> = label.into_reactive().into();
    button_style(variant, size)
        .accessibility_label(move || Some(label.get()))
        .w(px(size.height()))
        .px(px(0.0))
        .child(icon)
}
