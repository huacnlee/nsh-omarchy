//! [`Pagination`], drawn by [`crate::components::pagination`].

use nsh_core::prelude::*;

use crate::components::pagination::pagination_bar;

/// The props of [`Pagination`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PaginationProps {
    /// The current page, from 1. A press shows the page it asks for and
    /// reports it with `onChange`; a new `page` from the app replaces it.
    pub page: Prop<u32>,
    /// How many pages there are, at least 1.
    pub total_pages: Prop<u32>,
    /// The most pages shown at once, the first and the last included (at
    /// least 5; 7 when left out).
    #[prop(default = 7)]
    pub visible_pages: Prop<u32>,
    /// No item takes a press nor the keyboard focus.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers call it: what it pages through ("Results";
    /// "Pages" when left out).
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`Pagination`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum PaginationEvent {
    /// A page, Previous or Next was pressed: the page it asks for.
    #[args(page)]
    Change(u32),
}

/// Square page cells with Previous and Next at the ends (disabled there);
/// the current page has the selected fill and bright text:
/// `<Pagination page={page()} totalPages={12} onChange={({ page }) => page.set(page)} />`.
#[component]
pub fn Pagination(props: PaginationProps, cx: Scope<PaginationEvent>) -> impl IntoElement {
    let page = props.page;
    let current = signal(page.get_untracked());
    effect(move || {
        let next = page.get();
        untracked(|| current.set(next));
    });
    pagination_bar()
        .self_start()
        .page(current)
        .total_pages(props.total_pages)
        .visible_pages(props.visible_pages)
        .disabled(props.disabled)
        .accessibility_label(move || {
            Some(props.accessibility_label.get().unwrap_or_else(|| "Pages".into()))
        })
        .on_change(move |next: &u32| {
            current.set(*next);
            cx.emit(PaginationEvent::Change(*next));
        })
}
