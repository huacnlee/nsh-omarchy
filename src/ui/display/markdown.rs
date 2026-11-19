//! [`Markdown`]: a document in Omarchy's type, on `nsh_base`'s `TextView`
//! restyled by [`prose_parts`].

use nsh_base::{TextViewLinkPressEvent, text_view};
use nsh_core::prelude::*;

use crate::components::text_view::prose_parts;

/// The props of [`Markdown`].
#[derive(Clone, PartialEq, Props)]
pub struct MarkdownProps {
    /// The document, as CommonMark with GitHub's extensions.
    pub source: Prop<SharedString>,
    /// Whether its text can be selected and copied (true when left out).
    #[prop(default = true)]
    pub selectable: Prop<bool>,
}

/// The events of a [`Markdown`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum MarkdownEvent {
    /// A link was pressed: its URL. Nothing opens it; the app decides.
    LinkPress(SharedString),
}

/// A Markdown document in Omarchy's type: `<Markdown source={notes} />`. It
/// does not scroll; put it in a scroll view for long documents.
#[component]
pub fn Markdown(props: MarkdownProps, cx: Scope<MarkdownEvent>) -> impl IntoElement {
    provide_context(prose_parts());
    text_view()
        .markdown(move || Some(props.source.get()))
        .selectable(props.selectable)
        .on_link_press(move |event: &TextViewLinkPressEvent| {
            cx.emit(MarkdownEvent::LinkPress(SharedString::from(event.url.to_string())))
        })
}
