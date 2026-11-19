//! Fields: text, numbers, codes, selects, dates and colors.
//!
//! ```ignore
//! use nsh_omarchy as om;
//!
//! let name = signal(String::new());
//! om::field("Hostname").description("Return submits.").child(
//!     om::input()
//!         .value(move || name.get())
//!         .on_change(move |text: &String| name.set(text.clone())),
//! )
//! ```


use chrono::{Datelike, NaiveDate};
use nsh_core::element::{IntoProp, JsHost, JsIn, JsOwned, JsValue, PropError, TsTypes};
use nsh_core::prelude::*;

use crate::types::Controlled;
use crate::ui::fields as ui;

pub use crate::ui::fields::{
    Calendar, CalendarElement, ColorPicker, ColorPickerElement, Combobox, ComboboxElement,
    DatePicker, DatePickerElement, Field, FieldElement, Input, InputElement, NumberInput,
    NumberInputElement, OtpInput, OtpInputElement, Select, SelectElement, Textarea,
    TextareaElement,
};
pub use nsh_core::Hsla;

// ---- Constructors -------------------------------------------------------

/// A one-line text field: `input().value(move || name.get())`.
pub fn input() -> InputElement {
    ui::input()
}

/// A text field of several lines; Return types a new line.
pub fn textarea() -> TextareaElement {
    ui::textarea()
}

/// A number with − / + steppers: `number_input().min(0.0).max(40.0)`.
pub fn number_input() -> NumberInputElement {
    ui::number_input()
}

/// A one-time code of `length` digits, one cell each: `otp_input(6)`.
pub fn otp_input(length: impl IntoReactive<u32>) -> OtpInputElement {
    ui::otp_input().length(length)
}

/// A trigger that opens a list of `items` to choose one from.
pub fn select<M>(items: impl IntoProp<Vec<SelectItem>, M>) -> SelectElement {
    ui::select().items(items)
}

/// A field whose typing filters `items`; one value, or several with
/// `.multiple(true)`.
pub fn combobox<M>(items: impl IntoProp<Vec<SelectItem>, M>) -> ComboboxElement {
    ui::combobox().items(items)
}

/// A month of days to pick a [`Date`] from, or a [`DateRange`] with
/// `.range(true)`.
pub fn calendar() -> CalendarElement {
    ui::calendar()
}

/// A trigger showing the picked date (`Nov 19, 2026`) that opens a
/// calendar.
pub fn date_picker() -> DatePickerElement {
    ui::date_picker()
}

/// A color editor: saturation and brightness, hue (and opacity with
/// `.alpha(true)`), a hex field and preset swatches.
pub fn color_picker() -> ColorPickerElement {
    ui::color_picker()
}

/// A control's `label` above it, and a description or an error below:
/// `field("Hostname").child(input())`.
pub fn field<M>(label: impl IntoProp<SharedString, M>) -> FieldElement {
    ui::field().label(label)
}

// A day as a field's value: `.value(Date::new(2026, 11, 19))`.
impl From<Date> for Controlled<Option<Date>> {
    fn from(value: Date) -> Self {
        Self::Set(Some(value))
    }
}

impl From<DateRange> for Controlled<Option<DateRange>> {
    fn from(value: DateRange) -> Self {
        Self::Set(Some(value))
    }
}

// ---- Select items -------------------------------------------------------

/// An item of a [`Select`]'s or a [`Combobox`]'s list:
/// `SelectItem::new("dwindle", "Dwindle")`; in TSX
/// `{ value, label, group?, disabled? }`.
#[derive(Clone, Debug, Default, PartialEq, JsObject)]
#[non_exhaustive]
pub struct SelectItem {
    /// What choosing it sets the value to: unique among the items.
    pub value: SharedString,
    /// What its row and the trigger show, and screen readers say.
    pub label: SharedString,
    /// Its section: adjacent items of one group show under this title.
    #[prop(skip_none)]
    pub group: Option<SharedString>,
    /// It cannot be chosen.
    #[prop(default)]
    pub disabled: bool,
}

