//! [`Card`], [`Panel`], [`Separator`] and [`EmptyState`].

use nsh_core::prelude::*;

use crate::Axis;
use crate::components::icon::{IconName, icon};
use crate::components::surface;
use crate::tokens;

/// A surface's bold title: a screen reader's header.
fn surface_title(title: impl IntoReactive<SharedString>) -> Text {
    text(title)
        .role(AccessibilityRole::Header)
        .text_size(px(tokens::TEXT_LG))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::bright())
}

// ---- Card -----------------------------------------------------------------

/// The props of [`Card`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CardProps {
    /// The card's title, a screen reader's header.
    pub title: Prop<Option<SharedString>>,
    /// A line under the title.
    pub description: Prop<Option<SharedString>>,
    /// The body.
    pub children: Prop<Children>,
    /// Actions under the body, right-aligned.
    pub footer: Prop<Option<Slot>>,
}

/// A raised surface with an optional title, description and footer:
/// `<Card title="Updates" footer={<Button label="Update" />}>..</Card>`.
#[component]
pub fn Card(props: CardProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let title = props.title;
    let description = props.description;
    let footer = props.footer;
    surface::card()
        .child(dynamic(move || {
            let (title, description) = (title.get(), description.get());
            if title.is_none() && description.is_none() {
                return None;
            }
            Some(
                view()
                    .flex_col()
                    .gap(px(tokens::SPACE_1))
                    .children(title.map(|t| surface_title(t).into_element()))
                    .children(description.map(|d| {
                        text(d)
                            .text_size(px(tokens::TEXT_MD))
                            .text_color(move || tokens::secondary())
                            .into_element()
                    }))
                    .into_element(),
            )
        }))
        .child(props.children)
        .child(dynamic(move || {
            footer.get().map(|footer| {
                view()
                    .flex_row()
                    .justify_end()
                    .gap(px(tokens::SPACE_2))
                    .border_t(px(tokens::BORDER))
                    .border_color(move || tokens::divider())
                    .pt(px(tokens::SPACE_3))
                    .slot(Some(footer), |s| s)
                    .into_element()
            })
        }))
}

// ---- Panel ----------------------------------------------------------------

/// A [`Panel`]'s surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PropEnum)]
#[non_exhaustive]
pub enum PanelVariant {
    /// The raised surface color inside its border: the default.
    #[default]
    Fill,
    /// Only the border, over whatever is behind it.
    Outline,
}

/// The props of [`Panel`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PanelProps {
    /// The panel's title, a screen reader's header.
    pub title: Prop<SharedString>,
    /// `fill` (the raised surface, default) or `outline` (the border only).
    #[prop(default)]
    pub variant: Prop<PanelVariant>,
    /// The content, under the title.
    pub children: Prop<Children>,
}

/// A titled section on a square surface: `fill` (default) or `outline`.
#[component]
pub fn Panel(props: PanelProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let variant = props.variant;
    let title = props.title;
    surface::card()
        .bg(move || match variant.get() {
            PanelVariant::Fill => tokens::surface(),
            PanelVariant::Outline => AdaptiveColor::TRANSPARENT,
        })
        .child(surface_title(move || title.get()))
        .child(props.children)
}

// ---- Separator ------------------------------------------------------------

/// The props of [`Separator`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SeparatorProps {
    /// `horizontal` (across its container, default) or `vertical` (down a
    /// row, as tall as the row).
    #[prop(default = Axis::Horizontal)]
    pub layout: Prop<Axis>,
}

/// A 1pt divider: horizontal (default) or vertical.
#[component]
pub fn Separator(props: SeparatorProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let layout = props.layout;
    let vertical = move || layout.get() == Axis::Vertical;
    // One view either way (an app's class and styles land on it): 1pt
    // across the axis, stretched along it by its parent.
    view()
        .flex_shrink_0()
        .self_stretch()
        .w(move || -> Length { if vertical() { px(tokens::BORDER).into() } else { Length::Auto } })
        .h(move || -> Length { if vertical() { Length::Auto } else { px(tokens::BORDER).into() } })
        .bg(move || tokens::divider())
        .accessibility_elements_hidden(true)
}

// ---- EmptyState -----------------------------------------------------------

/// The props of [`EmptyState`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct EmptyStateProps {
    /// What is missing: "No windows".
    pub title: Prop<SharedString>,
    /// A hint at what to do.
    pub description: Prop<Option<SharedString>>,
    /// A glyph above the title, by its Lucide name (`"inbox"`).
    pub icon: Prop<Option<IconName>>,
    /// The actions under the hint (`<Button label="Open" />`), in a row.
    pub children: Prop<Children>,
}

/// What a view shows while it has nothing: an icon, a title, a hint and
/// actions, centered in a dashed frame.
#[component]
pub fn EmptyState(props: EmptyStateProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let title = props.title;
    let description = props.description;
    let glyph = props.icon;
    view()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(tokens::SPACE_2))
        .p(px(tokens::SPACE_4 * 2.0))
        .border(px(tokens::BORDER))
        .border_dashed()
        .border_color(move || tokens::control_border())
        .child(dynamic(move || {
            glyph.get().map(|name| {
                icon(name)
                    .size(px(24.0))
                    .mb(px(tokens::SPACE_1))
                    .text_color(move || tokens::secondary())
                    .into_element()
            })
        }))
        .child(surface_title(move || title.get()))
        .child(dynamic(move || {
            description.get().map(|d| {
                text(d)
                    .text_size(px(tokens::TEXT_MD))
                    .text_center()
                    .text_color(move || tokens::secondary())
                    .into_element()
            })
        }))
        .child(
            view()
                .flex_row()
                .items_center()
                .gap(px(tokens::SPACE_2))
                .mt(px(tokens::SPACE_1))
                .child(props.children),
        )
}
