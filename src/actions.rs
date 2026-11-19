//! Actions: buttons, toggles, links.

use nsh_core::element::IntoProp;
use nsh_core::prelude::*;

use crate::ui::button;
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
