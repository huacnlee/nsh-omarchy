//! The charts as components TSX renders: `<LineChart>`, `<AreaChart>`,
//! `<BarChart>` and `<PieChart>`, drawn by [`crate::components::chart`].
//!
//! Each takes its points as `data` (`{ label, value }[]`) and further
//! series as `series` (`{ name, values }[]`); a change of any prop draws the
//! chart again from its new values.

use nsh_core::prelude::*;

use crate::components::chart;

/// One point of a chart: a label along x (a slice's name) and its value.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct ChartPoint {
    /// The label along x, or the slice's name.
    pub label: SharedString,
    /// The value.
    pub value: f64,
}

/// A further series of a cartesian chart: a value per point of `data`, in
/// order.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct SeriesData {
    /// The series' name, in the legend and the tooltip.
    pub name: SharedString,
    /// A value per point of `data`.
    pub values: Vec<f64>,
}

/// How a line or an area goes from point to point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum ChartCurve {
    /// Straight segments.
    #[default]
    Linear,
    /// A smooth curve through the points.
    Natural,
    /// Steps: each value held until the next point.
    StepAfter,
}

/// How a value is written: with `decimals` digits after the point (at
/// most 6) and `unit` after it, when either is given.
fn formatter(
    decimals: Option<u32>,
    unit: Option<SharedString>,
) -> Option<impl Fn(f64) -> SharedString + 'static> {
    if decimals.is_none() && unit.is_none() {
        return None;
    }
    Some(move |value: f64| -> SharedString {
        let unit = unit.as_deref().unwrap_or("");
        if !value.is_finite() {
            return "–".into();
        }
        let digits = match decimals {
            Some(d) => d.min(6) as usize,
            None if value.fract() == 0.0 => 0,
            None => 1,
        };
        format!("{value:.digits$}{unit}").into()
    })
}

/// The points as `(label, value)` pairs.
fn pairs(data: Vec<ChartPoint>) -> Vec<(SharedString, f64)> {
    data.into_iter().map(|p| (p.label, p.value)).collect()
}

/// Applies the builder methods the cartesian charts share, from the props
/// of the same names.
macro_rules! cartesian {
    ($chart:expr, $props:expr) => {{
        let props = &$props;
        let mut chart = $chart;
        if let Some(name) = props.name.get() {
            chart = chart.name(name);
        }
        for s in props.series.get() {
            chart = chart.series(s.name, s.values);
        }
        if let Some(label) = props.accessibility_label.get() {
            chart = chart.accessibility_label(label);
        }
        chart = chart
            .height(props.height.get())
            .grid(props.grid.get())
            .x_axis(props.x_axis.get())
            .y_axis(props.y_axis.get())
            .interactive(props.interactive.get());
        if let Some(legend) = props.legend.get() {
            chart = chart.legend(legend);
        }
        if let (Some(min), Some(max)) = (props.y_min.get(), props.y_max.get()) {
            chart = chart.y_domain(min, max);
        }
        if let Some(format) = formatter(props.decimals.get(), props.unit.get()) {
            chart = chart.format(format);
        }
        chart
    }};
}

/// Applies a [`ChartCurve`] to a line or an area chart.
macro_rules! curve {
    ($chart:expr, $curve:expr) => {
        match $curve {
            ChartCurve::Linear => $chart.linear(),
            ChartCurve::Natural => $chart.natural(),
            ChartCurve::StepAfter => $chart.step_after(),
        }
    };
}

// ---- Line -----------------------------------------------------------------

