//! [`Sidebar`], [`SidebarSection`] and [`SidebarItem`]: grouped rows, the
//! current one in the selected fill with bright text.
//!
//! The rows are `nsh_base` tabs in one tab list, so the list is one stop
//! of Tab whose arrow keys (Up / Down, Home / End) move between the items
//! and select them, past disabled ones.

use std::{cell::Cell, rc::Rc};

use nsh_core::prelude::*;

use super::Chosen;
use crate::components::icon::{IconName, icon};
use crate::components::sidebar::section_heading;
use crate::tokens;

/// What a [`SidebarSection`] and a [`SidebarItem`] find of their
/// [`Sidebar`] through context.
#[derive(Clone)]
struct SidebarContext {
    chosen: Chosen,
    request: Rc<dyn Fn(SharedString)>,
    /// How many sections were placed: the first sits closer to the top.
    sections: Rc<Cell<usize>>,
}

/// The props of [`Sidebar`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SidebarProps {
    /// The current item's `value`. Given, the app owns it and writes back
    /// what `onChange` asks for; left out, the first enabled item starts
    /// current and the sidebar keeps it itself.
    pub value: Prop<Option<SharedString>>,
    /// Content above the items: the app's name, a search field.
    pub header: Prop<Option<Slot>>,
    /// Content at the bottom, under the items: an account, settings.
    pub footer: Prop<Option<Slot>>,
    /// What screen readers call the navigation ("Mailboxes").
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The `SidebarSection`s and `SidebarItem`s, top to bottom.
    pub children: Prop<Children>,
}

/// The events of a [`Sidebar`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum SidebarEvent {
    /// An item was pressed or reached with the arrow keys: the `value` it
    /// asks to make current.
    #[args(value)]
    Change(SharedString),
}

/// A navigation sidebar, 208 points wide on the surface color with a
/// border at its end: `<Sidebar value={page()} onChange={({ value }) =>
/// page.set(value)}><SidebarItem value="inbox" label="Inbox" /></Sidebar>`.
/// One stop of Tab; Up / Down and Home / End move between the items and
/// select them.
#[component]
pub fn Sidebar(props: SidebarProps, cx: Scope<SidebarEvent>) -> impl IntoElement {
    let chosen = Chosen::new(props.value);
    provide_context(SidebarContext {
        chosen,
        request: Rc::new(move |value: SharedString| {
            if chosen.request(&value) {
                cx.emit(SidebarEvent::Change(value));
            }
        }),
        sections: Rc::new(Cell::new(0)),
    });
    let list = nsh_base::tabs().gap(px(1.0)).child(props.children);
    view()
        .role(AccessibilityRole::Navigation)
        .optional_label(props.accessibility_label)
        .w(px(208.0))
        .flex_shrink_0()
        .gap(px(tokens::SPACE_2))
        .py(px(tokens::SPACE_2))
        .border_r_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::surface())
        .slot(props.header.get(), |s| s)
        .child(scroll().flex_1().min_h(px(0.0)).child(list))
        .slot(props.footer.get(), |s| s)
}

/// The props of [`SidebarSection`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SidebarSectionProps {
    /// The group's heading.
    pub title: Prop<SharedString>,
    /// The group's `SidebarItem`s.
    pub children: Prop<Children>,
}

/// A titled group of a [`Sidebar`]'s items: `<SidebarSection title="Mail">`.
#[component]
pub fn SidebarSection(props: SidebarSectionProps) -> impl IntoElement {
    let first = use_context::<SidebarContext>().is_none_or(|s| {
        let placed = s.sections.get();
        s.sections.set(placed + 1);
        placed == 0
    });
    let title = props.title;
    view()
        .gap(px(1.0))
        .child(section_heading(move || title.get(), first))
        .child(props.children)
}

/// The props of [`SidebarItem`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SidebarItemProps {
    /// What identifies the item in the sidebar's `value`: the app's, never
    /// its label.
    pub value: Prop<SharedString>,
    /// The row's text, which screen readers say.
    pub label: Prop<SharedString>,
    /// A glyph before the label, by its Lucide name (`"inbox"`).
    pub icon: Prop<Option<IconName>>,
    /// Not choosable: no press, skipped by the arrow keys, dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Content at the row's end: a count, a badge.
    pub suffix: Prop<Option<Slot>>,
}

/// One row of a [`Sidebar`]: `<SidebarItem value="inbox" label="Inbox" />`.
#[component]
pub fn SidebarItem(props: SidebarItemProps) -> impl IntoElement {
    let sidebar = use_context::<SidebarContext>();
    let chosen = sidebar.as_ref().map(|s| s.chosen);
    if let Some(chosen) = chosen
        && !props.disabled.get_untracked()
    {
        chosen.seed(props.value.get_untracked());
    }
    let request = sidebar.map(|s| s.request);
    let (value, label, glyph) = (props.value, props.label, props.icon);
    nsh_base::tab()
        .selected(move || chosen.is_some_and(|c| value.with(|v| c.is(v))))
        .disabled(props.disabled)
        .accessibility_label(move || Some(label.get()))
        .on_press(move |_: &()| {
            if let Some(request) = &request {
                request(value.get_untracked());
            }
        })
        .flex_row()
        .items_center()
        .gap(px(tokens::SPACE_2))
        .h(px(tokens::CONTROL_SM))
        .mx(px(tokens::SPACE_1))
        .px(px(tokens::SPACE_2))
        .text_color(move || tokens::foreground())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| {
            s.bg(move || tokens::hover_fill())
                .outline(px(tokens::FOCUS_BORDER))
                .outline_color(move || tokens::focus_border())
        })
        .selected_style(|s| {
            s.bg(move || tokens::selected_fill())
                .text_color(move || tokens::bright())
        })
        .disabled_style(|s| {
            s.bg(move || AdaptiveColor::TRANSPARENT)
                .text_color(move || tokens::disabled())
        })
        .child(dynamic(move || glyph.get().map(|name| icon(name).into_element())))
        .child(
            text(move || label.get())
                .flex_1()
                .number_of_lines(1)
                .text_size(px(tokens::TEXT_MD)),
        )
        .child(view().flex_row().items_center().slot(props.suffix.get(), |s| s))
}
