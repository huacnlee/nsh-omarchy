//! Gallery pages of the `charts` group. Each `pub fn` is one page.
//!
//! Every chart follows the pointer and the keys (Tab to it, Left / Right
//! step the rows, Escape leaves) with a tooltip reading out the row.

use nsh_core::prelude::*;

use crate::components::chart as c;
use crate::tokens;

const MONTHS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const DESKTOP: [f64; 6] = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
const MOBILE: [f64; 6] = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];

fn months(values: [f64; 6]) -> impl Iterator<Item = (&'static str, f64)> {
    MONTHS.into_iter().zip(values)
}

/// A demo: a caption over its chart.
fn section(caption: &'static str, chart: impl IntoElement) -> AnyElement {
    view()
        .gap(px(tokens::SPACE_2))
        .max_w(px(640.0))
        .child(
            text(caption)
                .text_size(px(tokens::TEXT_SM))
                .text_color(move || tokens::secondary()),
        )
        .child(chart)
        .into_element()
}

fn page(sections: Vec<AnyElement>) -> AnyElement {
    view()
        .gap(px(tokens::SPACE_6))
        .children(sections)
        .into_element()
}

pub fn line_chart() -> AnyElement {
    page(vec![
        section(
            "One series",
            c::line_chart(months(DESKTOP))
                .name("Desktop")
                .accessibility_label("Desktop visitors, January to June"),
        ),
        section(
            "Two series with dots",
            c::line_chart(months(DESKTOP))
                .name("Desktop")
                .series("Mobile", MOBILE)
                .dots(true),
        ),
        section(
            "Steps",
            c::line_chart(months(MOBILE))
                .name("Mobile")
                .step_after()
                .height(140.0),
        ),
    ])
}

pub fn bar_chart() -> AnyElement {
    page(vec![
        section(
            "One series",
            c::bar_chart(months(DESKTOP))
                .name("Desktop")
                .accessibility_label("Desktop visitors per month"),
        ),
        section(
            "Grouped",
            c::bar_chart(months(DESKTOP))
                .name("Desktop")
                .series("Mobile", MOBILE),
        ),
        section(
            "Stacked",
            c::bar_chart(months(DESKTOP))
                .name("Desktop")
                .series("Mobile", MOBILE)
                .stacked(true)
                .format(|v| format!("{v:.0} visits").into()),
        ),
    ])
}

pub fn area_chart() -> AnyElement {
    page(vec![
        section(
            "One series",
            c::area_chart(months(DESKTOP))
                .name("Desktop")
                .natural()
                .accessibility_label("Desktop visitors over the half year"),
        ),
        section(
            "Stacked",
            c::area_chart(months(DESKTOP))
                .name("Desktop")
                .series("Mobile", MOBILE)
                .stacked(true),
        ),
    ])
}

pub fn pie_chart() -> AnyElement {
    let browsers = [
        ("Chrome", 275.0),
        ("Safari", 200.0),
        ("Firefox", 187.0),
        ("Edge", 173.0),
        ("Other", 90.0),
    ];
    page(vec![
        section(
            "Pie",
            c::pie_chart(browsers).accessibility_label("Visitors by browser"),
        ),
        section("Donut", c::pie_chart(browsers).donut(0.6).size(140.0)),
    ])
}
