//! [`Calendar`] and [`DatePicker`], on `nsh_base`'s calendar and date
//! picker ([`crate::components::calendar`],
//! [`crate::components::date_picker`]).

use nsh_base::{CalendarValue, DatePickerValue, DateTime, Matcher, PickedDate};
use nsh_core::prelude::*;

use super::Value;
use crate::components::button::ControlSize;
use crate::components::calendar::calendar as calendar_look;
use crate::components::date_picker::date_picker as date_picker_look;
use crate::fields::{Controlled, Date, DateRange};

/// What the calendar draws: one day or none, or a range or none.
#[derive(Clone, Copy)]
struct Days {
    range: Prop<bool>,
    single: Value<Option<Date>>,
    span: Value<Option<DateRange>>,
}

impl Days {
    fn new(
        range: Prop<bool>,
        value: Prop<Controlled<Option<Date>>>,
        span: Prop<Controlled<Option<DateRange>>>,
    ) -> Self {
        Self {
            range,
            single: Value::new(value, None),
            span: Value::new(span, None),
        }
    }

    /// The value as `nsh_base`'s (tracked).
    fn base(self) -> nsh_base::Date {
        if self.range.get() {
            match self.span.get() {
                Some(span) => {
                    nsh_base::Date::Range(Some(span.start.naive()), Some(span.end.naive()))
                }
                None => nsh_base::Date::Range(None, None),
            }
        } else {
            nsh_base::Date::Single(self.single.get().map(Date::naive))
        }
    }

    /// The value as a trigger shows it, `None` for none (tracked).
    fn label(self) -> Option<String> {
        if self.range.get() {
            self.span.get().map(|span| span.label())
        } else {
            self.single.get().map(|day| day.label())
        }
    }

    /// The user picked the complete value `picked`: the day or the range
    /// asked for, kept while the app does not own it.
    fn picked(self, picked: nsh_base::Date) -> Option<Picked> {
        match picked {
            nsh_base::Date::Single(Some(day)) => {
                let day = Date::from_naive(day);
                self.single.request(Some(day));
                Some(Picked::Day(day))
            }
            nsh_base::Date::Range(Some(start), Some(end)) => {
                let span = DateRange::new(Date::from_naive(start), Date::from_naive(end));
                self.span.request(Some(span));
                Some(Picked::Range(span))
            }
            _ => None,
        }
    }
}

/// A complete pick.
enum Picked {
    Day(Date),
    Range(DateRange),
}

/// The days that cannot be picked: all while `disabled`, else those before
/// `min` and after `max`.
fn matcher(
    disabled: Prop<bool>,
    min: Prop<Option<Date>>,
    max: Prop<Option<Date>>,
) -> impl Fn() -> Option<Matcher> + 'static {
    // One function for every read, so the calendar is not asked again.
    let every = Matcher::custom(|_| true);
    move || {
        if disabled.get() {
            return Some(every.clone());
        }
        let (min, max) = (min.get(), max.get());
        (min.is_some() || max.is_some())
            .then(|| Matcher::interval(min.map(Date::naive), max.map(Date::naive)))
    }
}

/// The props of [`Calendar`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CalendarProps {
    /// The picked day, `null` for none (`"2026-11-19"`). Set, the app owns
    /// it and writes back what `onChange` reports; left out, the calendar
    /// keeps the pick.
    #[prop(default)]
    pub value: Prop<Controlled<Option<Date>>>,
    /// Pick a range: the first press starts it, the second ends it
    /// (`rangeValue`, `onRangeChange`).
    #[prop(default)]
    pub range: Prop<bool>,
    /// The picked range while `range`, `null` for none (`{ start, end }`).
    /// Set, the app owns it; left out, the calendar keeps it.
    #[prop(default)]
    pub range_value: Prop<Controlled<Option<DateRange>>>,
    /// The first day that can be picked.
    pub min: Prop<Option<Date>>,
    /// The last day that can be picked.
    pub max: Prop<Option<Date>>,
    /// No day can be picked; the months still turn.
    #[prop(default)]
    pub disabled: Prop<bool>,
}

