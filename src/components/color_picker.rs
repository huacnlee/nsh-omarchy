//! Omarchy color picker: [`color_picker`] styles `nsh_base`'s
//! `ColorPicker` root, and the parts inside it are styled here too: a
//! saturation and brightness [`color_area`], [`color_slider`]s of a
//! channel (hue, opacity, ...), a hex [`color_field`] and preset
//! [`color_swatch`]es. [`color_picker_panel`] lays them out as one editor.
//!
//! Square everywhere: the area, the tracks, the thumbs and the swatches. A
//! swatch that is the picker's color has an accent border. Keyboard: the
//! area's thumb moves with the arrows, a slider's with Left and Right
//! (Page Up and Down in larger steps), the field takes any CSS color on
//! Return or blur, Space or Return on a swatch picks it.
//!
//! Motion: swatch and field borders are fast color transitions.

use nsh_base::{
    ColorAreaElement, ColorChannel, ColorFieldElement, ColorPickerElement, ColorSliderElement,
    ColorSwatchElement,
};
use nsh_core::element::StyleRefinement;
use nsh_core::prelude::*;

use crate::tokens;

/// Width of the editor panel.
const PANEL_WIDTH: f32 = 240.0;
/// Height of the saturation and brightness area.
const AREA_HEIGHT: f32 = 144.0;
/// Height of a slider's track.
const TRACK: f32 = 12.0;
/// The area's thumb.
const AREA_THUMB: f32 = 12.0;
/// A slider's thumb, across the track.
const SLIDER_THUMB: f32 = 6.0;
/// A swatch's side.
const SWATCH: f32 = 20.0;

/// The picker's root: holds the color its parts show and change. Wire
/// `.value(..)` (an `Hsla`) and `.on_change(..)` to the owner's state, or
/// set `.default_value(..)`; its children are the parts.
pub fn color_picker() -> ColorPickerElement {
    nsh_base::color_picker().gap(px(tokens::SPACE_2))
}

/// The parts laid out as one editor in a bordered surface: the area, the
/// hue and opacity sliders, the hex field beside a preview, and `swatches`
/// (presets) under them.
pub fn color_picker_panel(swatches: Vec<Hsla>) -> ColorPickerElement {
    color_picker()
        .w(px(PANEL_WIDTH))
        .p(px(tokens::SPACE_3))
        .border_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::surface())
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
        .child(color_area())
        .child(color_slider(ColorChannel::Hue).accessibility_label("Hue"))
        .child(color_slider(ColorChannel::Alpha).accessibility_label("Opacity"))
        .child(
            view()
                .flex_row()
                .items_center()
                .gap(px(tokens::SPACE_2))
                .child(color_preview())
                .child(color_field().flex_1()),
        )
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .gap(px(tokens::SPACE_1))
                .children(swatches.into_iter().map(color_swatch)),
        )
}

/// The saturation (across) and brightness (up) of the picker's hue, with a
/// square thumb filled with the color.
pub fn color_area() -> ColorAreaElement {
    nsh_base::color_area()
        .accessibility_label("Saturation and brightness")
        .h(px(AREA_HEIGHT))
        .border_1()
        .border_color(move || tokens::border())
        .panel_style(|s: StyleRefinement| s.w_full().h_full())
        .thumb_style(|s: StyleRefinement| {
            s.size(px(AREA_THUMB))
                .border_2()
                .border_color(move || tokens::bright())
        })
        .fill_thumb(true)
}

/// A slider of one `channel` of the picker's color (HSB's, or the
/// opacity): a square track of the channel's colors over a checkerboard,
/// and a slim thumb filled with the color.
pub fn color_slider(channel: ColorChannel) -> ColorSliderElement {
    nsh_base::color_slider()
        .channel(channel)
        .h(px(TRACK + 4.0))
        .justify_center()
        .track_style(|s: StyleRefinement| {
            s.w_full()
                .h(px(TRACK))
                .border_1()
                .border_color(move || tokens::border())
        })
        .thumb_style(|s: StyleRefinement| {
            s.w(px(SLIDER_THUMB))
                .h(px(TRACK + 4.0))
                .border_1()
                .border_color(move || tokens::bright())
        })
        .fill_thumb(true)
        .checkerboard_color(move || Some(color(tokens::divider())))
}

/// The color as text (hex by default): it reads any CSS color when left or
/// submitted.
pub fn color_field() -> ColorFieldElement {
    nsh_base::color_field()
        .accessibility_label("Hex color")
        .placeholder("#rrggbb")
        .h(px(tokens::CONTROL_MD))
        .px(px(tokens::SPACE_2))
        .border_1()
        .border_color(move || tokens::control_border())
        .bg(move || tokens::normal_fill())
        .font_family(move || tokens::font_mono())
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .focus(|s| s.border_color(move || tokens::focus_border()))
}

/// A preset: pressing it picks `value` in the picker around it; it has an
/// accent border while it is the picker's color.
pub fn color_swatch(value: Hsla) -> ColorSwatchElement {
    nsh_base::color_swatch()
        .color(value)
        .size(px(SWATCH))
        .border_1()
        .border_color(move || tokens::border())
        .selected_style(|s: StyleRefinement| s.border_2().border_color(move || tokens::accent()))
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .focus_visible(|s| s.border_2().border_color(move || tokens::focus_border()))
}

/// The picker's color as a square, a preview (no Tab stop).
pub fn color_preview() -> ColorSwatchElement {
    nsh_base::color_swatch()
        .accessibility_label("Current color")
        .size(px(tokens::CONTROL_MD))
        .border_1()
        .border_color(move || tokens::border())
        .checkerboard_color(move || Some(color(tokens::divider())))
}
