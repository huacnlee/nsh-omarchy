//! [`TitleBar`], drawn by [`crate::components::title_bar`].

use nsh_core::prelude::*;

use crate::components::title_bar::{self as look, title_bar_title};

/// The props of [`TitleBar`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TitleBarProps {
    /// The window's title, first in the bar: emphasized text.
    pub title: Prop<Option<SharedString>>,
    /// What follows the title in the bar (a search field, actions); they
    /// stay interactive.
    pub children: Prop<Children>,
}

/// The window's title bar, drawn by the app: it hides the system's bar of
/// the window it renders in, drags the window from its background and lays
/// its title and children out in a row clear of the system's window
/// controls. Put it first in the window's root: `<TitleBar title="Files">`.
#[component]
pub fn TitleBar(props: TitleBarProps) -> impl IntoElement {
    let title = props.title;
    look::title_bar()
        .child(dynamic(move || {
            title
                .get()
                .map(|t| title_bar_title(t).into_element())
        }))
        .child(props.children)
}
