//! [`Checkbox`], [`Switch`], [`RadioGroup`] / [`Radio`], [`Slider`] and
//! [`Progress`], drawn by [`crate::components`]'s `checkbox`, `switch`,
//! `radio`, `slider` and `progress`.
//!
//! A value prop that is set is the app's: the control asks for a change
//! with `onChange` and draws the value it is given. Left out, the control
//! keeps the value itself.

use std::rc::Rc;

use nsh_base::SliderValue;
use nsh_core::prelude::*;

use crate::components::{
    self as ui, checkbox::checkbox_control, radio::radio_control, switch::switch_control,
};
use crate::tokens;
use crate::types::{Axis, Tone};

/// The props of [`Checkbox`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CheckboxProps {
    /// The visible label, which screen readers say.
    pub label: Prop<SharedString>,
    /// Checked or not. Set, the app owns it: write back what `onChange`
    /// asks for. Left out, the checkbox keeps its own (unchecked at first).
    pub checked: Prop<Option<bool>>,
    /// Neither checked nor unchecked: a dash, and a press asks for checked.
    #[prop(default)]
    pub indeterminate: Prop<bool>,
    /// No presses, dimmed, no interaction styles.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers say in place of the label.
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`Checkbox`] and a [`Switch`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum CheckedEvent {
    /// A press (a click, a tap, Space or Return) asks for this value.
    #[args(checked)]
    Change(bool),
}

/// A boolean value that is the app's when `given`, else kept in `own`:
/// what to draw, and how to ask for `next`.
fn owned_bool(given: Prop<Option<bool>>) -> (Memo<bool>, impl Fn(bool) + Copy + 'static) {
    let own = signal(false);
    let shown = computed(move || given.get().unwrap_or_else(|| own.get()));
    let keep = move |next: bool| {
        if given.get_untracked().is_none() {
            own.set(next);
        }
    };
    (shown, keep)
}

/// A square box with a drawn check (a dash while indeterminate), and its
/// label: `<Checkbox label="Sync" checked={sync()} onChange={({ checked }) => sync.set(checked)} />`.
#[component]
pub fn Checkbox(props: CheckboxProps, cx: Scope<CheckedEvent>) -> impl IntoElement {
    let (checked, keep) = owned_bool(props.checked);
    checkbox_control()
        .label(props.label)
        .accessibility_label(props.accessibility_label)
        .checked(move || checked.get())
        .indeterminate(props.indeterminate)
        .disabled(props.disabled)
        .on_change(move |next: &bool| {
            keep(*next);
            cx.emit(CheckedEvent::Change(*next));
        })
}

/// The props of [`Switch`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SwitchProps {
    /// The visible label, which screen readers say.
    pub label: Prop<SharedString>,
    /// On or off. Set, the app owns it: write back what `onChange` asks
    /// for. Left out, the switch keeps its own (off at first).
    pub checked: Prop<Option<bool>>,
    /// No presses, dimmed, no interaction styles.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers say in place of the label.
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// A square track whose square thumb slides from the start (off) to the end
/// (on) over 140 ms; on fills the track and brightens the thumb.
#[component]
pub fn Switch(props: SwitchProps, cx: Scope<CheckedEvent>) -> impl IntoElement {
    let (checked, keep) = owned_bool(props.checked);
    switch_control()
        .label(props.label)
        .accessibility_label(props.accessibility_label)
        .checked(move || checked.get())
        .disabled(props.disabled)
        .on_change(move |next: &bool| {
            keep(*next);
            cx.emit(CheckedEvent::Change(*next));
        })
}

/// What a [`RadioGroup`] gives its [`Radio`]s, through context.
#[derive(Clone)]
struct Radios {
    value: Memo<Option<SharedString>>,
    disabled: Prop<bool>,
    choose: Rc<dyn Fn(SharedString)>,
}

/// The props of [`RadioGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct RadioGroupProps {
    /// The checked radio's `value`. Set, the app owns it: write back what
    /// `onChange` asks for. Left out, the group keeps its own (none
    /// checked at first).
    pub value: Prop<Option<SharedString>>,
    /// `vertical` (a column, the default) or `horizontal` (a row).
    #[prop(default = Axis::Vertical)]
    pub layout: Prop<Axis>,
    /// Every radio of the set disabled.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers call the set ("Plan").
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The `Radio`s.
    pub children: Prop<Children>,
}

/// The events of a [`RadioGroup`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum RadioGroupEvent {
    /// A radio was pressed or reached by an arrow key: the value it asks
    /// for.
    #[args(value)]
    Change(SharedString),
}

/// A set of radios, one stop of Tab at the checked one: the arrows move
/// around the set and check the radio they land on, Space checks the
/// focused one.
#[component]
pub fn RadioGroup(props: RadioGroupProps, cx: Scope<RadioGroupEvent>) -> impl IntoElement {
    let given = props.value;
    let own = signal(None::<SharedString>);
    let value = computed(move || given.get().or_else(|| own.get()));
    provide_context(Radios {
        value,
        disabled: props.disabled,
        choose: Rc::new(move |next: SharedString| {
            if given.get_untracked().is_none() {
                own.set(Some(next.clone()));
            }
            cx.emit(RadioGroupEvent::Change(next));
        }),
    });
    let layout = props.layout;
    ui::radio::radio_group()
        .accessibility_label(move || props.accessibility_label.get())
        .style_with(move |s| match layout.get() {
            Axis::Horizontal => s.flex_row().flex_wrap().gap(px(tokens::SPACE_3)),
            _ => s,
        })
        .child(props.children)
}

