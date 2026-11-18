//! Omarchy accordions and collapsibles, both controlled.
//!
//! - [`accordion_list`]: `nsh_base::accordion()` with a top border; it holds
//!   the open sections' values (`.value(..)`) and reports the next ones
//!   (`.on_change(..)`); `.multiple(true)` lets several stay open.
//! - [`accordion_section`]: one section: a heading whose trigger toggles
//!   it (Space/Return, a focus outline, hover wash, bright title and a
//!   turned chevron while open) and a panel that reveals with a springing
//!   height (at once under reduced motion).
//! - [`collapsible_section`]: a single region under a trigger, open while
//!   the owner says so.

use nsh_base::{
    AccordionElement, AccordionItemElement, CollapsibleElement, accordion, accordion_header,
    accordion_item, accordion_panel, accordion_trigger, collapsible,
};
use nsh_core::{AccessibilityState, prelude::*};

use crate::tokens;

/// The chevron of a trigger, a quarter turn while its section is open.
const CHEVRON_CLASS: &str = "transition-transform duration-[140ms] group-aria-expanded:rotate-90";

/// A stack of [`accordion_section`]s. Chain `.value(..)` (the open
/// sections' values) and `.on_change(..)`; the owner writes the value.
pub fn accordion_list() -> AccordionElement {
    accordion()
        .border_t_1()
        .border_color(move || tokens::border())
}

/// One section of an [`accordion_list`]: `title` in its header, `body` in
/// its panel, identified by `value`. Chain `.disabled(..)` to lock it.
pub fn accordion_section(
    value: impl Into<SharedString>,
    title: impl Into<SharedString>,
    body: impl IntoElement,
) -> AccordionItemElement {
    let title: SharedString = title.into();
    accordion_item()
        .value(value.into())
        .border_b_1()
        .border_color(move || tokens::border())
        .child(
            accordion_header().child(
                accordion_trigger()
                    .accessibility_label(title.clone())
                    .group()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap(px(tokens::SPACE_2))
                    .h(px(tokens::CONTROL_LG))
                    .px(px(tokens::SPACE_2))
                    .text_color(move || tokens::foreground())
                    .transition_colors()
                    .duration(tokens::DURATION_FAST)
                    .hover(|s| s.bg(move || tokens::hover_fill()))
                    .active(|s| s.bg(move || tokens::pressed_fill()))
                    .focus_visible(|s| {
                        s.outline(px(tokens::FOCUS_BORDER))
                            .outline_color(move || tokens::focus_border())
                    })
                    .expanded_style(|s| s.text_color(move || tokens::bright()))
                    .disabled_style(|s| {
                        s.text_color(move || tokens::disabled())
                            .bg(move || AdaptiveColor::TRANSPARENT)
                    })
                    .child(
                        text(title.clone())
                            .text_size(px(tokens::TEXT_MD))
                            .font_weight(FontWeight::SEMIBOLD),
                    )
                    .child(chevron()),
            ),
        )
        .child(
            accordion_panel()
                .reveal(true)
                .accessibility_label(title)
                .child(panel_body(body)),
        )
}

fn chevron() -> Svg {
    super::icon::indicator(super::icon::IconName::ChevronRight, 14.0)
        .app_class(SharedString::from(CHEVRON_CLASS))
}

fn panel_body(body: impl IntoElement) -> View {
    view()
        .px(px(tokens::SPACE_3))
        .pt(px(tokens::SPACE_1))
        .pb(px(tokens::SPACE_3))
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::secondary())
        .child(body)
}

/// A collapsible region: a trigger row titled `title` that calls
/// `on_toggle`, and `content` below it while `open` holds (revealed with a
/// springing height). The owner holds `open` and flips it in `on_toggle`.
pub fn collapsible_section(
    open: impl Fn() -> bool + Copy + 'static,
    title: impl Into<SharedString>,
    on_toggle: impl Fn() + 'static,
    content: impl IntoElement,
) -> CollapsibleElement {
    let title: SharedString = title.into();
    collapsible()
        .open(open)
        .reveal(true)
        .border_1()
        .border_color(move || tokens::border())
        .content(
            view()
                .border_t_1()
                .border_color(move || tokens::divider())
                .child(panel_body(content).pt(px(tokens::SPACE_2))),
        )
        .child(
            view()
                .role(AccessibilityRole::Button)
                .focusable(true)
                .on_press(on_toggle)
                .label(title.clone())
                .accessibility_state(move || AccessibilityState {
                    expanded: Some(open()),
                    ..AccessibilityState::default()
                })
                .flex_row()
                .items_center()
                .gap(px(tokens::SPACE_2))
                .h(px(tokens::CONTROL_MD))
                .px(px(tokens::SPACE_2))
                .text_color(move || {
                    if open() {
                        tokens::bright()
                    } else {
                        tokens::foreground()
                    }
                })
                .transition_colors()
                .duration(tokens::DURATION_FAST)
                .hover(|s| s.bg(move || tokens::hover_fill()))
                .active(|s| s.bg(move || tokens::pressed_fill()))
                .focus_visible(|s| {
                    s.outline(px(tokens::FOCUS_BORDER))
                        .outline_color(move || tokens::focus_border())
                })
                .child(
                    super::icon::indicator(super::icon::IconName::ChevronRight, 14.0)
                        .rotate(move || deg(if open() { 90.0 } else { 0.0 }))
                        .transition_transform()
                        .duration(tokens::DURATION_BASE),
                )
                .child(
                    text(title)
                        .text_size(px(tokens::TEXT_MD))
                        .font_weight(FontWeight::SEMIBOLD),
                ),
        )
}
