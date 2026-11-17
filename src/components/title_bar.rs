//! Omarchy title bar: the app draws the window's top bar itself, in the
//! theme's background with a hairline under it, in place of the system's.
//!
//! [`title_bar`] hides the system's title bar of the window it renders in,
//! makes its own background drag the window, and reserves the edges the
//! system's window controls keep (macOS's traffic lights, Windows' caption
//! buttons, GTK's window buttons), following full screen. Its children (a
//! title, a search field, actions) sit in a row between those edges and
//! stay interactive. On platforms without desktop windows it is a plain bar.

use nsh_core::desktop::{TitleBarOptions, TitleBarStyle, WindowAttributes};
use nsh_core::element::desktop::{current_window, title_bar as window_title_bar};
use nsh_core::geometry::Points;
use nsh_core::prelude::*;

use crate::tokens;

/// The bar's height: the shell's compact bar, room for a `CONTROL_SM` control.
pub const TITLE_BAR_HEIGHT: f32 = 36.0;

/// The window's title bar, drawn by the app: hides the system's on the
/// window it renders in, drags the window from its background, and lays out
/// its children in a row clear of the system's window controls. Put it
/// first in the window's root.
///
/// ```ignore
/// view()
///     .flex_1()
///     .child(title_bar().child(title_bar_title("Files")).child(view().flex_1()).child(button("Share")))
///     .child(content())
/// ```
pub fn title_bar() -> View {
    let mut options = TitleBarOptions::default();
    options.style = TitleBarStyle::Hidden;
    options.height = Some(Points(TITLE_BAR_HEIGHT));
    let mut attributes = WindowAttributes::default();
    attributes.title_bar = Some(options);
    current_window().set(attributes);
    view()
        .flex_row()
        .flex_shrink_0()
        .items_center()
        .gap(px(tokens::SPACE_2))
        .h(px(TITLE_BAR_HEIGHT))
        // The system's controls keep their edges; the bar's own padding
        // starts after them (zero where there are none).
        .pl(move || px(window_title_bar().left.0 + tokens::SPACE_3))
        .pr(move || px(window_title_bar().right.0 + tokens::SPACE_3))
        .bg(move || tokens::background())
        .border_b(px(tokens::BORDER))
        .border_color(move || tokens::divider())
        .window_drag(true)
}

/// The window's title in a [`title_bar`]: emphasized text at body size.
pub fn title_bar_title(title: impl IntoReactive<SharedString>) -> Text {
    text(title)
        .text_size(px(tokens::TEXT_MD))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::bright())
}
