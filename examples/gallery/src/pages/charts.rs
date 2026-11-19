//! The charts pages.

use nsh_core::prelude::*;

use super::placeholder;
use crate::registry::Page;

pub fn line_chart() -> AnyElement {
    placeholder("line_chart")
}

pub fn bar_chart() -> AnyElement {
    placeholder("bar_chart")
}

pub fn area_chart() -> AnyElement {
    placeholder("area_chart")
}

pub fn pie_chart() -> AnyElement {
    placeholder("pie_chart")
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Charts", id: "line_chart", title: "Line chart", make: line_chart },
    Page { group: "Charts", id: "bar_chart", title: "Bar chart", make: bar_chart },
    Page { group: "Charts", id: "area_chart", title: "Area chart", make: area_chart },
    Page { group: "Charts", id: "pie_chart", title: "Pie chart", make: pie_chart },
];
