//! Typography and rich text.
//!
//! The `*_text` constructors are `text(..)` with a role of Omarchy's type
//! scale: compact desktop sizes from [`crate::tokens`], titles in the bright
//! color, secondary text dimmed, code in the monospaced family.
//!
//! [`markdown`] shows a Markdown document through `nsh_base`'s `TextView`,
//! restyled with [`prose_parts`] to the same scale (12pt body instead of the
//! 15pt reading size, square code blocks on the inset surface).

use nsh_base::{TextViewLinkPressEvent, TextViewPart as Part, TextViewParts, text_view};
use nsh_core::element::StyleRefinement;
use nsh_core::prelude::*;

use crate::tokens;

// ---- Typography -----------------------------------------------------------

/// A page heading: `TEXT_2XL`, semibold, bright.
pub fn heading_text(content: impl IntoReactive<SharedString>) -> Text {
    text(content)
        .role(AccessibilityRole::Header)
        .text_size(px(tokens::TEXT_2XL))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::bright())
}

/// A section or surface title: `TEXT_LG`, semibold, bright.
pub fn title_text(content: impl IntoReactive<SharedString>) -> Text {
    text(content)
        .role(AccessibilityRole::Header)
        .text_size(px(tokens::TEXT_LG))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::bright())
}

/// Body copy: `TEXT_MD` in the foreground color.
pub fn body_text(content: impl IntoReactive<SharedString>) -> Text {
    text(content)
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
}

/// The label of a field or a group: `TEXT_SM`, medium, foreground.
pub fn label_text(content: impl IntoReactive<SharedString>) -> Text {
    text(content)
        .text_size(px(tokens::TEXT_SM))
        .font_weight(FontWeight::MEDIUM)
        .text_color(move || tokens::foreground())
}

/// A caption, hint or timestamp: `TEXT_XS` in the secondary color.
pub fn caption_text(content: impl IntoReactive<SharedString>) -> Text {
    text(content)
        .text_size(px(tokens::TEXT_XS))
        .text_color(move || tokens::secondary())
}

/// Inline code: monospaced `TEXT_SM` on the inset surface.
pub fn code_text(content: impl IntoReactive<SharedString>) -> Text {
    text(content)
        .font_family(move || tokens::font_mono())
        .text_size(px(tokens::TEXT_SM))
        .px(px(tokens::SPACE_1))
        .bg(move || tokens::inset())
        .text_color(move || tokens::foreground())
}

// ---- Rich text ------------------------------------------------------------

/// The line height of body text in a text view, a multiple of its size.
const PROSE_LINE: f32 = 1.6;

/// Omarchy's styles for the parts of `nsh_base`'s text views: give them to
/// the text views of a component with `provide_context(prose_parts())`
/// (what [`markdown`] does) to restyle one built by hand.
pub fn prose_parts() -> TextViewParts {
    let heading = |size: f32| {
        move |s: StyleRefinement| {
            s.text_size(px(size))
                .line_height(px(size * 1.3))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || tokens::bright())
        }
    };
    TextViewParts::new()
        .part(Part::Root, |s| {
            s.gap(px(tokens::SPACE_3))
                .text_size(px(tokens::TEXT_MD))
                .line_height(px(tokens::TEXT_MD * PROSE_LINE))
                .text_color(move || tokens::foreground())
        })
        .part(Part::Heading1, heading(tokens::TEXT_2XL))
        .part(Part::Heading2, heading(tokens::TEXT_XL))
        .part(Part::Heading3, heading(tokens::TEXT_LG))
        .part(Part::Heading4, heading(tokens::TEXT_MD))
        .part(Part::Heading5, heading(tokens::TEXT_MD))
        .part(Part::Heading6, heading(tokens::TEXT_SM))
        .part(Part::Link, |s| s.text_color(move || tokens::accent()))
        .part(Part::Strong, |s| s.text_color(move || tokens::bright()))
        .part(Part::Code, |s| {
            s.font_family(move || tokens::font_mono())
                .text_size(px(tokens::TEXT_SM))
                .bg(move || tokens::inset())
        })
        .part(Part::Keyboard, |s| {
            s.font_family(move || tokens::font_mono())
                .text_size(px(tokens::TEXT_XS))
                .bg(move || tokens::inset())
        })
        .part(Part::CodeBlock, |s| {
            s.rounded(px(tokens::RADIUS))
                .border_1()
                .border_color(move || tokens::divider())
                .bg(move || tokens::inset())
        })
        .part(Part::CodeBlockCode, |s| {
            s.font_family(move || tokens::font_mono())
                .text_size(px(tokens::TEXT_SM))
                .line_height(px(tokens::TEXT_SM * PROSE_LINE))
                .p(px(tokens::SPACE_3))
        })
        .part(Part::Blockquote, |s| {
            s.border_color(move || tokens::accent())
                .text_color(move || tokens::secondary())
        })
        .part(Part::Alert, |s| s.rounded(px(tokens::RADIUS)))
        .part(Part::ListMarker, |s| {
            s.text_color(move || tokens::secondary())
        })
        .part(Part::TaskMarker, |s| {
            s.rounded(px(tokens::RADIUS))
                .border_color(move || tokens::control_border())
        })
        .part(Part::TaskMarkerChecked, |s| {
            s.bg(move || tokens::accent())
                .border_color(move || tokens::accent())
                .text_color(move || tokens::on_accent())
        })
        .part(Part::Table, |s| s.rounded(px(tokens::RADIUS)))
        .part(Part::TableRow, |s| {
            s.border_color(move || tokens::divider())
        })
        .part(Part::TableCell, |s| {
            s.text_size(px(tokens::TEXT_MD))
                .line_height(px(tokens::TEXT_MD * PROSE_LINE))
        })
        .part(Part::TableHeaderCell, |s| {
            s.font_weight(FontWeight::SEMIBOLD)
                .text_color(move || tokens::bright())
        })
        .part(Part::Rule, |s| s.border_color(move || tokens::divider()))
        .part(Part::Highlight, |s| s.bg(move || tokens::selected_fill()))
        .part(Part::Footnotes, |s| {
            s.text_size(px(tokens::TEXT_SM))
                .text_color(move || tokens::secondary())
        })
}

/// The props of [`Markdown`].
#[derive(Clone, PartialEq, Props)]
pub struct MarkdownProps {
    /// The document, as CommonMark with GitHub's extensions.
    pub source: Prop<SharedString>,
    /// Whether its text can be selected and copied (default true).
    #[prop(default = true)]
    pub selectable: Prop<bool>,
}

/// The events of a [`Markdown`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum MarkdownEvent {
    /// A link was pressed: its URL. Nothing opens it; the app decides.
    LinkPress(SharedString),
}

/// A Markdown document in Omarchy's type: `markdown().source("# Notes ...")`.
/// It does not scroll; put it in a `scroll` for long documents.
#[component]
pub fn Markdown(props: MarkdownProps, cx: Scope<MarkdownEvent>) -> impl IntoElement {
    provide_context(prose_parts());
    text_view()
        .markdown(move || Some(props.source.get()))
        .selectable(props.selectable)
        .on_link_press(move |event: &TextViewLinkPressEvent| {
            cx.emit(MarkdownEvent::LinkPress(event.url.clone()))
        })
}
