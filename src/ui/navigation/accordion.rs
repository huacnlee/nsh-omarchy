//! [`Accordion`], [`AccordionItem`] and [`Collapsible`], drawn by
//! [`crate::components::accordion`].

use nsh_core::prelude::*;

use crate::components::accordion::{
    accordion_list, accordion_section_with, collapsible_section_with,
};

/// The props of [`Accordion`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AccordionProps {
    /// The open items' `value`s. Given, the app owns them and writes back
    /// what `onChange` asks for; left out, every item starts closed and the
    /// accordion keeps them itself.
    pub value: Prop<Option<Vec<SharedString>>>,
    /// Several items open at once; without it, opening one closes the one
    /// that was.
    #[prop(default)]
    pub multiple: Prop<bool>,
    /// Every item locked: no trigger takes presses.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The `AccordionItem`s.
    pub children: Prop<Children>,
}

/// The events of an [`Accordion`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum AccordionEvent {
    /// A trigger was pressed: the open items' values as they would be.
    #[args(value)]
    Change(Vec<SharedString>),
}

/// Stacked items that open under their headings with a springing height,
/// the chevron turning: `<Accordion><AccordionItem value="a" title="What
/// is it?">…</AccordionItem></Accordion>`.
#[component]
pub fn Accordion(props: AccordionProps, cx: Scope<AccordionEvent>) -> impl IntoElement {
    let own = signal(Vec::<SharedString>::new());
    let value = props.value;
    accordion_list()
        .value(move || value.get().unwrap_or_else(|| own.get()))
        .multiple(props.multiple)
        .disabled(props.disabled)
        .on_change(move |next: &Vec<SharedString>| {
            if value.with_untracked(Option::is_none) {
                own.set(next.clone());
            }
            cx.emit(AccordionEvent::Change(next.clone()));
        })
        .child(props.children)
}

/// The props of [`AccordionItem`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AccordionItemProps {
    /// What identifies the item in its accordion's `value`: the app's,
    /// never its title.
    pub value: Prop<SharedString>,
    /// The heading, which its trigger shows.
    pub title: Prop<SharedString>,
    /// Its trigger takes no presses; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The content shown while it is open.
    pub children: Prop<Children>,
}

/// One item of an [`Accordion`]: a heading whose trigger opens and closes
/// the content under it.
#[component]
pub fn AccordionItem(props: AccordionItemProps) -> impl IntoElement {
    accordion_section_with(props.value, props.title, props.children).disabled(props.disabled)
}

/// The props of [`Collapsible`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CollapsibleProps {
    /// The trigger row's text.
    pub title: Prop<SharedString>,
    /// Open or closed. Given, the app owns it and writes back what
    /// `onOpenChange` asks for; left out, it starts closed and the
    /// collapsible keeps it itself.
    pub open: Prop<Option<bool>>,
    /// Its trigger takes no presses; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The content shown while it is open.
    pub children: Prop<Children>,
}

/// The events of a [`Collapsible`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum CollapsibleEvent {
    /// Its trigger was pressed: whether it asks to open.
    #[args(open)]
    OpenChange(bool),
}

/// A framed region under a trigger row (its title and a turning chevron)
/// that opens and closes it with a springing height:
/// `<Collapsible title="Advanced">…</Collapsible>`.
#[component]
pub fn Collapsible(props: CollapsibleProps, cx: Scope<CollapsibleEvent>) -> impl IntoElement {
    let own = signal(false);
    let open_prop = props.open;
    let open = computed(move || open_prop.get().unwrap_or_else(|| own.get()));
    let disabled = props.disabled;
    collapsible_section_with(
        move || open.get(),
        props.title,
        move || disabled.get(),
        move || {
            if disabled.get_untracked() {
                return;
            }
            let next = !open.get_untracked();
            if open_prop.with_untracked(Option::is_none) {
                own.set(next);
            }
            cx.emit(CollapsibleEvent::OpenChange(next));
        },
        props.children,
    )
}
