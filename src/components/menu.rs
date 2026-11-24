//! Omarchy menus: a dropdown and a context menu on `nsh_base`'s menus, and
//! the rows, separators and headings inside them.
//!
//! `nsh_base` owns the behavior: opening from the trigger (press, Return,
//! Space, Down), arrow keys / Home / End / typeahead between rows, Escape and
//! a press outside to close, focus back to the trigger. This module draws:
//!
//! - the popup: popover surface, 1px border, square, a short fade and
//!   [`ENTER_DISTANCE`](tokens::ENTER_DISTANCE) slide in over
//!   [`DURATION_BASE`](tokens::DURATION_BASE) (none under reduced motion);
//! - rows: hover/focus wash with accent text (the cursor), pressed fill,
//!   disabled text without washes;
//! - shortcuts: right-aligned monospaced keycap text;
//! - checkable rows: an accent check at the right edge while checked;
//! - separators (1px divider) and group headings.
//!
//! [`menu_dropdown`] and [`menu_context`] return the `nsh_base` elements, so
//! `.trigger(..)`, `.open(..)` and `.on_open_change(..)` stay available.

use nsh_base::{
    ContextMenuElement, MenuElement, MenuItemElement, MenuSeparatorElement, context_menu, menu,
    menu_item, menu_separator,
};
use nsh_core::{element::StyleRefinement, prelude::*};

use crate::tokens;

/// The popup's own look: surface, border, padding, type.
fn popup_surface(s: StyleRefinement) -> StyleRefinement {
    s.min_w(px(180.0))
        .p(px(tokens::SPACE_1))
        .gap(px(1.0))
        .bg(move || tokens::popover())
        .border_1()
        .border_color(move || tokens::border())
        .text_color(move || tokens::foreground())
        .text_size(px(tokens::TEXT_MD))
}

/// Its entrance and exit: a fade and a short slide, or nothing under reduced
/// motion.
fn popup_motion(s: StyleRefinement, animate: bool) -> StyleRefinement {
    if animate {
        s.transition_all()
            .duration(tokens::DURATION_BASE)
            .ease_out()
    } else {
        s
    }
}

fn popup_hidden(s: StyleRefinement, animate: bool) -> StyleRefinement {
    if animate {
        s.opacity(0.0).translate_y(px(-tokens::ENTER_DISTANCE))
    } else {
        s
    }
}

/// A dropdown menu: give it a `.trigger(..)` (e.g. [`menu_trigger`]) and
/// rows as children.
pub fn menu_dropdown() -> MenuElement {
    let animate = !reduce_motion();
    menu()
        .offset(px(tokens::SPACE_1))
        .margin(px(tokens::SPACE_2))
        .popup_style(move |s| popup_motion(popup_surface(s), animate))
        .popup_starting_style(move |s| popup_hidden(s, animate))
        .popup_ending_style(move |s| popup_hidden(s, animate))
        .backdrop_style(|s| s.bg(move || tokens::foreground().opacity(0.2)))
}

/// A context menu: its `.trigger(..)` is the area a secondary click (or
/// Shift+F10 / the Menu key) opens it on; rows as children.
pub fn menu_context() -> ContextMenuElement {
    let animate = !reduce_motion();
    context_menu()
        .offset(px(2.0))
        .margin(px(tokens::SPACE_2))
        .popup_style(move |s| popup_motion(popup_surface(s), animate))
        .popup_starting_style(move |s| popup_hidden(s, animate))
        .popup_ending_style(move |s| popup_hidden(s, animate))
        .backdrop_style(|s| s.bg(move || tokens::foreground().opacity(0.2)))
}

/// A button-like trigger for [`menu_dropdown`]: `label` and a chevron, an
/// outline, a selected fill while the menu is open.
pub fn menu_trigger(label: impl IntoReactive<SharedString>) -> View {
    view()
        .flex_row()
        .items_center()
        .self_start()
        .gap(px(tokens::SPACE_2))
        .h(px(tokens::CONTROL_MD))
        .px(px(tokens::SPACE_3))
        .border_1()
        .border_color(move || tokens::control_border())
        .text_color(move || tokens::foreground())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| s.border_color(move || tokens::focus_border()))
        .expanded(|s| s.bg(move || tokens::selected_fill()))
        .child(text(label).text_size(px(tokens::TEXT_MD)))
        .child(super::icon::indicator(
            super::icon::IconName::ChevronDown,
            14.0,
        ))
}

