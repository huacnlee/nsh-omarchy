//! Omarchy date picker: [`date_picker`] styles `nsh_base`'s `DatePicker`,
//! a trigger that opens a calendar in a popup (a sheet on touch screens).
//!
//! The trigger is an outline control showing the owner's label and a
//! chevron; its border is the focus border with the keyboard on it and the
//! accent while the popup is open. The popup is a raised surface with a
//! 1px border holding the Omarchy [`calendar`](crate::calendar)'s look.
//! Keyboard: Return or Space on the trigger opens it, the calendar's keys
//! move and pick, Escape closes it and gives the focus back.
//!
//! Motion: the popup enters `ENTER_DISTANCE` over `DURATION_BASE` (the base
//! dropdown's motion); the trigger's border is a fast color transition.

use nsh_base::DatePickerElement;
use nsh_core::element::StyleRefinement;
use nsh_core::prelude::*;

use crate::components::calendar::{
    calendar_header_style, calendar_item, calendar_item_style, calendar_row_style,
};
use crate::tokens;

/// Width of the trigger.
const TRIGGER_WIDTH: f32 = 240.0;

/// A date picker whose trigger shows `label`: the owner's formatted value,
/// or a placeholder. Wire `.value(..)` and `.on_change(..)` (a
/// [`PickedDate`]) to the owner's state, or leave them out for its own;
/// `.range(true)` picks a range. Name it with `.accessibility_label(..)`
/// (default "Choose a date").
pub fn date_picker(label: impl IntoReactive<SharedString>) -> DatePickerElement {
    nsh_base::date_picker()
        .accessibility_label("Choose a date")
        .previous_accessibility_label("Previous month")
        .next_accessibility_label("Next month")
        .date_format("%b %-d, %Y")
        .offset(px(tokens::SPACE_1))
        .margin(px(tokens::SPACE_2))
        .popup_style(|s: StyleRefinement| {
            s.p(px(tokens::SPACE_3))
                .border_1()
                .border_color(move || tokens::border())
                .bg(move || tokens::popover())
                .text_size(px(tokens::TEXT_MD))
                .text_color(move || tokens::foreground())
        })
        .calendar_style(|s: StyleRefinement| s.gap(px(tokens::SPACE_2)))
        .header_style(calendar_header_style)
        .row_style(calendar_row_style)
        .item_class("group")
        .item_style(calendar_item_style)
        .render_item(calendar_item)
        .trigger(trigger(label))
        .self_start()
}

/// The closed control: the label and a chevron in an outline box.
fn trigger(label: impl IntoReactive<SharedString>) -> View {
    view()
        .flex_row()
        .items_center()
        .gap(px(tokens::SPACE_2))
        .w(px(TRIGGER_WIDTH))
        .h(px(tokens::CONTROL_MD))
        .px(px(tokens::SPACE_2))
        .border_1()
        .border_color(move || tokens::control_border())
        .bg(move || tokens::normal_fill())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .expanded(|s| s.border_color(move || tokens::accent()))
        .focus_visible(|s| s.border_color(move || tokens::focus_border()))
        .child(
            text(label)
                .flex_1()
                .min_w(px(0.0))
                .number_of_lines(1)
                .text_size(px(tokens::TEXT_MD)),
        )
        .child(
            text("\u{25BE}")
                .text_size(px(tokens::TEXT_SM))
                .text_color(move || tokens::secondary()),
        )
}