/// The events of a [`Calendar`] and a [`DatePicker`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum CalendarEvent {
    /// The user picked a day: the day asked for.
    #[args(value)]
    Change(Date),
    /// The user picked a range's end (while `range`): the range asked for.
    #[args(value)]
    RangeChange(DateRange),
}

/// A month of days to pick from, with month and year views behind the
/// header's toggles. The days are one Tab stop: the arrows move a day or a
/// week, Page Up / Page Down a month (with Shift a year), Return and Space
/// pick.
#[component]
pub fn Calendar(props: CalendarProps, cx: Scope<CalendarEvent>) -> impl IntoElement {
    let days = Days::new(props.range, props.value, props.range_value);
    calendar_look()
        .range(move || props.range.get())
        .value(move || CalendarValue::Controlled(days.base()))
        .disabled_matcher(matcher(props.disabled, props.min, props.max))
        .on_change(move |picked: &nsh_base::Date| match days.picked(*picked) {
            Some(Picked::Day(day)) => cx.emit(CalendarEvent::Change(day)),
            Some(Picked::Range(span)) => cx.emit(CalendarEvent::RangeChange(span)),
            None => {}
        })
}

/// The first moment of `day`.
fn midnight(day: chrono::NaiveDate) -> chrono::NaiveDateTime {
    day.and_time(chrono::NaiveTime::MIN)
}

/// The props of [`DatePicker`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DatePickerProps {
    /// The picked day, `null` for none (`"2026-11-19"`). Set, the app owns
    /// it and writes back what `onChange` reports; left out, the picker
    /// keeps the pick.
    #[prop(default)]
    pub value: Prop<Controlled<Option<Date>>>,
    /// Pick a range: the first press starts it, the second ends it
    /// (`rangeValue`, `onRangeChange`).
    #[prop(default)]
    pub range: Prop<bool>,
    /// The picked range while `range`, `null` for none (`{ start, end }`).
    /// Set, the app owns it; left out, the picker keeps it.
    #[prop(default)]
    pub range_value: Prop<Controlled<Option<DateRange>>>,
    /// The first day that can be picked.
    pub min: Prop<Option<Date>>,
    /// The last day that can be picked.
    pub max: Prop<Option<Date>>,
    /// It does not open; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What the trigger says while nothing is picked (default `"Choose a
    /// date"`, `"Choose dates"` for a range).
    pub placeholder: Prop<Option<SharedString>>,
    /// `small` (24 points tall), `medium` (28, the default) or `large` (32).
    #[prop(default)]
    pub size: Prop<ControlSize>,
    /// What screen readers call the trigger (default `"Choose a date"`).
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// A trigger showing the picked day (`Nov 19, 2026`) that opens a calendar
/// below it: Return or Space opens it, the calendar's keys move and pick,
/// a pick or Escape closes it and gives the focus back.
#[component]
pub fn DatePicker(props: DatePickerProps, cx: Scope<CalendarEvent>) -> impl IntoElement {
    let spoken = super::spoken(props.accessibility_label);
    let days = Days::new(props.range, props.value, props.range_value);
    let empty = move || days.label().is_none();
    let label = move || {
        let words = days.label().map(SharedString::from);
        words.unwrap_or_else(|| {
            props.placeholder.get().unwrap_or_else(|| {
                if props.range.get() {
                    "Choose dates"
                } else {
                    "Choose a date"
                }
                .into()
            })
        })
    };
    let size = props.size;
    date_picker_look(label, empty, move || size.get().height())
        .range(move || props.range.get())
        .value(move || {
            DatePickerValue::Controlled(match days.base() {
                nsh_base::Date::Single(day) => DateTime::Single(day.map(midnight)),
                nsh_base::Date::Range(start, end) => {
                    DateTime::Range(start.map(midnight), end.map(midnight))
                }
            })
        })
        .disabled(move || props.disabled.get())
        .disabled_matcher(matcher(Prop::new(false), props.min, props.max))
        .accessibility_label(move || Some(spoken().unwrap_or_else(|| "Choose a date".into())))
        .on_change(
            move |picked: &PickedDate| match days.picked(picked.value.date()) {
                Some(Picked::Day(day)) => cx.emit(CalendarEvent::Change(day)),
                Some(Picked::Range(span)) => cx.emit(CalendarEvent::RangeChange(span)),
                None => {}
            },
        )
}