/// The props of [`Radio`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct RadioProps {
    /// The app's id of the option: the group's `value` while checked.
    pub value: Prop<SharedString>,
    /// The visible label, which screen readers say.
    pub label: Prop<SharedString>,
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
    let (chosen, all, choose) = match group {
        Some(Radios {
            value,
            disabled,
            choose,
        }) => (Some(value), Some(disabled), Some(choose)),
        None => (None, None, None),
    };
    radio_control()
        .label(props.label)
        .checked(move || chosen.is_some_and(|c| c.get().as_ref() == Some(&value.get())))
        .disabled(move || disabled.get() || all.is_some_and(|d| d.get()))
        .on_change(move |_: &bool| {
            if let Some(choose) = &choose {
                choose(value.get_untracked());
            }
        })
}

/// The props of [`Slider`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SliderProps {
    /// A number, or `[start, end]` for a range with two thumbs. Set, the
    /// app owns it: write back what `onChange` asks for. Left out, the
    /// slider keeps its own (`min` at first).
    pub value: Prop<Option<SliderValue>>,
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
    /// Shows the value after the track, in tabular figures.
    #[prop(default)]
    pub show_value: Prop<bool>,
    /// What screen readers say for the thumb: what the value is
    /// ("Volume").
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`Slider`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum SliderEvent {
    /// Every value the user moves through.
    #[args(value)]
    Change(SliderValue),
    /// The value the user's change ended at.
    #[args(value)]
    ChangeEnd(SliderValue),
}

/// A value, or a range's ends, as text: `40`, `20 – 80`.
fn value_text(value: SliderValue) -> SharedString {
    match value {
        SliderValue::Single(v) => format!("{v}").into(),
        SliderValue::Range(a, b) => format!("{a} – {b}").into(),
    }
}

/// An inset rail with an accent fill up to a square thumb. Keys on the
/// focused thumb: Left/Down and Right/Up step, PageUp/PageDown jump,
/// Home/End go to the ends; a press on the rail moves the nearer thumb
/// there.
#[component]
pub fn Slider(props: SliderProps, cx: Scope<SliderEvent>) -> impl IntoElement {
    let given = props.value;
    let min = props.min;
    let own = signal(None::<SliderValue>);
    let value = computed(move || {
        given
            .get()
            .or_else(|| own.get())
            .unwrap_or_else(|| SliderValue::Single(min.get()))
    });
    let disabled = props.disabled;
    ui::slider::slider()
        .value(move || Some(value.get()))
        .min(props.min)
        .max(props.max)
        .step(props.step)
        .disabled(disabled)
        .accessibility_label(props.accessibility_label)
        .on_change(move |next: &SliderValue| {
            if given.get_untracked().is_none() {
                own.set(Some(*next));
            }
            cx.emit(SliderEvent::Change(*next));
        })
        .on_change_end(move |next: &SliderValue| cx.emit(SliderEvent::ChangeEnd(*next)))
        .text_size(px(tokens::TEXT_MD))
        .child(when(props.show_value, move || {
            text(move || value_text(value.get()))
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

/// The fill color of a [`Tone`].
fn tone_color(tone: Tone) -> AdaptiveColor {
    match tone {
        Tone::Success => tokens::success(),
        Tone::Warning => tokens::warning(),
        Tone::Danger => tokens::danger(),
        _ => tokens::accent(),
    }
}

/// The props of [`Progress`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ProgressProps {
    /// 0 to 100.
    #[prop(default)]
    pub value: Prop<f64>,
    /// The extent of the work is unknown: a third of the track pulses.
    #[prop(default)]
    pub indeterminate: Prop<bool>,
    /// The fill's color: `accent` (the default), `success`, `warning` or
    /// `danger`.
    #[prop(default)]
    pub tone: Prop<Tone>,
    /// A label above the bar, which screen readers say.
    pub label: Prop<Option<SharedString>>,
    /// Shows the percentage beside the label.
    #[prop(default)]
    pub show_value: Prop<bool>,
}

/// A thin inset track with a fill that glides to each value.
#[component]
pub fn Progress(props: ProgressProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let ProgressProps {
        value,
        indeterminate,
        tone,
        label,
        show_value,
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
                .value(move || value.get() as f32)
                .indeterminate(indeterminate)
                .accessibility_label(move || label.get())
                .indicator_class(move || {
                    indeterminate
                        .get()
                        .then(|| SharedString::from("w-1/3 animate-pulse"))
                })
                .indicator_style(move |s| {
                    s.h_full()
                        .bg(move || tone_color(tone.get()))
                        .transition()
                        .duration(tokens::DURATION_BASE)
                }),
        )
}

impl ProgressElement {
    /// A green fill, for work done well: `.tone(Tone::Success)`.
    pub fn success(self) -> Self {
        self.tone(Tone::Success)
    }
    /// An amber fill, close to a limit: `.tone(Tone::Warning)`.
    pub fn warning(self) -> Self {
        self.tone(Tone::Warning)
    }
    /// A red fill, failing or past a limit: `.tone(Tone::Danger)`.
    pub fn danger(self) -> Self {
        self.tone(Tone::Danger)
    }
}

crate::axis_shorthands!(RadioGroupElement);

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
