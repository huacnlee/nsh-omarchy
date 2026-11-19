//! [`ColorPicker`], on `nsh_base`'s color picker and its parts
//! ([`crate::components::color_picker`]).

use nsh_base::{ColorChannel, ColorPickerEvent as BaseEvent};
use nsh_core::prelude::*;

use super::Value;
use crate::components::color_picker::{
    color_area, color_field, color_picker as picker_look, color_preview, color_slider, color_swatch,
};
use crate::types::Controlled;
use crate::tokens;

/// Width of the editor.
const WIDTH: f32 = 240.0;

/// The props of [`ColorPicker`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ColorPickerProps {
    /// The color: any CSS color in (`"#7aa2f7"`, `"rgb(0 0 0 / 0.5)"`),
    /// `#rrggbbaa` out. Set, the app owns it and writes back what
    /// `onChange` reports; left out, the picker keeps its own (the accent
    /// at first).
    #[prop(default)]
    pub value: Prop<Controlled<Hsla>>,
    /// An opacity slider under the hue.
    #[prop(default)]
    pub alpha: Prop<bool>,
    /// Preset colors under the editor; pressing one picks it.
    #[prop(default)]
    pub swatches: Prop<Vec<Hsla>>,
    /// No changes from any part; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers call the editor (default `"Color"`).
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`ColorPicker`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ColorPickerEvent {
    /// The user changed the color (a drag on the area or a slider, a key,
    /// the hex field, a swatch), as often as it changes during a drag: the
    /// color asked for.
    #[args(value)]
    Change(Hsla),
}

/// A color editor in a bordered surface: saturation and brightness, hue
/// (and opacity with `alpha`), the hex field beside a preview, and the
/// `swatches`. The area's thumb moves with the arrows, a slider's with
/// Left and Right; the field takes any CSS color on Return.
#[component]
pub fn ColorPicker(props: ColorPickerProps, cx: Scope<ColorPickerEvent>) -> impl IntoElement {
    let label = super::spoken(props.accessibility_label);
    let value = Value::new(props.value, color(tokens::accent()));
    let swatches = props.swatches;
    let alpha = props.alpha;
    view()
        .w(px(WIDTH))
        .p(px(tokens::SPACE_3))
        .border_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::surface())
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
        .label(move || label().unwrap_or_else(|| "Color".into()))
        .child(
            picker_look()
                .value(move || Some(value.get()))
                .disabled(move || props.disabled.get())
                .on_change(move |change: &BaseEvent| {
                    if let BaseEvent::Change(next, _) = change {
                        value.request(*next);
                        cx.emit(ColorPickerEvent::Change(*next));
                    }
                })
                .child(color_area())
                .child(color_slider(ColorChannel::Hue).accessibility_label("Hue"))
                .child(dynamic(move || {
                    alpha.get().then(|| {
                        color_slider(ColorChannel::Alpha)
                            .accessibility_label("Opacity")
                            .into_element()
                    })
                }))
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap(px(tokens::SPACE_2))
                        .child(color_preview())
                        .child(color_field().flex_1()),
                )
                .child(dynamic(move || {
                    let colors = swatches.get();
                    (!colors.is_empty()).then(|| {
                        view()
                            .flex_row()
                            .flex_wrap()
                            .gap(px(tokens::SPACE_1))
                            .children(colors.into_iter().map(color_swatch))
                            .into_element()
                    })
                })),
        )
}