/// The props of [`LineChart`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct LineChartProps {
    /// The points, in order along x.
    pub data: Prop<Vec<ChartPoint>>,
    /// The first series' name ("Value" when left out).
    pub name: Prop<Option<SharedString>>,
    /// Further series, each a value per point of `data`.
    #[prop(default)]
    pub series: Prop<Vec<SeriesData>>,
    /// What screen readers say of the chart; by default a summary of its
    /// series and range.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The plot's height in points (200 when left out).
    #[prop(default = 200.0)]
    pub height: Prop<f32>,
    /// Horizontal grid lines (true when left out).
    #[prop(default = true)]
    pub grid: Prop<bool>,
    /// The labels along the bottom (true when left out).
    #[prop(default = true)]
    pub x_axis: Prop<bool>,
    /// The values along the left (true when left out).
    #[prop(default = true)]
    pub y_axis: Prop<bool>,
    /// The legend under the plot (shown with two series or more when left
    /// out).
    pub legend: Prop<Option<bool>>,
    /// Follows the pointer and the arrow keys with a tooltip (true when
    /// left out); false draws the marks only, with no focus stop.
    #[prop(default = true)]
    pub interactive: Prop<bool>,
    /// The value at the bottom of the y axis; used with `yMax`.
    pub y_min: Prop<Option<f64>>,
    /// The value at the top of the y axis; used with `yMin`.
    pub y_max: Prop<Option<f64>>,
    /// Digits after the point in the tooltip, the summary and the y axis.
    pub decimals: Prop<Option<u32>>,
    /// Written after each value ("%", " ms").
    pub unit: Prop<Option<SharedString>>,
    /// How the line goes from point to point ("linear" when left out).
    #[prop(default)]
    pub curve: Prop<ChartCurve>,
    /// A square dot on every point (false when left out).
    #[prop(default)]
    pub dots: Prop<bool>,
}

/// A line per series over labels along x, in the theme's chart colors,
/// with a tooltip that follows the pointer and the arrow keys.
#[component]
pub fn LineChart(props: LineChartProps) -> impl IntoElement {
    dynamic(move || {
        let chart = cartesian!(chart::line_chart(pairs(props.data.get())), props);
        curve!(chart, props.curve.get()).dots(props.dots.get())
    })
}

// ---- Area -----------------------------------------------------------------

/// The props of [`AreaChart`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AreaChartProps {
    /// The points, in order along x.
    pub data: Prop<Vec<ChartPoint>>,
    /// The first series' name ("Value" when left out).
    pub name: Prop<Option<SharedString>>,
    /// Further series, each a value per point of `data`.
    #[prop(default)]
    pub series: Prop<Vec<SeriesData>>,
    /// What screen readers say of the chart; by default a summary of its
    /// series and range.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The plot's height in points (200 when left out).
    #[prop(default = 200.0)]
    pub height: Prop<f32>,
    /// Horizontal grid lines (true when left out).
    #[prop(default = true)]
    pub grid: Prop<bool>,
    /// The labels along the bottom (true when left out).
    #[prop(default = true)]
    pub x_axis: Prop<bool>,
    /// The values along the left (true when left out).
    #[prop(default = true)]
    pub y_axis: Prop<bool>,
    /// The legend under the plot (shown with two series or more when left
    /// out).
    pub legend: Prop<Option<bool>>,
    /// Follows the pointer and the arrow keys with a tooltip (true when
    /// left out); false draws the marks only, with no focus stop.
    #[prop(default = true)]
    pub interactive: Prop<bool>,
    /// The value at the bottom of the y axis; used with `yMax`.
    pub y_min: Prop<Option<f64>>,
    /// The value at the top of the y axis; used with `yMin`.
    pub y_max: Prop<Option<f64>>,
    /// Digits after the point in the tooltip, the summary and the y axis.
    pub decimals: Prop<Option<u32>>,
    /// Written after each value ("%", " ms").
    pub unit: Prop<Option<SharedString>>,
    /// How the edge goes from point to point ("linear" when left out).
    #[prop(default)]
    pub curve: Prop<ChartCurve>,
    /// Stacks the series, each on the ones before it (false when left out).
    #[prop(default)]
    pub stacked: Prop<bool>,
}

/// A tinted area per series under a line of its color, side by side or
/// stacked, with a tooltip that follows the pointer and the arrow keys.
#[component]
pub fn AreaChart(props: AreaChartProps) -> impl IntoElement {
    dynamic(move || {
        let chart = cartesian!(chart::area_chart(pairs(props.data.get())), props);
        curve!(chart, props.curve.get()).stacked(props.stacked.get())
    })
}

// ---- Bar ------------------------------------------------------------------

