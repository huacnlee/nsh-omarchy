//! Choices: checkboxes, switches, radios, sliders, progress.

use nsh_core::element::IntoProp;
use nsh_core::prelude::*;

use crate::ui::choices;
pub use crate::ui::choices::{
    Checkbox, CheckboxElement, Progress, ProgressElement, Radio, RadioElement, RadioGroup,
    RadioGroupElement, Slider, SliderElement, Switch, SwitchElement,
};
pub use nsh_base::SliderValue;

/// A square box and its label:
/// `checkbox("Remember me").checked(move || remember.get()).on_change(move |on: &bool| remember.set(*on))`.
pub fn checkbox<M>(label: impl IntoProp<SharedString, M>) -> CheckboxElement {
    choices::checkbox().label(label)
}

/// An on/off switch and its label: `switch("Wi-Fi").on_change(move |on: &bool| ..)`.
pub fn switch<M>(label: impl IntoProp<SharedString, M>) -> SwitchElement {
    choices::switch().label(label)
}

/// A set of [`radio`]s, one of them checked, one stop of Tab:
/// `radio_group().value("pro").child(radio("free", "Free")).child(radio("pro", "Pro"))`.
pub fn radio_group() -> RadioGroupElement {
    choices::radio_group()
}

/// One option of a [`radio_group`]: its `value` (the app's id) and its
/// label.
pub fn radio<M1, M2>(
    value: impl IntoProp<SharedString, M1>,
    label: impl IntoProp<SharedString, M2>,
) -> RadioElement {
    choices::radio().value(value).label(label)
}

/// A value, or a range of two, picked along a rail:
/// `slider().value(SliderValue::Single(40.0)).show_value(true)`.
pub fn slider() -> SliderElement {
    choices::slider()
}

/// A bar of work done, 0 to 100: `progress().value(64.0).label("Downloading")`.
pub fn progress() -> ProgressElement {
    choices::progress()
}
