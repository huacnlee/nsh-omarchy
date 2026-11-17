//! Omarchy navigation sidebar: grouped rows, the current one carrying the
//! selected fill and bright text, as the gallery's own sidebar.
//!
//! [`NavSidebar`] (`nav_sidebar()`) is controlled: it shows `value` (the
//! current item's id) and reports the item the user picks with
//! `on_change`. Each row is a semantic button (a Tab stop, Space and
//! Return press it, a screen reader hears its label and selected state).
//! While a row has the keyboard focus, Up/Down (and k/j) move to the
//! previous/next enabled item and Home/End to the first/last, selecting it
//! and moving the focus with it; the keys are bound only then, so they never
//! take a key from the rest of the app.

use nsh_core::{AccessibilityState, keymap::When, prelude::*};

use crate::tokens;

/// One item of a [`NavSidebar`].
#[derive(Debug, Clone, Default, PartialEq, JsObject)]
pub struct NavSidebarItem {
    /// What identifies it in `value` and `on_change`.
    pub id: SharedString,
    /// What the row says.
    pub label: SharedString,
    /// A count or tag shown at the row's end.
    pub badge: Option<SharedString>,
    /// Not choosable: no press, skipped by the keys, disabled text.
    pub disabled: bool,
}

impl NavSidebarItem {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            badge: None,
            disabled: false,
        }
    }

    /// Shows `badge` at the row's end.
    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Makes the item not choosable.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// A titled group of a [`NavSidebar`]'s items.
#[derive(Debug, Clone, Default, PartialEq, JsObject)]
pub struct NavSidebarGroup {
    /// The heading over the group (empty: none).
    pub title: SharedString,
    pub items: Vec<NavSidebarItem>,
}

impl NavSidebarGroup {
    pub fn new(title: impl Into<SharedString>, items: Vec<NavSidebarItem>) -> Self {
        Self {
            title: title.into(),
            items,
        }
    }
}

/// The props of [`NavSidebar`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct NavSidebarProps {
    /// The groups, top to bottom.
    #[prop(default)]
    pub groups: Prop<Vec<NavSidebarGroup>>,
    /// The current item's id. The owner holds it and follows `on_change`.
    #[prop(default)]
    pub value: Prop<SharedString>,
    /// What screen readers call the navigation.
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`NavSidebar`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum NavSidebarEvent {
    /// The user picked an item (a press, or the keys): its id.
    #[args(id)]
    Change(SharedString),
}

/// What the keys do.
#[derive(Debug, Clone, Copy)]
enum Step {
    By(isize),
    First,
    Last,
}

const KEYS: [(&str, Step); 6] = [
    ("down", Step::By(1)),
    ("j", Step::By(1)),
    ("up", Step::By(-1)),
    ("k", Step::By(-1)),
    ("home", Step::First),
    ("end", Step::Last),
];