/// The props of [`BarChart`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct BarChartProps {
    /// The bars, in order along x.
    pub data: Prop<Vec<ChartPoint>>,
    /// The first series' name ("Value" when left out).
    pub name: Prop<Option<SharedString>>,
    /// Further series, each a value per point of `data`.
    #[prop(default)]
    pub series: Prop<Vec<SeriesData>>,
    /// What screen readers say of the chart; by default a summary of its
    /// series and range.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The plot's height in points (200 when left out).
    #[prop(default = 200.0)]
    pub height: Prop<f32>,
    /// Horizontal grid lines (true when left out).
    #[prop(default = true)]
    pub grid: Prop<bool>,
    /// The labels along the bottom (true when left out).
    #[prop(default = true)]
    pub x_axis: Prop<bool>,
    /// The values along the left (true when left out).
    #[prop(default = true)]
    pub y_axis: Prop<bool>,
    /// The legend under the plot (shown with two series or more when left
    /// out).
    pub legend: Prop<Option<bool>>,
    /// Follows the pointer and the arrow keys with a tooltip (true when
    /// left out); false draws the marks only, with no focus stop.
    #[prop(default = true)]
    pub interactive: Prop<bool>,
    /// The value at the bottom of the y axis; used with `yMax`.
    pub y_min: Prop<Option<f64>>,
    /// The value at the top of the y axis; used with `yMin`.
    pub y_max: Prop<Option<f64>>,
    /// Digits after the point in the tooltip, the summary and the y axis.
    pub decimals: Prop<Option<u32>>,
    /// Written after each value ("%", " ms").
    pub unit: Prop<Option<SharedString>>,
    /// Stacks the series in one bar per label (false when left out).
    #[prop(default)]
    pub stacked: Prop<bool>,
}

/// Square bars per label, side by side per series or stacked, with a
/// tooltip that follows the pointer and the arrow keys.
#[component]
pub fn BarChart(props: BarChartProps) -> impl IntoElement {
    dynamic(move || {
        cartesian!(chart::bar_chart(pairs(props.data.get())), props).stacked(props.stacked.get())
    })
}

// ---- Pie ------------------------------------------------------------------

/// The props of [`PieChart`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PieChartProps {
    /// The slices: each point's label is its name.
    pub data: Prop<Vec<ChartPoint>>,
    /// What screen readers say of the chart; by default its slices and
    /// their shares.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The pie's diameter in points (180 when left out).
    #[prop(default = 180.0)]
    pub size: Prop<f32>,
    /// A donut: the hole's radius as a share of the pie's, 0 to 0.95 (0,
    /// a full pie, when left out).
    #[prop(default)]
    pub donut: Prop<f32>,
    /// The legend of names and shares beside the pie (true when left out).
    #[prop(default = true)]
    pub legend: Prop<bool>,
    /// Follows the pointer and the arrow keys with a tooltip (true when
    /// left out).
    #[prop(default = true)]
    pub interactive: Prop<bool>,
    /// Digits after the point in the tooltip and the summary.
    pub decimals: Prop<Option<u32>>,
    /// Written after each value ("%", " ms").
    pub unit: Prop<Option<SharedString>>,
}

/// Slices of a whole in the theme's chart colors, with a legend of their
/// shares and a tooltip that follows the pointer and the arrow keys.
#[component]
pub fn PieChart(props: PieChartProps) -> impl IntoElement {
    dynamic(move || {
        let mut chart = chart::pie_chart(pairs(props.data.get()))
            .size(props.size.get())
            .donut(props.donut.get())
            .legend(props.legend.get())
            .interactive(props.interactive.get());
        if let Some(label) = props.accessibility_label.get() {
            chart = chart.accessibility_label(label);
        }
        if let Some(format) = formatter(props.decimals.get(), props.unit.get()) {
            chart = chart.format(format);
        }
        chart
    })
}

#[cfg(test)]
mod tests {
    use super::formatter;

    #[test]
    fn values_are_written_with_the_decimals_and_unit_given() {
        assert!(formatter(None, None).is_none());
        let percent = formatter(Some(1), Some("%".into())).unwrap();
        assert_eq!(percent(12.345).as_ref(), "12.3%");
        let ms = formatter(None, Some(" ms".into())).unwrap();
        assert_eq!(ms(40.0).as_ref(), "40 ms");
        assert_eq!(ms(f64::NAN).as_ref(), "–");
    }
}
