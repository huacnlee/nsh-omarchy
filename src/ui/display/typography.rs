//! The type ramp: [`Heading`], [`Title`], [`Body`], [`Caption`] and
//! [`Code`], each a text holding its children.

use nsh_core::prelude::*;

use crate::tokens;

/// The props of the type ramp: [`Heading`], [`Title`], [`Body`],
/// [`Caption`] and [`Code`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TypeProps {
    /// The text (strings, and spans of their own).
    pub children: Prop<Children>,
}

/// A text of the ramp holding `children`.
fn type_text(children: Prop<Children>, size: f32) -> Text {
    text(SharedString::default())
        .text_size(px(size))
        .child(children)
}

/// A page's heading: 20pt, bright, a screen reader's header.
#[component]
pub fn Heading(props: TypeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    type_text(props.children, tokens::TEXT_2XL)
        .role(AccessibilityRole::Header)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::bright())
}

/// A section's title: 14pt, bright, a screen reader's header.
#[component]
pub fn Title(props: TypeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    type_text(props.children, tokens::TEXT_LG)
        .role(AccessibilityRole::Header)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::bright())
}

/// Body text: 12pt.
#[component]
pub fn Body(props: TypeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    type_text(props.children, tokens::TEXT_MD).text_color(move || tokens::foreground())
}

/// Captions and hints: 11pt, secondary.
#[component]
pub fn Caption(props: TypeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    type_text(props.children, tokens::TEXT_SM).text_color(move || tokens::secondary())
}

/// Inline code: monospace on the inset surface.
#[component]
pub fn Code(props: TypeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    type_text(props.children, tokens::TEXT_SM)
        .font_family(move || tokens::font_mono())
        .px(px(tokens::SPACE_1))
        .bg(move || tokens::inset())
        .text_color(move || tokens::bright())
}
