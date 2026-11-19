//! The charts pages. Every chart follows the pointer and the keys (Tab to
//! it, Left / Right step the points, Escape leaves) with a tooltip reading
//! out the point.

use nsh_core::prelude::*;
use nsh_omarchy::{self as om, ChartCurve, ChartPoint, SeriesData, tokens};

use crate::registry::Page;

const MONTHS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const DESKTOP: [f64; 6] = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
const MOBILE: [f64; 6] = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];

/// The six months' points of `values`.
fn months(values: [f64; 6]) -> Vec<ChartPoint> {
    MONTHS.into_iter().zip(values).map(Into::into).collect()
}

/// The mobile series, beside the desktop points.
fn mobile() -> Vec<SeriesData> {
    vec![SeriesData::new("Mobile", MOBILE)]
}

/// A demo: a caption over its chart.
fn section(caption: &'static str, chart: impl IntoElement) -> AnyElement {
    view()
        .gap(px(tokens::SPACE_2))
        .max_w(px(640.0))
        .child(om::caption(caption))
        .child(chart)
        .into_element()
}

fn page(sections: Vec<AnyElement>) -> AnyElement {
    view().gap(px(tokens::SPACE_6)).children(sections).into_element()
}

pub fn line_chart() -> AnyElement {
    page(vec![
        section(
            "One series",
            om::line_chart(months(DESKTOP))
                .name("Desktop")
                .accessibility_label("Desktop visitors, January to June"),
        ),
        section(
            "Two series with dots",
            om::line_chart(months(DESKTOP)).name("Desktop").series(mobile()).dots(true),
        ),
        section(
            "Steps",
            om::line_chart(months(MOBILE))
                .name("Mobile")
                .curve(ChartCurve::StepAfter)
                .height(140.0),
        ),
    ])
}

pub fn bar_chart() -> AnyElement {
    page(vec![
        section(
            "One series",
            om::bar_chart(months(DESKTOP))
                .name("Desktop")
                .accessibility_label("Desktop visitors per month"),
        ),
        section("Grouped", om::bar_chart(months(DESKTOP)).name("Desktop").series(mobile())),
        section(
            "Stacked",
            om::bar_chart(months(DESKTOP))
                .name("Desktop")
                .series(mobile())
                .stacked(true)
                .unit(" visits"),
        ),
    ])
}

pub fn area_chart() -> AnyElement {
    page(vec![
        section(
            "One series",
            om::area_chart(months(DESKTOP))
                .name("Desktop")
                .curve(ChartCurve::Natural)
                .accessibility_label("Desktop visitors over the half year"),
        ),
        section(
            "Stacked",
            om::area_chart(months(DESKTOP)).name("Desktop").series(mobile()).stacked(true),
        ),
    ])
}

pub fn pie_chart() -> AnyElement {
    let browsers = || {
        vec![
            ChartPoint::new("Chrome", 275.0),
            ChartPoint::new("Safari", 200.0),
            ChartPoint::new("Firefox", 187.0),
            ChartPoint::new("Edge", 173.0),
            ChartPoint::new("Other", 90.0),
        ]
    };
    page(vec![
        section("Pie", om::pie_chart(browsers()).accessibility_label("Visitors by browser")),
        section("Donut", om::pie_chart(browsers()).donut(0.6).size(140.0)),
    ])
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Charts", id: "line_chart", title: "Line chart", make: line_chart },
    Page { group: "Charts", id: "bar_chart", title: "Bar chart", make: bar_chart },
    Page { group: "Charts", id: "area_chart", title: "Area chart", make: area_chart },
    Page { group: "Charts", id: "pie_chart", title: "Pie chart", make: pie_chart },
];
