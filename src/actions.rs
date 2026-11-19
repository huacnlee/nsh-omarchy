//! Actions: buttons, toggles, links.

use nsh_core::element::IntoProp;
use nsh_core::prelude::*;

use crate::ui::{actions, button};
pub use crate::ui::actions::{
    ButtonGroup, ButtonGroupElement, Link, LinkElement, LinkTarget, Toggle, ToggleElement,
    ToggleGroup, ToggleGroupElement,
};
pub use crate::ui::button::{Button, ButtonElement, IconButton, IconButtonElement};

/// A labelled action: `button("Save").primary().on_press(save)`.
pub fn button<M>(label: impl IntoProp<SharedString, M>) -> ButtonElement {
    button::button().label(label)
}

/// A square, icon-only action; `label` is what screen readers say:
/// `icon_button(IconName::Plus, "Add").on_press(add)`.
pub fn icon_button<M>(
    icon: crate::IconName,
    label: impl IntoProp<SharedString, M>,
) -> IconButtonElement {
    button::icon_button().icon(icon).label(label)
}

/// Buttons joined edge to edge, sharing a size:
/// `button_group().child(button("Copy")).child(button("Paste"))`.
pub fn button_group() -> ButtonGroupElement {
    actions::button_group()
}

/// A button that is on or off:
/// `toggle("Bold").checked(move || bold.get()).on_change(move |on: &bool| bold.set(*on))`.
pub fn toggle<M>(label: impl IntoProp<SharedString, M>) -> ToggleElement {
    actions::toggle().label(label)
}

/// A row of toggles named by their `value`s, one stop of Tab:
/// `toggle_group().value(move || marks.get()).child(toggle("Bold").value("bold"))`.
pub fn toggle_group() -> ToggleGroupElement {
    actions::toggle_group()
}

/// Underlined text that goes to `href`: a URL, opened by the system, or a
/// path of the app's with `.open_with(LinkTarget::App)`.
pub fn link<M1, M2>(
    label: impl IntoProp<SharedString, M1>,
    href: impl IntoProp<SharedString, M2>,
) -> LinkElement {
    actions::link().label(label).href(href)
}