/// A grouped navigation list. Give it a width and let it scroll in a
/// `scroll()` when it can outgrow its place.
#[component]
pub fn NavSidebar(props: NavSidebarProps, cx: Scope<NavSidebarEvent>) -> impl IntoElement {
    // The enabled ids in order, for the keys.
    let order = computed(move || {
        props.groups.with(|groups| {
            groups
                .iter()
                .flat_map(|g| g.items.iter())
                .filter(|i| !i.disabled)
                .map(|i| i.id.clone())
                .collect::<Vec<_>>()
        })
    });
    // Each row's node, to move the keyboard focus with the keys.
    let rows = stored(Vec::<(SharedString, ElementRef)>::new());
    // Which row has the keyboard focus, if any.
    let focused = signal(None::<SharedString>);

    let step = move |step: Step| {
        let order = order.get_untracked();
        if order.is_empty() {
            return;
        }
        let from = focused
            .get_untracked()
            .or_else(|| Some(props.value.get_untracked()))
            .and_then(|id| order.iter().position(|o| *o == id));
        let last = order.len() - 1;
        let to = match (step, from) {
            (Step::First, _) => 0,
            (Step::Last, _) => last,
            (Step::By(by), Some(at)) => (at as isize + by).clamp(0, last as isize) as usize,
            (Step::By(by), None) => {
                if by < 0 {
                    last
                } else {
                    0
                }
            }
        };
        let id = order[to].clone();
        if id != props.value.get_untracked() {
            cx.emit(NavSidebarEvent::Change(id.clone()));
        }
        let node = rows.with(|rows| rows.iter().find(|(r, _)| *r == id).map(|(_, n)| *n));
        if let Some(node) = node {
            node.focus();
        }
    };
    // The keys, bound only while a row has the focus.
    effect(move || {
        if focused.with(Option::is_none) {
            return;
        }
        for (keystroke, action) in KEYS {
            on_key_with(
                keystroke,
                KeyOptions {
                    when: When::NotEditing,
                    repeat: true,
                    ..KeyOptions::default()
                },
                move |_| step(action),
            );
        }
    });

    let list = dynamic(move || {
        let groups = props.groups.get();
        untracked(|| {
            rows.replace(Vec::new());
            let mut children: Vec<AnyElement> = Vec::new();
            for group in groups {
                if !group.title.is_empty() {
                    children.push(heading(group.title).into_element());
                }
                for item in group.items {
                    children.push(row(item, props.value, focused, rows, cx).into_element());
                }
            }
            view().py(px(tokens::SPACE_1)).children(children)
        })
    });

    view()
        .role(AccessibilityRole::Navigation)
        .optional_label(props.accessibility_label)
        .child(list)
}

fn heading(title: SharedString) -> Text {
    text(title)
        .px(px(tokens::SPACE_3))
        .pt(px(tokens::SPACE_3))
        .pb(px(tokens::SPACE_1))
        .text_size(px(tokens::TEXT_XS))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::secondary())
}

fn row(
    item: NavSidebarItem,
    value: Prop<SharedString>,
    focused: Signal<Option<SharedString>>,
    rows: Stored<Vec<(SharedString, ElementRef)>>,
    cx: Scope<NavSidebarEvent>,
) -> impl IntoElement {
    let node = element_ref();
    rows.with(|rows| rows.push((item.id.clone(), node)));
    let id = item.id.clone();
    let selected = computed(move || value.with(|v| *v == id));
    let (pressed, entered, left) = (item.id.clone(), item.id.clone(), item.id.clone());
    let disabled = item.disabled;
    // A semantic button: a Tab stop, Space and Return press it, screen
    // readers hear its label and whether it is the current item.
    view()
        .role(AccessibilityRole::Button)
        .focusable(!disabled)
        .disabled(disabled)
        .label(item.label.clone())
        .accessibility_state(move || AccessibilityState {
            selected: selected.get(),
            disabled,
            ..AccessibilityState::default()
        })
        .element_ref(&node)
        .on_focus(move || focused.set(Some(entered.clone())))
        .on_blur(move || {
            if focused.with(|f| f.as_ref() == Some(&left)) {
                focused.set(None);
            }
        })
        .on_press(move || {
            if !disabled && !selected.get_untracked() {
                cx.emit(NavSidebarEvent::Change(pressed.clone()));
            }
        })
        .flex_row()
        .items_center()
        .gap(px(tokens::SPACE_2))
        .h(px(tokens::CONTROL_SM))
        .mx(px(tokens::SPACE_1))
        .px(px(tokens::SPACE_2))
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| {
            s.outline(px(tokens::FOCUS_BORDER))
                .outline_color(move || tokens::focus_border())
        })
        .style_when(selected, |s| s.bg(move || tokens::selected_fill()))
        .disabled_style(|s| s.bg(move || AdaptiveColor::TRANSPARENT))
        .child(
            text(item.label)
                .flex_1()
                .number_of_lines(1)
                .text_size(px(tokens::TEXT_MD))
                .text_color(move || {
                    if disabled {
                        tokens::disabled()
                    } else if selected.get() {
                        tokens::bright()
                    } else {
                        tokens::foreground()
                    }
                }),
        )
        .child(item.badge.map(|badge| {
            text(badge)
                .font_family(tokens::font_mono())
                .text_size(px(tokens::TEXT_XS))
                .text_color(move || tokens::secondary())
        }))
}
