//! The component gallery: a sidebar of every page, grouped, and the page.
//!
//! Up/Down or j/k move through the sidebar, Home/End jump to its ends.

pub mod pages;
pub mod registry;

use nsh_base::button;
use nsh_core::prelude::*;

use crate::theme::{Theme, active_theme};
use crate::tokens;
use registry::{PAGES, Page};

/// The gallery's root component.
#[component]
pub fn Gallery() -> impl IntoElement {
    crate::init();
    // One toast stack for every page, provided before the pages are built.
    let toaster = crate::components::toaster();
    let current = signal(0_usize);
    let step = move |by: isize| {
        let next = (current.get_untracked() as isize + by).clamp(0, PAGES.len() as isize - 1);
        current.set(next as usize);
    };
    on_key("down", move |_| step(1));
    on_key("j", move |_| step(1));
    on_key("up", move |_| step(-1));
    on_key("k", move |_| step(-1));
    on_key("home", move |_| current.set(0));
    on_key("end", move |_| current.set(PAGES.len() - 1));

    view()
        .flex_1()
        .bg(move || tokens::background())
        .text_color(move || tokens::foreground())
        .text_size(px(tokens::TEXT_MD))
        .child(top_bar())
        .child(
            view()
                .flex_1()
                .flex_row()
                .min_h(px(0.0))
                .child(sidebar(current))
                .child(content(current)),
        )
        .child(toaster.view())
}

fn top_bar() -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap(px(tokens::SPACE_2))
        .h(px(36.0))
        .px(px(tokens::SPACE_3))
        .border_b_1()
        .border_color(move || tokens::border())
        .child(text("Omarchy").font_weight(FontWeight::SEMIBOLD).text_color(move || tokens::bright()))
        .child(text(move || active_theme().name.clone()).text_color(move || tokens::secondary()))
        .child(view().flex_1())
        .child(bar_button("System", || crate::system_theme::follow_system()))
        .child(bar_button("Dark", || Theme::tokyo_night().apply()))
        .child(bar_button("Light", || Theme::flexoki_light().apply()))
}

fn bar_button(label: &'static str, on_press: impl Fn() + 'static) -> impl IntoElement {
    button()
        .on_press(on_press)
        .h(px(tokens::CONTROL_SM))
        .px(px(tokens::SPACE_2))
        .justify_center()
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| s.border_1().border_color(move || tokens::focus_border()))
        .child(text(label).text_size(px(tokens::TEXT_SM)))
}

fn sidebar(current: Signal<usize>) -> impl IntoElement {
    let mut rows: Vec<AnyElement> = Vec::new();
    let mut last_group = "";
    for (index, page) in PAGES.iter().enumerate() {
        if page.group != last_group {
            last_group = page.group;
            rows.push(
                text(page.group)
                    .px(px(tokens::SPACE_3))
                    .pt(px(tokens::SPACE_3))
                    .pb(px(tokens::SPACE_1))
                    .text_size(px(tokens::TEXT_XS))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(move || tokens::secondary())
                    .into_element(),
            );
        }
        rows.push(sidebar_row(current, index, page).into_element());
    }
    scroll()
        .w(px(200.0))
        .border_r_1()
        .border_color(move || tokens::border())
        .child(view().py(px(tokens::SPACE_1)).children(rows))
}

fn sidebar_row(current: Signal<usize>, index: usize, page: &'static Page) -> impl IntoElement {
    let selected = computed(move || current.get() == index);
    button()
        .on_press(move || current.set(index))
        .accessibility_label(page.title)
        .flex_row()
        .items_center()
        .h(px(tokens::CONTROL_SM))
        .mx(px(tokens::SPACE_1))
        .px(px(tokens::SPACE_2))
        .border_l(px(2.0))
        .border_color(move || AdaptiveColor::TRANSPARENT)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| s.bg(move || tokens::hover_fill()))
        .style_when(selected, |s| {
            s.bg(move || tokens::selected_fill()).border_color(move || tokens::accent())
        })
        .child(
            text(page.title)
                .text_size(px(tokens::TEXT_MD))
                .text_color(move || if selected.get() { tokens::bright() } else { tokens::foreground() }),
        )
}

fn content(current: Signal<usize>) -> impl IntoElement {
    scroll().flex_1().child(dynamic(move || {
        let page = &PAGES[current.get()];
        untracked(|| {
            view()
                .p(px(tokens::SPACE_6))
                .gap(px(tokens::SPACE_4))
                .child(
                    text(page.title)
                        .text_size(px(tokens::TEXT_2XL))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(move || tokens::bright()),
                )
                .child((page.make)())
                .key(page.id)
                .into_element()
        })
    }))
}
