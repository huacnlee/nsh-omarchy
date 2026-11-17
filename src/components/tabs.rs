//! Omarchy tabs: a tab list with a sliding indicator, its tabs and panels.
//!
//! `nsh_base::tabs()` owns the keyboard (one Tab stop; Left/Right, Up/Down,
//! Home/End move between tabs and select them) and slides the indicator to
//! the selected tab natively with a spring (at once under reduced motion).
//! This module gives it Omarchy's look:
//!
//! - [`nav_tabs`]: an underline list, a 2px accent bar under the selected tab.
//! - [`nav_tabs_segmented`]: a segmented list, a selected fill behind the tab.
//! - [`nav_tab`]: one tab, secondary text, bright when selected.
//! - [`nav_tab_panel`]: a panel that stays mounted and hides while not shown.
//! - [`NavTabStrip`] (`nav_tab_strip()`): a controlled strip built from labels.
//!
//! States: hover wash and press fill (fast color transition), a visible 1px
//! focus outline, bright text while selected, disabled text when disabled.

use nsh_base::{TabElement, TabsElement, tab, tabs};
use nsh_core::prelude::*;

use crate::tokens;

/// An underline tab list: a divider under the row and a 2px accent bar that
/// slides to the selected tab. Put [`nav_tab`]s in it.
pub fn nav_tabs() -> TabsElement {
    tabs()
        .flex_row()
        .items_end()
        .border_b_1()
        .border_color(move || tokens::divider())
        .indicator_style(|s| s.bottom(px(0.0)).h(px(2.0)).bg(move || tokens::accent()))
}

/// A segmented tab list: a resting fill around the row and a selected fill
/// that slides behind the selected tab. Put [`nav_tab`]s in it.
pub fn nav_tabs_segmented() -> TabsElement {
    tabs()
        .flex_row()
        .self_start()
        .p(px(2.0))
        .gap(px(2.0))
        .bg(move || tokens::normal_fill())
        .border_1()
        .border_color(move || tokens::border())
        .indicator_style(|s| {
            s.top(px(2.0))
                .bottom(px(2.0))
                .bg(move || tokens::selected_fill())
        })
}

/// One tab, labelled `label`. Chain `.selected(..)` and `.on_press(..)`:
/// the app owns which tab is selected.
pub fn nav_tab(label: impl IntoReactive<SharedString>) -> TabElement {
    tab()
        .flex_row()
        .items_center()
        .justify_center()
        .h(px(tokens::CONTROL_MD))
        .px(px(tokens::SPACE_3))
        .text_color(move || tokens::secondary())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| {
            s.bg(move || tokens::hover_fill())
                .text_color(move || tokens::foreground())
        })
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| {
            s.outline(px(tokens::FOCUS_BORDER))
                .outline_color(move || tokens::focus_border())
        })
        .selected_style(|s| s.text_color(move || tokens::bright()))
        .disabled_style(|s| {
            s.text_color(move || tokens::disabled())
                .bg(move || AdaptiveColor::TRANSPARENT)
        })
        .child(text(label).text_size(px(tokens::TEXT_MD)))
}

/// A tab's panel: kept mounted (its scroll offset and drafts survive a
/// switch) and hidden, also from screen readers, while `shown` is false.
pub fn nav_tab_panel(shown: impl Fn() -> bool + 'static) -> View {
    let shown = computed(shown);
    view()
        .role(AccessibilityRole::Region)
        .accessibility_elements_hidden(move || !shown.get())
        .style_when(move || !shown.get(), |s| s.hidden())
}

/// The props of [`NavTabStrip`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct NavTabStripProps {
    /// The tabs' labels, in order.
    #[prop(default)]
    pub labels: Prop<Vec<SharedString>>,
    /// The selected tab's index. The owner holds it and follows `on_change`.
    #[prop(default)]
    pub selected: Prop<u32>,
    /// The segmented look instead of the underline (read once).
    #[prop(default)]
    pub segmented: Prop<bool>,
}

/// The events of a [`NavTabStrip`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum NavTabStripEvent {
    /// A tab was pressed or reached with the arrow keys: its index.
    #[args(index)]
    Change(u32),
}

/// A controlled tab list made from `labels`: shows `selected` and reports
/// the tab the user picks with `on_change`. Show the panels next to it,
/// e.g. with [`nav_tab_panel`].
#[component]
pub fn NavTabStrip(props: NavTabStripProps, cx: Scope<NavTabStripEvent>) -> impl IntoElement {
    let rows = computed(move || {
        props
            .labels
            .get()
            .into_iter()
            .enumerate()
            .map(|(index, label)| (index as u32, label))
            .collect::<Vec<_>>()
    });
    let items = each(
        rows,
        |row: &(u32, SharedString)| row.0,
        move |row, _| {
            let index = row.get_untracked().0;
            nav_tab(move || row.with(|r| r.1.clone()))
                .selected(move || props.selected.get() == index)
                .on_press(move |_: &()| cx.emit(NavTabStripEvent::Change(index)))
        },
    );
    let list = if props.segmented.get_untracked() {
        nav_tabs_segmented()
    } else {
        nav_tabs()
    };
    list.child(items)
}
