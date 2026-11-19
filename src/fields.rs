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

use std::rc::Rc;

use chrono::{Datelike, NaiveDate};
use nsh_core::element::{IntoProp, JsHost, JsIn, JsKind, JsOwned, JsValue, PropError, TsTypes};
use nsh_core::prelude::*;

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

// ---- Controlled values --------------------------------------------------

/// A field's value prop: set by the app (it owns the value and writes back
/// what `on_change` reports), or left out (the field keeps its own).
///
/// It is never written: the setters take the value itself
/// (`.value("dwindle")`, `.value(Some(4.0))`), a signal of it or a closure
/// returning it. In TSX it is the value, `undefined` when left out.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Controlled<T> {
    /// Left out: the field keeps its own value.
    Uncontrolled,
    /// The app's value.
    Set(T),
}

impl<T> Default for Controlled<T> {
    fn default() -> Self {
        Self::Uncontrolled
    }
}

impl<T: Clone> Controlled<T> {
    /// The app's value, or `None` while it is left out.
    pub fn get(&self) -> Option<T> {
        match self {
            Self::Uncontrolled => None,
            Self::Set(value) => Some(value.clone()),
        }
    }

    /// The app's value, else `own()`.
    pub(crate) fn or_else(&self, own: impl FnOnce() -> T) -> T {
        match self {
            Self::Uncontrolled => own(),
            Self::Set(value) => value.clone(),
        }
    }

    /// Left out.
    pub(crate) fn is_uncontrolled(&self) -> bool {
        matches!(self, Self::Uncontrolled)
    }
}

impl<T> From<T> for Controlled<T> {
    fn from(value: T) -> Self {
        Self::Set(value)
    }
}

impl From<&str> for Controlled<String> {
    fn from(value: &str) -> Self {
        Self::Set(value.to_owned())
    }
}

impl From<SharedString> for Controlled<String> {
    fn from(value: SharedString) -> Self {
        Self::Set(value.to_string())
    }
}

impl From<&str> for Controlled<Option<String>> {
    fn from(value: &str) -> Self {
        Self::Set(Some(value.to_owned()))
    }
}

impl From<String> for Controlled<Option<String>> {
    fn from(value: String) -> Self {
        Self::Set(Some(value))
    }
}

impl From<Option<&str>> for Controlled<Option<String>> {
    fn from(value: Option<&str>) -> Self {
        Self::Set(value.map(str::to_owned))
    }
}

impl From<Vec<&str>> for Controlled<Vec<String>> {
    fn from(value: Vec<&str>) -> Self {
        Self::Set(value.into_iter().map(str::to_owned).collect())
    }
}

impl From<f64> for Controlled<Option<f64>> {
    fn from(value: f64) -> Self {
        Self::Set(Some(value))
    }
}

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

/// `T`'s TypeScript type: `undefined` (left out) is the field's own value.
impl<T: JsValue> JsValue for Controlled<T> {
    fn ts_type(types: &mut TsTypes) -> String {
        T::ts_type(types)
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        match value.kind() {
            JsKind::Undefined => Ok(Self::Uncontrolled),
            _ => T::from_js(value).map(Self::Set),
        }
    }

    fn to_js<'a>(&self, host: &'a dyn JsHost) -> Result<JsOwned<'a>, PropError> {
        match self {
            Self::Uncontrolled => Ok(JsOwned::new(host, host.undefined())),
            Self::Set(value) => value.to_js(host),
        }
    }
}

/// How a signal of a field's value is its value prop ([`IntoProp`]).
#[doc(hidden)]
pub struct ControlledHandle(());

macro_rules! controlled_handles {
    ($($handle:ident),*) => {$(
        /// A signal of the value, as the app's value.
        impl<T: Clone + PartialEq + 'static> IntoProp<Controlled<T>, ControlledHandle> for $handle<T> {
            fn into_prop(self) -> Reactive<Controlled<T>> {
                Reactive::Bound(Rc::new(move || Controlled::Set(self.get())))
            }
        }
    )*};
}

controlled_handles!(Signal, ReadSignal, Memo);

// ---- Select items -------------------------------------------------------

/// An item of a [`Select`]'s or a [`Combobox`]'s list:
/// `SelectItem::new("dwindle", "Dwindle")`; in TSX
/// `{ value, label, group?, disabled? }`.
#[derive(Clone, Debug, Default, PartialEq, JsObject)]
#[non_exhaustive]
pub struct SelectItem {
    /// What choosing it sets the value to: unique among the items.
    pub value: String,
    /// What its row and the trigger show, and screen readers say.
    pub label: String,
    /// Its section: adjacent items of one group show under this title.
    #[prop(skip_none)]
    pub group: Option<String>,
    /// It cannot be chosen.
    #[prop(default)]
    pub disabled: bool,
}

impl SelectItem {
    /// An item of `value`, labelled `label`.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            group: None,
            disabled: false,
        }
    }

    /// Its section: adjacent items of one group show under `name`.
    pub fn group(mut self, name: impl Into<String>) -> Self {
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
