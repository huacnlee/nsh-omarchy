//! Chart pages: each renders its charts with their captions, legends and
//! accessibility summaries.

use nsh_engine::{EngineConfig, Points, Root, Size};
use nsh_testing::Headless;

fn start() -> Headless {
    Headless::start(EngineConfig {
        viewport: Size::new(Points(1200.0), Points(1400.0)),
        ..EngineConfig::new(Root::of::<nsh_omarchy::gallery::Gallery>())
    })
    .unwrap()
}

fn open(app: &mut Headless, title: &str) -> Result<(), String> {
    app.wait_for("Omarchy")?;
    app.press(&format!("label:{title}"))?;
    app.wait_for(&format!("text:{title}"))?;
    Ok(())
}

#[test]
fn line_chart_page_renders() -> Result<(), String> {
    let mut app = start();
    open(&mut app, "Line chart")?;
    app.wait_for("text:Two series with dots")?;
    // The app's summary, and one made from the data.
    app.wait_for("label:Desktop visitors, January to June")?;
    app.wait_for("label:Line chart of Desktop and Mobile, 6 points, from 73 to 305")?;
    // The legend of a two-series chart, and the month labels.
    app.wait_for("text:Mobile")?;
    app.wait_for("text:Mar")?;
    Ok(())
}

#[test]
fn bar_chart_page_renders() -> Result<(), String> {
    let mut app = start();
    open(&mut app, "Bar chart")?;
    app.wait_for("text:Grouped")?;
    app.wait_for("text:Stacked")?;
    app.wait_for("label:Desktop visitors per month")?;
    app.wait_for("label:Bar chart of Desktop and Mobile, 6 points, from 73 visits to 305 visits")?;
    Ok(())
}

#[test]
fn area_chart_page_renders() -> Result<(), String> {
    let mut app = start();
    open(&mut app, "Area chart")?;
    app.wait_for("label:Desktop visitors over the half year")?;
    app.wait_for("label:Area chart of Desktop and Mobile, 6 points, from 73 to 305")?;
    Ok(())
}

#[test]
fn pie_chart_page_renders() -> Result<(), String> {
    let mut app = start();
    open(&mut app, "Pie chart")?;
    app.wait_for("label:Visitors by browser")?;
    app.wait_for("text:Donut")?;
    // The legend: names and shares (275 of 925).
    app.wait_for("text:Chrome")?;
    app.wait_for("text:30%")?;
    app.wait_for("label:Pie chart: Chrome 30%, Safari 22%, Firefox 20%, Edge 19%, Other 10%")?;
    Ok(())
}

#[test]
fn pie_tooltip_reads_the_slice_under_the_pointer() -> Result<(), String> {
    let mut config = EngineConfig {
        viewport: Size::new(Points(1200.0), Points(1400.0)),
        ..EngineConfig::new(Root::of::<nsh_omarchy::gallery::Gallery>())
    };
    // A mouse to hover with: the desktop.
    config.environment.os = nsh_core::platform::Os::MacOs;
    config.environment.idiom = nsh_core::platform::Idiom::Desktop;
    let mut app = Headless::start(config).unwrap();
    open(&mut app, "Pie chart")?;
    // The middle of a whole pie reads as 3 o'clock: inside Chrome, the
    // first slice (0 to 108 degrees).
    app.hover("label:Visitors by browser")?;
    app.wait_for("text:Chrome: 275 (30%)")?;
    Ok(())
}
