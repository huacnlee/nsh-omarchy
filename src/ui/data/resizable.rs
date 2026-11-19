//! [`ResizablePanelGroup`] and [`ResizablePanel`]: panels side by side or
//! stacked, resized by dragging the lines between them. Over `nsh_base`'s
//! group and panel, as [`crate::components::resizable`] styles them; the
//! panels find their group through `nsh_base`'s context.

use nsh_core::geometry::Axis;
use nsh_core::prelude::*;

use crate::components::resizable as look;

/// The props of [`ResizablePanelGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ResizablePanelGroupProps {
    /// `horizontal` (the default: side by side, dragged left and right) or
    /// `vertical` (stacked, dragged up and down).
    #[prop(default = Axis::Horizontal)]
    pub layout: Prop<Axis>,
    /// Its panels: `ResizablePanel`s.
    pub children: Prop<Children>,
}

/// The events of a [`ResizablePanelGroup`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ResizablePanelGroupEvent {
    /// A drag of a line ended and moved it: every panel's size along the
    /// layout, in points, in order. Pass them back as the panels' `size`
    /// to keep the layout.
    #[args(sizes)]
    Resize(Vec<f32>),
}

/// Panels laid out along `layout` with a 1px line between each two, the
/// accent under the pointer and while dragged. It fills its parent: give
/// the parent a size.
#[component]
pub fn ResizablePanelGroup(
    props: ResizablePanelGroupProps,
    cx: Scope<ResizablePanelGroupEvent>,
) -> impl IntoElement {
    look::resizable()
        .axis(move || props.layout.get())
        .handle_accessibility_label(move || Some(look::handle_label(props.layout.get())))
        .on_resize(move |sizes: &Vec<Points>| {
            cx.emit(ResizablePanelGroupEvent::Resize(sizes.iter().map(|s| s.0).collect()))
        })
        .child(props.children)
}

/// The props of [`ResizablePanel`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ResizablePanelProps {
    /// The size it wants along the group's layout, in points; left out, it
    /// shares what the others leave. A drag gives it the user's size until
    /// a new one is given.
    pub size: Prop<Option<f32>>,
    /// The smallest it gets, in points (default 64).
    #[prop(default = look::PANEL_MIN)]
    pub min_size: Prop<f32>,
    /// The largest it gets, in points (default no limit).
    pub max_size: Prop<Option<f32>>,
    /// Its content.
    pub children: Prop<Children>,
}

/// A panel of a [`ResizablePanelGroup`]: the group sizes it along its
/// layout and stretches it across.
#[component]
pub fn ResizablePanel(props: ResizablePanelProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    look::resizable_panel()
        .size(move || props.size.get().map(px))
        .min_size(move || px(props.min_size.get()))
        .max_size(move || props.max_size.get().map(px))
        .child(props.children)
}

crate::axis_shorthands!(ResizablePanelGroupElement);
