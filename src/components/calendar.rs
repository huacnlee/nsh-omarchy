//! Omarchy calendar: [`calendar`] styles `nsh_base`'s `Calendar`, a
//! month's days to pick a date (or a range) from, with month and year
//! views behind the header's toggles.
//!
//! The selected day (every day of a range) has the selected fill, no
//! border; today's number is in the accent with a bar under it; the
//! days of the months around are secondary. Keyboard: the days are one Tab
//! stop; the arrows move a day or a week, Home and End to the week's ends,
//! Page Up and Down a month (with Shift a year), Return and Space pick.
//!
//! Motion: hover washes are fast color transitions (`DURATION_FAST`).

use nsh_base::{CalendarElement, CalendarEntry, CalendarItemKind, Date};
use nsh_core::element::StyleRefinement;
use nsh_core::prelude::*;

use crate::tokens;

/// Width of a day's column.
const DAY: f32 = 32.0;
/// Width of a standalone calendar: seven days and its padding.
const WIDTH: f32 = DAY * 7.0 + tokens::SPACE_3 * 2.0 + tokens::BORDER * 2.0;

const WEEKDAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// A month calendar in a bordered surface. Wire `.value(..)` (a [`Date`])
/// and `.on_change(..)` to the owner's state, or leave them out for the
/// calendar's own; `.range(true)` picks a range.
pub fn calendar() -> CalendarElement {
    nsh_base::calendar()
        .w(px(WIDTH))
        .p(px(tokens::SPACE_3))
        .gap(px(tokens::SPACE_2))
        .border_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::surface())
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
        .previous_accessibility_label("Previous month")
        .next_accessibility_label("Next month")
        .header_style(calendar_header_style)
        .row_style(calendar_row_style)
        .item_class("group")
        .item_style(calendar_item_style)
        .render_item(calendar_item)
}

/// A date as the gallery and triggers show it (`Nov 17, 2026`), or `None`
/// while incomplete.
pub fn format_date(date: &Date) -> Option<SharedString> {
    date.format("%b %-d, %Y")
}

/// The calendar's header: Previous, the month and year toggles, Next.
pub(crate) fn calendar_header_style(s: StyleRefinement) -> StyleRefinement {
    s.flex_row().items_center().gap(px(tokens::SPACE_1))
}

/// Each row: the weekdays, a week, a row of months or years.
pub(crate) fn calendar_row_style(s: StyleRefinement) -> StyleRefinement {
    s.flex_row()
}

/// Each item: a control tall, square, the selected fill while selected.
pub(crate) fn calendar_item_style(s: StyleRefinement) -> StyleRefinement {
    s.h(px(tokens::CONTROL_MD))
        .min_w(px(DAY))
        .flex_row()
        .items_stretch()
        .border_1()
        .border_color(move || AdaptiveColor::TRANSPARENT)
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .selected(|s| s.bg(move || tokens::selected_fill()))
}

/// An item's content, from what it is (its state is the item's).
pub(crate) fn calendar_item(entry: &CalendarEntry) -> View {
    let label: SharedString = match entry.kind {
        CalendarItemKind::Weekday => WEEKDAYS[entry.value.rem_euclid(7) as usize].into(),
        CalendarItemKind::MonthToggle => month_name(entry.value).into(),
        CalendarItemKind::Month => month_name(entry.value)[..3].to_owned().into(),
        CalendarItemKind::Previous | CalendarItemKind::Next => SharedString::default(),
        _ => entry.value.to_string().into(),
    };
    let kind = entry.kind;
    let today = entry.today;
    let outside = entry.outside;
    let pressable = kind != CalendarItemKind::Weekday;
    let toggle = matches!(
        kind,
        CalendarItemKind::MonthToggle | CalendarItemKind::YearToggle
    );
    view()
        .flex_1()
        .items_center()
        .justify_center()
        .px(px(if toggle { tokens::SPACE_2 } else { 0.0 }))
        .border_1()
        .border_color(move || AdaptiveColor::TRANSPARENT)
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .when(pressable, |v| {
            v.group_hover(|s| s.bg(move || tokens::hover_fill()))
                .group_focus_visible(|s| s.border_color(move || tokens::focus_border()))
        })
        .when(
            matches!(kind, CalendarItemKind::Previous | CalendarItemKind::Next),
            |v| {
                v.child(
                    super::icon::icon(if kind == CalendarItemKind::Previous {
                        super::icon::IconName::ChevronLeft
                    } else {
                        super::icon::IconName::ChevronRight
                    })
                    .size(px(16.0))
                    .text_color(move || tokens::foreground()),
                )
            },
        )
        .when(
            !matches!(kind, CalendarItemKind::Previous | CalendarItemKind::Next),
            |v| {
                v.child(
                    text(label)
                        .text_size(px(match kind {
                            CalendarItemKind::Weekday => tokens::TEXT_SM,
                            _ => tokens::TEXT_MD,
                        }))
                        .font_weight(if today || toggle {
                            FontWeight::SEMIBOLD
                        } else {
                            FontWeight::NORMAL
                        })
                        .when(kind == CalendarItemKind::Day, |t| t.tabular_nums())
                        .text_color(move || {
                            if today {
                                tokens::accent()
                            } else if outside || kind == CalendarItemKind::Weekday {
                                tokens::secondary()
                            } else if toggle {
                                tokens::bright()
                            } else {
                                tokens::foreground()
                            }
                        }),
                )
            },
        )
        .when(today, |v| {
            v.child(
                view()
                    .w(px(tokens::SPACE_3))
                    .h(px(1.0))
                    .bg(move || tokens::accent()),
            )
        })
}

fn month_name(month: i32) -> &'static str {
    MONTHS[(month - 1).clamp(0, 11) as usize]
}
