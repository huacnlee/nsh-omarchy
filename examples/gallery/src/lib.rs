//! The nsh-omarchy component gallery: a sidebar of every page, grouped, and
//! the page. Built only from `nsh_omarchy`'s public API.
//!
//! Up/Down or j/k move through the sidebar, Home/End jump to its ends.

mod pages;
mod registry;

use nsh_core::prelude::*;
use nsh_omarchy::{self as om, tokens};

/// The gallery's root component.
#[component]
pub fn Gallery() -> impl IntoElement {
    om::init();
    let pages = registry::all();
    let count = pages.len();
    let current = signal(0_usize);
    let step = move |by: isize| {
        let next = (current.get_untracked() as isize + by).clamp(0, count as isize - 1);
        current.set(next as usize);
    };
    on_key("down", move |_| step(1));
    on_key("j", move |_| step(1));
    on_key("up", move |_| step(-1));
    on_key("k", move |_| step(-1));
    on_key("home", move |_| current.set(0));
    on_key("end", move |_| current.set(count - 1));

    view()
        .flex_1()
        .bg(move || tokens::background())
        .text_color(move || tokens::foreground())
        .text_size(px(tokens::TEXT_MD))
        .child(
            view()
                .flex_1()
                .flex_row()
                .min_h(px(0.0))
                .child(sidebar(current, &pages))
                .child(content(current, pages)),
        )
}

fn sidebar(current: Signal<usize>, pages: &[&'static registry::Page]) -> impl IntoElement {
    let mut rows: Vec<AnyElement> = Vec::new();
    for (index, page) in pages.iter().enumerate() {
        rows.push(
            om::button(page.title)
                .ghost()
                .selected(move || current.get() == index)
                .on_press(move || current.set(index))
                .into_element(),
        );
    }
    scroll()
        .w(px(200.0))
        .min_h(px(0.0))
        .border_r_1()
        .border_color(move || tokens::border())
        .child(view().p(px(tokens::SPACE_1)).children(rows))
}

fn content(current: Signal<usize>, pages: Vec<&'static registry::Page>) -> impl IntoElement {
    scroll().flex_1().child(dynamic(move || {
        let page = pages[current.get()];
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
                .test_id(page.id)
                .into_element()
        })
    }))
}
