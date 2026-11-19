//! Charts through the public API: each draws its points, legend and
//! accessibility summary.

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Points, Root, Size};
use nsh_omarchy::{self as om, ChartCurve, ChartPoint, SeriesData};
use nsh_testing::Headless;

const MONTHS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const DESKTOP: [f64; 6] = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
const MOBILE: [f64; 6] = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];

fn months(values: [f64; 6]) -> Vec<ChartPoint> {
    MONTHS.into_iter().zip(values).map(Into::into).collect()
}

fn browsers() -> Vec<ChartPoint> {
    vec![
        ChartPoint::new("Chrome", 275.0),
        ChartPoint::new("Safari", 200.0),
        ChartPoint::new("Firefox", 187.0),
        ChartPoint::new("Edge", 173.0),
        ChartPoint::new("Other", 90.0),
    ]
}

#[component]
fn Charts() -> impl IntoElement {
    om::init();
    view()
        .gap(px(16.0))
        .child(
            om::line_chart(months(DESKTOP))
                .name("Desktop")
                .series(vec![SeriesData::new("Mobile", MOBILE)])
                .curve(ChartCurve::Natural)
                .grid(false),
        )
        .child(
            om::bar_chart(months(DESKTOP))
                .name("Desktop")
                .series(vec![SeriesData::new("Mobile", MOBILE)])
                .stacked(true)
                .unit(" visits")
                .accessibility_label("Desktop visitors per month"),
        )
        .child(
            om::area_chart(months(DESKTOP))
                .name("Desktop")
                .x_axis(false)
                .y_axis(false)
                .legend(Some(false)),
        )
        .child(om::pie_chart(browsers()).donut(0.6).accessibility_label("Visitors by browser"))
}

fn start() -> Headless {
    Headless::start(EngineConfig {
        viewport: Size::new(Points(1000.0), Points(1600.0)),
        ..EngineConfig::new(Root::of::<Charts>())
    })
    .unwrap()
}

#[test]
fn every_chart_draws_its_summary() -> Result<(), String> {
    let mut app = start();
    app.wait_for("label:Line chart of Desktop and Mobile, 6 points, from 73 to 305")?;
    app.wait_for("label:Desktop visitors per month")?;
    app.wait_for("label:Area chart of Desktop, 6 points, from 73 to 305")?;
    app.wait_for("label:Visitors by browser")?;
    // The line chart's legend and its month labels; the pie's shares.
    app.wait_for("text:Mobile")?;
    app.wait_for("text:Mar")?;
    app.wait_for("text:Chrome")?;
    app.wait_for("text:30%")?;
    Ok(())
}

#[component]
fn Pie() -> impl IntoElement {
    om::init();
    view().child(om::pie_chart(browsers()))
}

#[test]
fn a_pie_summarizes_its_shares() -> Result<(), String> {
    let mut app = Headless::start(EngineConfig::new(Root::of::<Pie>())).unwrap();
    app.wait_for("label:Pie chart: Chrome 30%, Safari 22%, Firefox 20%, Edge 19%, Other 10%")?;
    Ok(())
}
