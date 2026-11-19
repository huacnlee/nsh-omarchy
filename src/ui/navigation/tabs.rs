//! [`Tabs`] and [`Tab`], drawn by [`crate::components::tabs`].

use std::rc::Rc;

use nsh_core::prelude::*;

use super::Chosen;
use crate::components::button::ControlSize;
use crate::components::icon::{IconName, icon};
use crate::components::tabs::{nav_tab_look, nav_tabs, nav_tabs_segmented};

/// The look of a [`Tabs`] list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PropEnum)]
#[non_exhaustive]
pub enum TabsVariant {
    /// A divider under the row; the selected fill slides behind the
    /// selected tab.
    #[default]
    Underline,
    /// A framed row of segments; the selected fill slides behind the
    /// selected one.
    Segmented,
}

/// What a [`Tab`] finds of its [`Tabs`] through context.
#[derive(Clone)]
struct TabsContext {
    chosen: Chosen,
    size: Prop<ControlSize>,
    request: Rc<dyn Fn(SharedString)>,
}

/// The props of [`Tabs`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TabsProps {
    /// The selected tab's `value`. Given, the app owns it and writes back
    /// what `onChange` asks for; left out, the first enabled tab starts
    /// selected and the list keeps the selection itself.
    pub value: Prop<Option<SharedString>>,
    /// `underline` (a divider under the row, the default) or `segmented`
    /// (a framed row of segments).
    #[prop(default)]
    pub variant: Prop<TabsVariant>,
    /// `small` (24 points tall), `medium` (28, the default) or `large` (32).
    #[prop(default)]
    pub size: Prop<ControlSize>,
    /// What screen readers call the list ("Sections").
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The `Tab`s.
    pub children: Prop<Children>,
}

/// The events of [`Tabs`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum TabsEvent {
    /// A tab was pressed or reached with the arrow keys: the `value` it
    /// asks to select.
    #[args(value)]
    Change(SharedString),
}

/// A tab list, one stop of Tab: Left / Right (and Up / Down, Home / End)
/// move between the tabs and select them as they go, and the selected fill
/// slides to the selected tab. Show the selected tab's content next to it,
/// by `value`: `<Tabs value={tab()} onChange={({ value }) => tab.set(value)}>
/// <Tab value="general" label="General" /></Tabs>`.
#[component]
pub fn Tabs(props: TabsProps, cx: Scope<TabsEvent>) -> impl IntoElement {
    let chosen = Chosen::new(props.value);
    provide_context(TabsContext {
        chosen,
        size: props.size,
        request: Rc::new(move |value: SharedString| {
            if chosen.request(&value) {
                cx.emit(TabsEvent::Change(value));
            }
        }),
    });
    dynamic(move || {
        let list = match props.variant.get() {
            TabsVariant::Segmented => nav_tabs_segmented(),
            _ => nav_tabs(),
        };
        untracked(|| {
            list.accessibility_label(move || props.accessibility_label.get())
                .child(props.children)
        })
    })
}

/// The props of [`Tab`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TabProps {
    /// What identifies the tab in its list's `value`: the app's, never its
    /// label.
    pub value: Prop<SharedString>,
    /// The tab's text, which screen readers say.
    pub label: Prop<SharedString>,
    /// A glyph before the label, by its Lucide name (`"settings"`).
    pub icon: Prop<Option<IconName>>,
    /// Not choosable: no press, skipped by the arrow keys, dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Content after the label: a count, a badge.
    pub suffix: Prop<Option<Slot>>,
}

/// One tab of a [`Tabs`] list: `<Tab value="general" label="General" />`.
#[component]
pub fn Tab(props: TabProps) -> impl IntoElement {
    let tabs = use_context::<TabsContext>();
    let size = tabs.as_ref().map(|t| t.size).unwrap_or_else(|| Prop::new(ControlSize::Medium));
    let chosen = tabs.as_ref().map(|t| t.chosen);
    if let Some(chosen) = chosen
        && !props.disabled.get_untracked()
    {
        chosen.seed(props.value.get_untracked());
    }
    let value = props.value;
    let label = props.label;
    let glyph = props.icon;
    let request = tabs.map(|t| t.request);
    nav_tab_look(
        nsh_base::tab(),
        move || size.get().height(),
        move || size.get().padding(),
    )
    .selected(move || chosen.is_some_and(|c| value.with(|v| c.is(v))))
    .disabled(props.disabled)
    .accessibility_label(move || Some(label.get()))
    .on_press(move |_: &()| {
        if let Some(request) = &request {
            request(value.get_untracked());
        }
    })
    .child(dynamic(move || glyph.get().map(|name| icon(name).into_element())))
    .child(text(move || label.get()).text_size(move || px(size.get().text_size())))
    .child(view().flex_row().items_center().slot(props.suffix.get(), |s| s))
}

crate::sizable!(TabsElement);

impl TabsElement {
    /// A divider under the row, the default: `.variant(TabsVariant::Underline)`.
    pub fn underline(self) -> Self {
        self.variant(TabsVariant::Underline)
    }

    /// A framed row of segments: `.variant(TabsVariant::Segmented)`.
    pub fn segmented(self) -> Self {
        self.variant(TabsVariant::Segmented)
    }
}
