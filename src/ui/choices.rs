//! `<Checkbox>`, `<Switch>`, `<RadioGroup>` / `<Radio>`, `<Slider>` and
//! `<Progress>` for TSX, drawn by [`crate::components`]'s `checkbox`,
//! `switch`, `radio`, `slider` and `progress`.
//!
//! All controlled: the value comes in as a prop, and the value the user asks
//! for goes out in `onChange`, for the app to write back.

use std::{cell::Cell, fmt, rc::Rc};

use nsh_base::SliderValue;
use nsh_core::{
    element::{Arg, Callback, JsHost, JsIn, JsKind, JsOwned, JsValue, PropError, TsTypes},
    prelude::*,
};

use crate::components::{
    self as ui, checkbox::checkbox_control, radio::radio_control, switch::switch_control,
};
use crate::tokens;

/// The props of [`Checkbox`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CheckboxProps {
    /// The visible label, which screen readers say.
    pub label: Prop<Option<SharedString>>,
    /// What screen readers say when there is no label.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Checked or not (controlled): write back what `onChange` asks for.
    #[prop(default)]
    pub checked: Prop<bool>,
    /// Neither checked nor unchecked: a dash, and a press asks for checked.
    #[prop(default)]
    pub indeterminate: Prop<bool>,
    /// No presses, dimmed, no interaction styles.
    #[prop(default)]
    pub disabled: Prop<bool>,
}

/// The events of a [`Checkbox`] and a [`Switch`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum CheckedEvent {
    /// A press (a click, a tap, Space or Return) asks for this value: write
    /// it back to `checked`.
    #[args(checked)]
    Change(bool),
}

/// A square box with a drawn check (a dash while indeterminate), and its
/// label: `<Checkbox label="Sync" checked={sync()} onChange={({ checked }) => sync.set(checked)} />`.
#[component]
pub fn Checkbox(props: CheckboxProps, cx: Scope<CheckedEvent>) -> impl IntoElement {
    checkbox_control()
        .label(move || props.label.get().unwrap_or_default())
        .accessibility_label(move || props.accessibility_label.get())
        .checked(props.checked)
        .indeterminate(props.indeterminate)
        .disabled(props.disabled)
        .on_change(move |next: &bool| cx.emit(CheckedEvent::Change(*next)))
}

/// The props of [`Switch`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SwitchProps {
    /// The visible label, which screen readers say.
    pub label: Prop<Option<SharedString>>,
    /// What screen readers say when there is no label.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// On or off (controlled): write back what `onChange` asks for.
    #[prop(default)]
    pub checked: Prop<bool>,
    /// No presses, dimmed, no interaction styles.
    #[prop(default)]
    pub disabled: Prop<bool>,
}

/// A square track whose square thumb slides from the start (off) to the end
/// (on) over 140 ms; on fills the track and brightens the thumb.
#[component]
pub fn Switch(props: SwitchProps, cx: Scope<CheckedEvent>) -> impl IntoElement {
    switch_control()
        .label(move || props.label.get().unwrap_or_default())
        .accessibility_label(move || props.accessibility_label.get())
        .checked(props.checked)
        .disabled(props.disabled)
        .on_change(move |next: &bool| cx.emit(CheckedEvent::Change(*next)))
}

/// What a [`RadioGroup`] gives its [`Radio`]s, through context.
#[derive(Clone)]
struct Radios {
    value: Prop<Option<SharedString>>,
    disabled: Prop<bool>,
    choose: Rc<dyn Fn(SharedString)>,
}

/// The props of [`RadioGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct RadioGroupProps {
    /// The checked radio's value (controlled), or none.
    pub value: Prop<Option<SharedString>>,
    /// What screen readers call the set ("Plan").
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Every radio of the set disabled.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Lays the radios out in a row.
    #[prop(default)]
    pub horizontal: Prop<bool>,
    /// The `Radio`s.
    pub children: Prop<Children>,
}

/// The events of a [`RadioGroup`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum RadioGroupEvent {
    /// A radio was pressed or reached by an arrow key: the value it asks
    /// for, to write back to `value`.
    #[args(value)]
    Change(SharedString),
}

/// A set of radios, one stop of Tab at the checked one: the arrows move
/// around the set and check the radio they land on, Space checks the
/// focused one.
#[component]
pub fn RadioGroup(props: RadioGroupProps, cx: Scope<RadioGroupEvent>) -> impl IntoElement {
    provide_context(Radios {
        value: props.value,
        disabled: props.disabled,
        choose: Rc::new(move |value| cx.emit(RadioGroupEvent::Change(value))),
    });
    let horizontal = props.horizontal;
    ui::radio::radio_group()
        .accessibility_label(move || props.accessibility_label.get())
        .style_with(move |s| match horizontal.get() {
            true => s.flex_row().flex_wrap().gap(px(tokens::SPACE_3)),
            false => s,
        })
        .child(props.children)
}

