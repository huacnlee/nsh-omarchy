//! Omarchy links, after gpui-omarchy's `link`: underlined accent text that
//! brightens under the pointer, with the focus border and a hover wash when
//! the keyboard reaches it. A disabled link draws `disabled()` text without
//! the underline and shows no hover, press or focus state; it is no stop of
//! Tab and goes nowhere.
//!
//! The constructors return `nsh_base`'s [`LinkElement`], so callers keep its
//! builder API: `.open_with(..)`, `.on_open(..)`, `.title(..)`,
//! `.current(..)`, `.disabled(..)` and every style method.

use nsh_base::{LinkElement, link as base_link};
use nsh_core::prelude::*;

use super::button::actions_duration;
use crate::tokens;

/// A styled link with no content: add its label or icon with `.child(..)`.
pub fn link_style() -> LinkElement {
    base_link()
        .flex_row()
        .self_start()
        .items_center()
        .gap(px(tokens::SPACE_1))
        .px(px(tokens::SPACE_1))
        .rounded(px(tokens::RADIUS))
        .border(px(tokens::BORDER))
        .border_color(AdaptiveColor::TRANSPARENT)
        .bg(AdaptiveColor::TRANSPARENT)
        .text_color(move || tokens::accent())
        .text_size(px(tokens::TEXT_MD))
        .underline()
        .transition_colors()
        .duration(move || actions_duration(tokens::DURATION_FAST))
        .hover(|s| s.text_color(move || tokens::bright()))
        .focus_visible(|s| {
            s.bg(move || tokens::hover_fill())
                .border_color(move || tokens::focus_border())
        })
        .active(|s| {
            s.bg(move || tokens::pressed_fill())
                .text_color(move || tokens::bright())
        })
        // `disabled:` ranks above hover, focus and press: the resting look,
        // in disabled text.
        .disabled_style(|s| {
            s.bg(AdaptiveColor::TRANSPARENT)
                .border_color(AdaptiveColor::TRANSPARENT)
                .text_color(move || tokens::disabled())
                .no_underline()
        })
}

/// A link labelled `label` to `href`, a URL (opened by the system) or a
/// path of the app's (reported by `.on_open(..)`).
pub fn link(
    label: impl IntoReactive<SharedString>,
    href: impl IntoReactive<SharedString>,
) -> LinkElement {
    let href: Prop<SharedString> = href.into_reactive().into();
    link_style()
        .href(move || Some(href.get()))
        .child(text(label))
}