impl SelectItem {
    /// An item of `value`, labelled `label`.
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            group: None,
            disabled: false,
        }
    }

    /// Its section: adjacent items of one group show under `name`.
    pub fn group(mut self, name: impl Into<SharedString>) -> Self {
        self.group = Some(name.into());
        self
    }

    /// Whether it cannot be chosen.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// As `nsh_base`'s item.
    pub(crate) fn to_base(&self) -> nsh_base::SelectItem {
        let mut item = nsh_base::SelectItem::new(self.value.clone(), self.label.clone())
            .disabled(self.disabled);
        if let Some(group) = &self.group {
            item = item.group(group.clone());
        }
        item
    }
}

// ---- Dates --------------------------------------------------------------

/// A calendar day: `Date::new(2026, 11, 19)`, `Date::today()`; in TSX an
/// ISO string, `"2026-11-19"`. It prints as that string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date(NaiveDate);

impl Date {
    /// The day `day` of month `month` (1 to 12) of `year`.
    ///
    /// # Panics
    ///
    /// When there is no such day (`Date::new(2026, 2, 30)`).
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self::from_ymd(year, month, day)
            .unwrap_or_else(|| panic!("no such day: {year}-{month:02}-{day:02}"))
    }

    /// The day `day` of month `month` of `year`, or `None` when there is
    /// no such day.
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).map(Self)
    }

    /// Today where the app runs (the engine's clock, the local time zone).
    pub fn today() -> Self {
        Self(today())
    }

    /// The year.
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// The month, 1 to 12.
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// The day of the month, from 1.
    pub fn day(&self) -> u32 {
        self.0.day()
    }

    /// The day as a date picker shows it: `Nov 19, 2026`.
    pub fn label(&self) -> String {
        self.0.format("%b %-d, %Y").to_string()
    }

    pub(crate) fn naive(self) -> NaiveDate {
        self.0
    }

    pub(crate) fn from_naive(date: NaiveDate) -> Self {
        Self(date)
    }
}

/// The ISO date, `2026-11-19`.
impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

/// An ISO date, `"2026-11-19"`.
impl std::str::FromStr for Date {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        NaiveDate::parse_from_str(text.trim(), "%Y-%m-%d")
            .map(Self)
            .map_err(|_| format!("not an ISO date (`2026-11-19`): {text:?}"))
    }
}

/// An ISO date string.
impl JsValue for Date {
    fn ts_type(_types: &mut TsTypes) -> String {
        "string".into()
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        value
            .as_string()
            .and_then(|text| text.parse().ok())
            .ok_or_else(|| value.expected("an ISO date (`\"2026-11-19\"`)"))
    }

    fn to_js<'a>(&self, host: &'a dyn JsHost) -> Result<JsOwned<'a>, PropError> {
        self.to_string().to_js(host)
    }
}

/// Two days, `start` to `end`, both included; in TSX `{ start, end }`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, JsObject)]
pub struct DateRange {
    /// The first day.
    pub start: Date,
    /// The last day.
    pub end: Date,
}

impl DateRange {
    /// `start` to `end`, both included.
    pub fn new(start: Date, end: Date) -> Self {
        Self { start, end }
    }

    /// The range as a date picker shows it: `Nov 19, 2026 – Nov 22, 2026`.
    pub fn label(&self) -> String {
        format!("{} \u{2013} {}", self.start.label(), self.end.label())
    }
}

/// Today where the app runs: the engine's clock in the local time zone.
fn today() -> NaiveDate {
    let ms = nsh_core::time::unix_ms();
    let utc = chrono::DateTime::from_timestamp_millis(ms as i64).unwrap_or_default();
    #[cfg(not(target_arch = "wasm32"))]
    {
        use chrono::TimeZone;
        chrono::Local
            .from_utc_datetime(&utc.naive_utc())
            .date_naive()
    }
    #[cfg(target_arch = "wasm32")]
    {
        #[link(wasm_import_module = "nsh")]
        unsafe extern "C" {
            /// The offset of local time at `ms` in seconds east of UTC.
            fn local_offset(ms: f64) -> f64;
        }
        // SAFETY: a pure host function.
        let east = unsafe { local_offset(ms) };
        let east = if east.is_finite() { east as i64 } else { 0 };
        (utc + chrono::TimeDelta::try_seconds(east).unwrap_or_default()).date_naive()
    }
}