/// The props of [`Radio`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct RadioProps {
    /// The app's id of the option: the group's `value` while checked.
    pub value: Prop<SharedString>,
    /// The visible label, which screen readers say.
    pub label: Prop<Option<SharedString>>,
    /// What screen readers say when there is no label.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// No presses, skipped by the arrow keys, dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
}

/// One option of a `RadioGroup`: a square box with a square dot while
/// checked, and its label.
#[component]
pub fn Radio(props: RadioProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let group = use_context::<Radios>();
    let (value, disabled) = (props.value, props.disabled);
    let (checked_in, disabled_in, choose) = match group {
        Some(Radios {
            value: chosen,
            disabled: all,
            choose,
        }) => (Some(chosen), Some(all), Some(choose)),
        None => (None, None, None),
    };
    radio_control()
        .label(move || props.label.get().unwrap_or_default())
        .accessibility_label(move || props.accessibility_label.get())
        .checked(move || checked_in.is_some_and(|c| c.get().as_ref() == Some(&value.get())))
        .disabled(move || disabled.get() || disabled_in.is_some_and(|d| d.get()))
        .on_change(move |_: &bool| {
            if let Some(choose) = &choose {
                choose(value.get_untracked());
            }
        })
}

/// A function that formats a slider's value for showing (TS `(value:
/// number) => string`), called in the next JS turn whenever the value or
/// the function changes.
#[derive(Clone)]
pub struct ValueFormat(Callback<f64, SharedString>);

impl fmt::Debug for ValueFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ValueFormat")
    }
}

/// Functions are told apart by what they answer, so one set again (an
/// arrow function in JSX is a new one at each render) is called again.
impl PartialEq for ValueFormat {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl JsValue for ValueFormat {
    fn ts_type(_types: &mut TsTypes) -> String {
        "((value: number) => string)".into()
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        match value.kind() {
            JsKind::Function => {
                <Callback<f64, SharedString> as Arg>::from_js(value).map(ValueFormat)
            }
            _ => Err(value.expected("a function of the value")),
        }
    }

    fn to_js<'a>(&self, host: &'a dyn JsHost) -> Result<JsOwned<'a>, PropError> {
        // A function stays the app's: it is not handed back.
        Ok(JsOwned::new(host, host.undefined()))
    }
}

/// The props of [`Slider`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SliderProps {
    /// A number, or `[start, end]` for a range with two thumbs (controlled).
    pub value: Prop<SliderValue>,
    /// The lowest value (0 when left out).
    #[prop(default)]
    pub min: Prop<f64>,
    /// The highest value (100 when left out).
    #[prop(default = 100.0)]
    pub max: Prop<f64>,
    /// The step the value snaps to (1 when left out).
    #[prop(default = 1.0)]
    pub step: Prop<f64>,
    /// No changes; the fill and the thumb greyed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers say for the thumb (a range's end thumb): what
    /// the value is.
    pub accessibility_label: Prop<SharedString>,
    /// What screen readers say for a range's start thumb.
    pub start_accessibility_label: Prop<Option<SharedString>>,
    /// Shows the value after the track, in tabular figures.
    #[prop(default)]
    pub show_value: Prop<bool>,
    /// Formats the shown value (the number when left out).
    pub format: Prop<Option<ValueFormat>>,
}

/// The events of a [`Slider`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum SliderEvent {
    /// Every value the user moves through: write it back to `value`.
    #[args(value)]
    Change(SliderValue),
    /// The value the user's change ended at.
    #[args(value)]
    ChangeEnd(SliderValue),
}

/// A value as JS's `String(value)` writes it (`40`, `40.5`).
fn number_text(value: f64) -> String {
    format!("{value}")
}

/// A value, or a range's ends, as text: `40`, `20 – 80`.
fn value_text(value: SliderValue, each: impl Fn(f64) -> String) -> SharedString {
    match value {
        SliderValue::Single(v) => each(v).into(),
        SliderValue::Range(a, b) => format!("{} – {}", each(a), each(b)).into(),
    }
}

