//! `<Icon>`: a glyph of [`crate::components::icon`]'s set for TSX.

use nsh_core::element::ImageSrc;
use nsh_core::prelude::*;

use crate::components::icon::IconName;

/// The props of [`Icon`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct IconProps {
    /// The glyph, by its Lucide name ("chevron-down").
    pub name: Prop<IconName>,
    /// What screen readers say for an icon that stands alone and means
    /// something; left out, the icon is decorative and hidden from them.
    pub label: Prop<Option<SharedString>>,
    /// Its width and height in points (16, the line height of body text,
    /// when left out).
    #[prop(default = 16.0)]
    pub size: Prop<f32>,
}

/// An icon of Omarchy's set, stroked in the inherited text color: tint it
/// with a text class (`class="text-primary"`).
#[component]
pub fn Icon(props: IconProps) -> impl IntoElement {
    svg(move || ImageSrc::svg_markup(&props.name.get().svg_markup()))
        .size(move || px(props.size.get()))
        .flex_shrink_0()
        .accessible(move || props.label.get().is_some())
        .accessibility_elements_hidden(move || props.label.get().is_none())
        .role(AccessibilityRole::Image)
        .label(move || props.label.get().unwrap_or_default())
}

/// The props' starting value before TSX sets `name` (which it must).
impl Default for IconName {
    fn default() -> Self {
        IconName::Info
    }
}