/// The look every row shares.
pub(crate) fn row_look(item: MenuItemElement) -> MenuItemElement {
    item.flex_row()
        .items_center()
        .gap(px(tokens::SPACE_2))
        .h(px(tokens::CONTROL_SM))
        .px(px(tokens::SPACE_2))
        .text_color(move || tokens::foreground())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        // The cursor row: the hover fill with accent text (a menu's own
        // cursor colour, not a persistent selection).
        .hover(|s| {
            s.bg(move || tokens::hover_fill())
                .text_color(move || tokens::accent())
        })
        .focus(|s| {
            s.bg(move || tokens::hover_fill())
                .text_color(move || tokens::accent())
        })
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .disabled_style(|s| {
            s.text_color(move || tokens::disabled())
                .bg(move || AdaptiveColor::TRANSPARENT)
        })
}

/// A menu row labelled `label`. Chain `.on_select(..)`, `.disabled(..)`,
/// and `.child(menu_shortcut(..))` for a keycap at its right edge.
pub fn menu_row(label: impl IntoReactive<SharedString>) -> MenuItemElement {
    row_look(menu_item()).child(text(label).flex_1().number_of_lines(1))
}

/// A checkable row: an accent check at its right edge while `checked`
/// holds. It stays open when chosen; the app flips the value in
/// `.on_select(..)`.
pub fn menu_check_row(
    label: impl IntoReactive<SharedString>,
    checked: impl Fn() -> bool + Clone + 'static,
) -> MenuItemElement {
    checkable(row_look(menu_item()), checked.clone())
        .child(text(label).flex_1().number_of_lines(1))
        .child(check_mark(checked))
}

/// Makes `item` checkable: it reports its state to assistive technologies
/// and stays open when chosen. The check itself is [`check_mark`], placed
/// after the row's label, at its right edge as a shortcut is.
pub(crate) fn checkable(
    item: MenuItemElement,
    checked: impl Fn() -> bool + 'static,
) -> MenuItemElement {
    item.checked(move || Some(checked()))
        .close_on_select(false)
        .indicator(view())
        .indicator_style(|s| s.w(px(0.0)))
}

/// The accent check at a checkable row's right edge, shown while `checked`
/// holds and keeping its room otherwise so labels do not shift.
pub(crate) fn check_mark(checked: impl Fn() -> bool + 'static) -> View {
    view()
        .ml(auto())
        .pl(px(tokens::SPACE_4))
        .items_center()
        .opacity(move || if checked() { 1.0_f32 } else { 0.0 })
        .text_color(move || tokens::accent())
        .child(super::icon::icon(super::icon::IconName::Check).size(px(12.0)))
}

/// A row's keyboard shortcut, written as the platform writes it
/// ([`super::shortcut_label`]: `"mod-n"` reads `cmd-n` on macOS, `ctrl-n`
/// elsewhere): monospaced, secondary, at the
/// row's right edge.
pub fn menu_shortcut(keys: impl Into<SharedString>) -> Text {
    text(super::shortcut_label(&keys.into()))
        .ml(auto())
        .pl(px(tokens::SPACE_4))
        .font_family(tokens::font_mono())
        .text_size(px(tokens::TEXT_SM))
        .text_color(move || tokens::secondary())
}

/// A 1px divider between groups of rows.
pub fn menu_divider() -> MenuSeparatorElement {
    menu_separator()
        .h(px(1.0))
        .my(px(tokens::SPACE_1))
        .bg(move || tokens::divider())
}

/// A heading over a group of rows: not focusable, not choosable.
pub fn menu_heading(label: impl IntoReactive<SharedString>) -> Text {
    text(label)
        .px(px(tokens::SPACE_2))
        .pt(px(tokens::SPACE_1))
        .pb(px(2.0))
        .text_size(px(tokens::TEXT_XS))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::secondary())
}