/// An inset rail with an accent fill up to a square thumb. Keys on the
/// focused thumb: Left/Down and Right/Up step, PageUp/PageDown jump,
/// Home/End go to the ends; a press on the rail moves the nearer thumb
/// there.
#[component]
pub fn Slider(props: SliderProps, cx: Scope<SliderEvent>) -> impl IntoElement {
    let shown = signal(SharedString::default());
    // Answers from an older value or function are dropped.
    let run = Rc::new(Cell::new(0u64));
    effect(move || {
        let value = props.value.get();
        run.set(run.get() + 1);
        match props.format.get() {
            None => shown.set(value_text(value, number_text)),
            Some(ValueFormat(f)) => {
                let (this, run) = (run.get(), run.clone());
                let ends: Vec<_> = match value {
                    SliderValue::Single(v) => vec![v],
                    SliderValue::Range(a, b) => vec![a, b],
                };
                let answers: Vec<_> = ends.iter().map(|&v| f.call_async(v)).collect();
                spawn(async move {
                    let mut texts = Vec::with_capacity(answers.len());
                    for answer in answers {
                        texts.push(answer.await.unwrap_or_default());
                    }
                    if run.get() == this {
                        shown.set(match &texts[..] {
                            [a, b] => format!("{a} – {b}").into(),
                            [a] => a.clone(),
                            _ => SharedString::default(),
                        });
                    }
                });
            }
        }
    });
    let disabled = props.disabled;
    ui::slider::slider()
        .value(move || Some(props.value.get()))
        .min(props.min)
        .max(props.max)
        .step(props.step)
        .disabled(disabled)
        .accessibility_label(move || Some(props.accessibility_label.get()))
        .start_accessibility_label(move || props.start_accessibility_label.get())
        .on_change(move |value: &SliderValue| cx.emit(SliderEvent::Change(*value)))
        .on_change_end(move |value: &SliderValue| cx.emit(SliderEvent::ChangeEnd(*value)))
        .text_size(px(tokens::TEXT_MD))
        .child(when(props.show_value, move || {
            text(move || shown.get())
                .min_w(px(64.0))
                .text_right()
                .tabular_nums()
                .text_color(move || {
                    if disabled.get() {
                        tokens::disabled()
                    } else {
                        tokens::bright()
                    }
                })
        }))
}

/// The color of a [`Progress`]'s fill.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PropEnum)]
pub enum ProgressTone {
    /// The accent.
    #[default]
    Accent,
    /// Done well.
    Success,
    /// Close to a limit.
    Warning,
    /// Failing, or past a limit.
    Danger,
}

impl ProgressTone {
    fn color(self) -> AdaptiveColor {
        match self {
            Self::Accent => tokens::accent(),
            Self::Success => tokens::success(),
            Self::Warning => tokens::warning(),
            Self::Danger => tokens::danger(),
        }
    }
}

/// The props of [`Progress`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ProgressProps {
    /// 0 to 100.
    #[prop(default)]
    pub value: Prop<f32>,
    /// The extent of the work is unknown: a third of the track pulses.
    #[prop(default)]
    pub indeterminate: Prop<bool>,
    /// A label above the bar, which screen readers say.
    pub label: Prop<Option<SharedString>>,
    /// Shows the percentage beside the label.
    #[prop(default)]
    pub show_value: Prop<bool>,
    /// What screen readers say (the label when left out).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The fill's color: the accent (default), or a status.
    #[prop(default)]
    pub tone: Prop<ProgressTone>,
}

/// A thin inset track with a fill that glides to each value.
#[component]
pub fn Progress(props: ProgressProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let ProgressProps {
        value,
        indeterminate,
        label,
        show_value,
        accessibility_label,
        tone,
    } = props;
    let header = move || label.get().is_some() || show_value.get();
    view()
        .gap(px(tokens::SPACE_1))
        .text_size(px(tokens::TEXT_SM))
        .text_color(move || tokens::secondary())
        .child(when(header, move || {
            view()
                .flex_row()
                .justify_between()
                .child(text(move || label.get().unwrap_or_default()))
                .child(when(
                    move || show_value.get() && !indeterminate.get(),
                    move || {
                        text(move || format!("{}%", value.get().round()))
                            .tabular_nums()
                            .text_color(move || tokens::bright())
                    },
                ))
        }))
        .child(
            ui::progress::progress()
                .value(value)
                .indeterminate(indeterminate)
                .accessibility_label(move || accessibility_label.get().or_else(|| label.get()))
                .indicator_class(move || {
                    indeterminate
                        .get()
                        .then(|| SharedString::from("w-1/3 animate-pulse"))
                })
                .indicator_style(move |s| {
                    s.h_full()
                        .bg(move || tone.get().color())
                        .transition()
                        .duration(tokens::DURATION_BASE)
                }),
        )
}

/// Registers this area's components for TSX.
pub(crate) fn register(registry: &mut Registry) {
    registry
        .register::<Checkbox>()
        .register::<Switch>()
        .register::<RadioGroup>()
        .register::<Radio>()
        .register::<Slider>()
        .register::<Progress>();
}
