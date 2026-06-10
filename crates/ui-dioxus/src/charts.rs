use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Shared chart vocabulary
// ---------------------------------------------------------------------------

/// Accent tone for single-series charts (`Sparkline`, `DonutGauge`).
/// Multi-series charts ignore the tone and cycle the `--ui-chart-1..6`
/// series ramp instead.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartTone {
    #[default]
    Primary,
    Success,
    Warning,
    Danger,
    Info,
    Neutral,
}

impl ChartTone {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Info => "info",
            Self::Neutral => "neutral",
        }
    }
}

/// One named data series for `LineChart` / `BarChart`.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartSeries {
    pub name: String,
    pub points: Vec<f32>,
}

impl ChartSeries {
    pub fn new(name: impl Into<String>, points: Vec<f32>) -> Self {
        Self {
            name: name.into(),
            points,
        }
    }
}

/// Plot rectangle inside the SVG viewBox, in viewBox units.
#[derive(Clone, Copy, Debug, PartialEq)]
struct PlotRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl PlotRect {
    const fn right(self) -> f32 {
        self.x + self.width
    }

    const fn bottom(self) -> f32 {
        self.y + self.height
    }
}

// ---------------------------------------------------------------------------
// Pure geometry helpers (unit-tested)
// ---------------------------------------------------------------------------

/// Min/max over every finite point in every series. `None` when there is no
/// finite data at all, so callers can render an explicit empty state instead
/// of a degenerate axis.
fn series_bounds(series: &[ChartSeries]) -> Option<(f32, f32)> {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for s in series {
        for value in &s.points {
            if value.is_finite() {
                min = min.min(*value);
                max = max.max(*value);
            }
        }
    }
    if min.is_finite() && max.is_finite() {
        Some((min, max))
    } else {
        None
    }
}

/// "Nice number" rounding from Heckbert's axis-labelling algorithm: snaps a
/// raw range to 1/2/5×10ⁿ so tick values land on human-friendly numbers.
fn nice_num(range: f32, round: bool) -> f32 {
    if range <= 0.0 || !range.is_finite() {
        return 1.0;
    }
    let exp = range.log10().floor();
    let magnitude = 10f32.powf(exp);
    let fraction = range / magnitude;
    let nice_fraction = if round {
        if fraction < 1.5 {
            1.0
        } else if fraction < 3.0 {
            2.0
        } else if fraction < 7.0 {
            5.0
        } else {
            10.0
        }
    } else if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice_fraction * magnitude
}

/// Evenly spaced, nicely rounded tick values covering `[min, max]`. The
/// returned ticks may extend slightly past the data so the axis ends on a
/// round number; charts use `ticks.first()/last()` as the plot domain.
fn nice_ticks(min: f32, max: f32, target_count: usize) -> Vec<f32> {
    if !min.is_finite() || !max.is_finite() || target_count < 2 {
        return Vec::new();
    }
    // A flat series still needs a vertical domain to draw within; pad ±1
    // (or ±|value| for large magnitudes) around the constant value.
    let (min, max) = if (max - min).abs() < f32::EPSILON {
        let pad = min.abs().max(1.0);
        (min - pad, max + pad)
    } else {
        (min, max)
    };
    let range = nice_num(max - min, false);
    let step = nice_num(range / (target_count - 1) as f32, true);
    let lo = (min / step).floor() * step;
    let hi = (max / step).ceil() * step;

    let mut ticks = Vec::new();
    let mut value = lo;
    // The 0.5-step epsilon keeps `hi` itself included despite float drift.
    while value <= hi + step * 0.5 {
        // Snap near-zero float residue (e.g. -1.19e-7) to exactly 0.
        ticks.push(if value.abs() < step * 1e-4 {
            0.0
        } else {
            value
        });
        value += step;
    }
    ticks
}

/// Compact tick label: trims trailing `.0` and abbreviates thousands and
/// millions (`1500.0 → "1.5k"`).
fn format_tick(value: f32) -> String {
    fn trim(v: f32) -> String {
        let s = format!("{v:.1}");
        s.strip_suffix(".0").unwrap_or(&s).to_string()
    }
    let abs = value.abs();
    if abs >= 1_000_000.0 {
        format!("{}M", trim(value / 1_000_000.0))
    } else if abs >= 1_000.0 {
        format!("{}k", trim(value / 1_000.0))
    } else {
        trim(value)
    }
}

fn map_x(index: usize, count: usize, plot: PlotRect) -> f32 {
    if count <= 1 {
        return plot.x + plot.width / 2.0;
    }
    plot.x + plot.width * (index as f32) / ((count - 1) as f32)
}

fn map_y(value: f32, min: f32, max: f32, plot: PlotRect) -> f32 {
    let span = (max - min).max(1e-6);
    let t = ((value - min) / span).clamp(0.0, 1.0);
    // SVG y grows downward; the largest value maps to the plot top.
    plot.bottom() - t * plot.height
}

/// SVG path (`M … L …`) through the series points, skipping non-finite
/// values. `None` when fewer than two drawable points remain.
fn line_path(points: &[f32], min: f32, max: f32, plot: PlotRect) -> Option<String> {
    let mut d = String::new();
    let mut drawn = 0usize;
    for (index, value) in points.iter().enumerate() {
        if !value.is_finite() {
            continue;
        }
        let x = map_x(index, points.len(), plot);
        let y = map_y(*value, min, max, plot);
        if drawn == 0 {
            d.push_str(&format!("M{x:.2} {y:.2}"));
        } else {
            d.push_str(&format!(" L{x:.2} {y:.2}"));
        }
        drawn += 1;
    }
    (drawn >= 2).then_some(d)
}

/// Closed variant of [`line_path`] dropping to the plot floor, for area
/// fills under a line.
fn area_path(points: &[f32], min: f32, max: f32, plot: PlotRect) -> Option<String> {
    let open = line_path(points, min, max, plot)?;
    let first_x = points
        .iter()
        .position(|v| v.is_finite())
        .map(|i| map_x(i, points.len(), plot))?;
    let last_x = points
        .iter()
        .rposition(|v| v.is_finite())
        .map(|i| map_x(i, points.len(), plot))?;
    Some(format!(
        "{open} L{:.2} {:.2} L{:.2} {:.2} Z",
        last_x,
        plot.bottom(),
        first_x,
        plot.bottom()
    ))
}

/// Geometry for one bar in a grouped `BarChart`, in viewBox units.
#[derive(Clone, Copy, Debug, PartialEq)]
struct BarGeom {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

/// Which slot of the grouped layout a bar occupies: series N of M, at
/// category index N of M.
#[derive(Clone, Copy, Debug, PartialEq)]
struct BarSlot {
    series_index: usize,
    series_count: usize,
    point_index: usize,
    point_count: usize,
}

/// Grouped-bar layout: each category index owns an equal slice of the plot
/// width; bars for every series share the inner 72% of that slice. Bars are
/// measured from the domain floor (`min`), which `nice_ticks` extends to 0
/// for all-positive data.
fn bar_geometry(slot: BarSlot, value: f32, min: f32, max: f32, plot: PlotRect) -> Option<BarGeom> {
    if !value.is_finite() || slot.point_count == 0 || slot.series_count == 0 {
        return None;
    }
    let group_width = plot.width / slot.point_count as f32;
    let inner_width = group_width * 0.72;
    let bar_width = inner_width / slot.series_count as f32;
    let group_left =
        plot.x + group_width * slot.point_index as f32 + (group_width - inner_width) / 2.0;
    let x = group_left + bar_width * slot.series_index as f32;

    let baseline = map_y(min.max(0.0).min(max), min, max, plot);
    let top = map_y(value, min, max, plot);
    let (y, height) = if top <= baseline {
        (top, baseline - top)
    } else {
        (baseline, top - baseline)
    };
    Some(BarGeom {
        x,
        y,
        width: (bar_width - 1.0).max(1.0),
        height: height.max(0.5),
    })
}

/// Class for the Nth series in a multi-series chart; cycles the six-step
/// series color ramp defined in `ui-styles`.
fn series_class(index: usize) -> String {
    format!("ui-chart-series--{}", index % 6 + 1)
}

/// Visible fraction of a draw-in animation overridden by `progress`, clamped
/// and NaN-safe so capture callers can hand in raw clock samples.
fn clamp_progress(progress: f32) -> f32 {
    if progress.is_finite() {
        progress.clamp(0.0, 1.0)
    } else {
        1.0
    }
}

// ---------------------------------------------------------------------------
// Sparkline
// ---------------------------------------------------------------------------

/// A compact, axis-free trend line. Decorative (`aria-hidden`) when `label`
/// is empty — the surrounding readout is expected to carry the value — and
/// `role="img"` with the label as accessible name otherwise.
#[component]
pub fn Sparkline(
    points: Vec<f32>,
    #[props(default)] label: String,
    #[props(default)] tone: ChartTone,
    #[props(default)] filled: bool,
) -> Element {
    const PLOT: PlotRect = PlotRect {
        x: 1.0,
        y: 2.0,
        width: 98.0,
        height: 28.0,
    };
    let bounds = series_bounds(&[ChartSeries::new("", points.clone())]);
    let path = bounds.and_then(|(min, max)| line_path(&points, min, max, PLOT));
    let area = (filled && path.is_some())
        .then(|| bounds.and_then(|(min, max)| area_path(&points, min, max, PLOT)))
        .flatten();

    let class = format!("ui-sparkline ui-sparkline--{}", tone.class_suffix());
    let decorative = label.is_empty();

    rsx! {
        span {
            class: "{class}",
            role: if decorative { "presentation" } else { "img" },
            "aria-hidden": if decorative { "true" } else { "false" },
            "aria-label": if decorative { "" } else { "{label}" },
            svg {
                view_box: "0 0 100 32",
                preserve_aspect_ratio: "none",
                "aria-hidden": "true",
                if let Some(d) = area {
                    path { class: "ui-sparkline-area", d: "{d}" }
                }
                if let Some(d) = path {
                    path {
                        class: "ui-sparkline-line",
                        d: "{d}",
                        fill: "none",
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// LineChart
// ---------------------------------------------------------------------------

/// A multi-series line chart with nice-number gridlines, an optional legend,
/// and a spring-paced draw-in. The SVG is `role="img"` named by `label`; the
/// underlying numbers are mirrored in a visually-hidden table so screen
/// reader users get the data, not a picture.
///
/// Set `progress` (0.0–1.0) to pin the draw-in deterministically — e.g. from
/// a `Scene` clock sample — instead of letting CSS animate it; `animate:
/// false` renders the settled chart with no motion at all.
#[component]
pub fn LineChart(
    label: String,
    series: Vec<ChartSeries>,
    #[props(default)] x_labels: Vec<String>,
    #[props(default = true)] show_grid: bool,
    #[props(default = true)] show_legend: bool,
    #[props(default)] show_area: bool,
    #[props(default = true)] animate: bool,
    #[props(default)] progress: Option<f32>,
) -> Element {
    const PLOT: PlotRect = PlotRect {
        x: 38.0,
        y: 10.0,
        width: 274.0,
        height: 138.0,
    };
    let bounds = series_bounds(&series);
    let ticks = bounds
        .map(|(min, max)| nice_ticks(min, max, 5))
        .unwrap_or_default();
    let (domain_min, domain_max) = match (ticks.first(), ticks.last()) {
        (Some(first), Some(last)) => (*first, *last),
        _ => (0.0, 1.0),
    };

    let static_offset = progress.map(|p| 1.0 - clamp_progress(p));
    let animated = animate && static_offset.is_none();
    let root_class = format!(
        "ui-chart ui-chart--line{}",
        if animated { " ui-chart--animate" } else { "" }
    );
    let label_count = x_labels.len();

    rsx! {
        figure { class: "{root_class}",
            svg {
                class: "ui-chart-canvas",
                view_box: "0 0 320 180",
                role: "img",
                "aria-label": "{label}",
                if bounds.is_none() {
                    text {
                        class: "ui-chart-empty",
                        x: "160",
                        y: "94",
                        text_anchor: "middle",
                        "No data"
                    }
                }
                if show_grid {
                    g { class: "ui-chart-grid", "aria-hidden": "true",
                        for tick in ticks.iter().copied() {
                            line {
                                x1: "{PLOT.x}",
                                x2: "{PLOT.right()}",
                                y1: "{map_y(tick, domain_min, domain_max, PLOT)}",
                                y2: "{map_y(tick, domain_min, domain_max, PLOT)}",
                            }
                            text {
                                class: "ui-chart-tick",
                                x: "{PLOT.x - 6.0}",
                                y: "{map_y(tick, domain_min, domain_max, PLOT) + 3.0}",
                                text_anchor: "end",
                                "{format_tick(tick)}"
                            }
                        }
                    }
                }
                g { "aria-hidden": "true",
                    for (index, s) in series.iter().enumerate() {
                        if show_area {
                            if let Some(d) = area_path(&s.points, domain_min, domain_max, PLOT) {
                                path { class: "ui-chart-area {series_class(index)}", d: "{d}" }
                            }
                        }
                        if let Some(d) = line_path(&s.points, domain_min, domain_max, PLOT) {
                            path {
                                class: "ui-chart-line {series_class(index)}",
                                d: "{d}",
                                fill: "none",
                                "pathLength": "1",
                                style: match static_offset {
                                    Some(offset) => format!("stroke-dashoffset:{offset:.4}"),
                                    None => format!("animation-delay:{}ms", index * 140),
                                },
                            }
                        }
                    }
                }
                g { class: "ui-chart-x-labels", "aria-hidden": "true",
                    for (index, text_label) in x_labels.iter().enumerate() {
                        text {
                            class: "ui-chart-tick",
                            x: "{map_x(index, label_count, PLOT)}",
                            y: "{PLOT.bottom() + 16.0}",
                            text_anchor: "middle",
                            "{text_label}"
                        }
                    }
                }
            }
            if show_legend && series.len() > 1 {
                figcaption { class: "ui-chart-legend",
                    for (index, s) in series.iter().enumerate() {
                        span { class: "ui-chart-legend-item",
                            span {
                                class: "ui-chart-swatch {series_class(index)}",
                                "aria-hidden": "true",
                            }
                            "{s.name}"
                        }
                    }
                }
            }
            {chart_sr_table(&label, &series, &x_labels)}
        }
    }
}

// ---------------------------------------------------------------------------
// BarChart
// ---------------------------------------------------------------------------

/// A grouped bar chart with a staggered rise-in. Accessibility mirrors
/// `LineChart`: named SVG plus a visually-hidden data table. `progress` pins
/// the rise deterministically for capture; `animate: false` disables motion.
#[component]
pub fn BarChart(
    label: String,
    series: Vec<ChartSeries>,
    #[props(default)] x_labels: Vec<String>,
    #[props(default = true)] show_grid: bool,
    #[props(default = true)] show_legend: bool,
    #[props(default = true)] animate: bool,
    #[props(default)] progress: Option<f32>,
) -> Element {
    const PLOT: PlotRect = PlotRect {
        x: 38.0,
        y: 10.0,
        width: 274.0,
        height: 138.0,
    };
    let bounds = series_bounds(&series);
    // Anchor the domain at zero so bar lengths stay proportional to values.
    let ticks = bounds
        .map(|(min, max)| nice_ticks(min.min(0.0), max.max(0.0), 5))
        .unwrap_or_default();
    let (domain_min, domain_max) = match (ticks.first(), ticks.last()) {
        (Some(first), Some(last)) => (*first, *last),
        _ => (0.0, 1.0),
    };

    let static_scale = progress.map(clamp_progress);
    let animated = animate && static_scale.is_none();
    let root_class = format!(
        "ui-chart ui-chart--bar{}",
        if animated { " ui-chart--animate" } else { "" }
    );
    let point_count = series.iter().map(|s| s.points.len()).max().unwrap_or(0);
    let series_count = series.len();
    let label_count = x_labels.len();

    rsx! {
        figure { class: "{root_class}",
            svg {
                class: "ui-chart-canvas",
                view_box: "0 0 320 180",
                role: "img",
                "aria-label": "{label}",
                if bounds.is_none() {
                    text {
                        class: "ui-chart-empty",
                        x: "160",
                        y: "94",
                        text_anchor: "middle",
                        "No data"
                    }
                }
                if show_grid {
                    g { class: "ui-chart-grid", "aria-hidden": "true",
                        for tick in ticks.iter().copied() {
                            line {
                                x1: "{PLOT.x}",
                                x2: "{PLOT.right()}",
                                y1: "{map_y(tick, domain_min, domain_max, PLOT)}",
                                y2: "{map_y(tick, domain_min, domain_max, PLOT)}",
                            }
                            text {
                                class: "ui-chart-tick",
                                x: "{PLOT.x - 6.0}",
                                y: "{map_y(tick, domain_min, domain_max, PLOT) + 3.0}",
                                text_anchor: "end",
                                "{format_tick(tick)}"
                            }
                        }
                    }
                }
                g { "aria-hidden": "true",
                    for (series_index, s) in series.iter().enumerate() {
                        for (point_index, value) in s.points.iter().enumerate() {
                            if let Some(bar) = bar_geometry(
                                BarSlot {
                                    series_index,
                                    series_count,
                                    point_index,
                                    point_count,
                                },
                                *value,
                                domain_min,
                                domain_max,
                                PLOT,
                            ) {
                                rect {
                                    class: "ui-chart-bar {series_class(series_index)}",
                                    x: "{bar.x}",
                                    y: "{bar.y}",
                                    width: "{bar.width}",
                                    height: "{bar.height}",
                                    rx: "1.5",
                                    style: match static_scale {
                                        Some(scale) => format!("transform:scaleY({scale:.4})"),
                                        None => format!(
                                            "animation-delay:{}ms",
                                            point_index * 40 + series_index * 90
                                        ),
                                    },
                                }
                            }
                        }
                    }
                }
                g { class: "ui-chart-x-labels", "aria-hidden": "true",
                    for (index, text_label) in x_labels.iter().enumerate() {
                        text {
                            class: "ui-chart-tick",
                            x: "{bar_group_center(index, label_count.max(point_count), PLOT)}",
                            y: "{PLOT.bottom() + 16.0}",
                            text_anchor: "middle",
                            "{text_label}"
                        }
                    }
                }
            }
            if show_legend && series.len() > 1 {
                figcaption { class: "ui-chart-legend",
                    for (index, s) in series.iter().enumerate() {
                        span { class: "ui-chart-legend-item",
                            span {
                                class: "ui-chart-swatch {series_class(index)}",
                                "aria-hidden": "true",
                            }
                            "{s.name}"
                        }
                    }
                }
            }
            {chart_sr_table(&label, &series, &x_labels)}
        }
    }
}

/// Horizontal center of a bar group, used to place category labels.
fn bar_group_center(index: usize, count: usize, plot: PlotRect) -> f32 {
    if count == 0 {
        return plot.x + plot.width / 2.0;
    }
    let group_width = plot.width / count as f32;
    plot.x + group_width * (index as f32 + 0.5)
}

// ---------------------------------------------------------------------------
// DonutGauge
// ---------------------------------------------------------------------------

/// A radial gauge sweeping from 12 o'clock. Exposes `role="meter"` with the
/// percentage as `aria-valuenow` and `display_value` as the human-readable
/// `aria-valuetext`. `progress` pins the sweep for deterministic capture.
#[component]
pub fn DonutGauge(
    label: String,
    value: f32,
    #[props(default)] display_value: String,
    #[props(default)] description: String,
    #[props(default)] tone: ChartTone,
    #[props(default = true)] animate: bool,
    #[props(default)] progress: Option<f32>,
) -> Element {
    let value = clamp_progress(value);
    let percent = (value * 100.0).round() as u32;
    let shown_value = if display_value.is_empty() {
        format!("{percent}%")
    } else {
        display_value
    };

    let static_fraction = progress.map(|p| value * clamp_progress(p));
    let animated = animate && static_fraction.is_none();
    let root_class = format!(
        "ui-donut-gauge ui-donut-gauge--{}{}",
        tone.class_suffix(),
        if animated { " ui-chart--animate" } else { "" }
    );
    // pathLength="1" normalizes the circumference, so dasharray "v 1" shows
    // exactly the value fraction regardless of the actual radius.
    let arc_style = match static_fraction {
        Some(fraction) => format!("stroke-dasharray:{fraction:.4} 1"),
        None => format!("--ui-gauge-value:{value:.4}"),
    };

    rsx! {
        div {
            class: "{root_class}",
            role: "meter",
            "aria-label": "{label}",
            "aria-valuemin": "0",
            "aria-valuemax": "100",
            "aria-valuenow": "{percent}",
            "aria-valuetext": "{shown_value}",
            svg {
                class: "ui-donut-gauge-canvas",
                view_box: "0 0 120 120",
                "aria-hidden": "true",
                circle {
                    class: "ui-donut-gauge-track",
                    cx: "60",
                    cy: "60",
                    r: "52",
                    fill: "none",
                }
                circle {
                    class: "ui-donut-gauge-arc",
                    cx: "60",
                    cy: "60",
                    r: "52",
                    fill: "none",
                    transform: "rotate(-90 60 60)",
                    "pathLength": "1",
                    style: "{arc_style}",
                }
            }
            div { class: "ui-donut-gauge-readout", "aria-hidden": "true",
                strong { class: "ui-donut-gauge-value", "{shown_value}" }
                if !description.is_empty() {
                    span { class: "ui-donut-gauge-description", "{description}" }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Screen-reader data table shared by LineChart / BarChart
// ---------------------------------------------------------------------------

fn chart_sr_table(label: &str, series: &[ChartSeries], x_labels: &[String]) -> Element {
    if series.is_empty() {
        return rsx! {};
    }
    let point_count = series.iter().map(|s| s.points.len()).max().unwrap_or(0);
    let columns: Vec<String> = (0..point_count)
        .map(|i| {
            x_labels
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("Point {}", i + 1))
        })
        .collect();
    let rows: Vec<(String, Vec<String>)> = series
        .iter()
        .map(|s| {
            let cells = (0..point_count)
                .map(|i| {
                    s.points
                        .get(i)
                        .filter(|v| v.is_finite())
                        .map(|v| format_tick(*v))
                        .unwrap_or_else(|| "—".to_string())
                })
                .collect();
            (s.name.clone(), cells)
        })
        .collect();

    rsx! {
        table { class: "visually-hidden",
            caption { "{label}" }
            thead {
                tr {
                    th { scope: "col", "Series" }
                    for column in columns.iter() {
                        th { scope: "col", "{column}" }
                    }
                }
            }
            tbody {
                for (name, cells) in rows.iter() {
                    tr {
                        th { scope: "row", "{name}" }
                        for cell in cells.iter() {
                            td { "{cell}" }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLOT: PlotRect = PlotRect {
        x: 10.0,
        y: 10.0,
        width: 100.0,
        height: 100.0,
    };

    /// One series, one category — the simplest grouped-bar slot.
    const SINGLE_SLOT: BarSlot = BarSlot {
        series_index: 0,
        series_count: 1,
        point_index: 0,
        point_count: 1,
    };

    #[test]
    fn series_bounds_skips_non_finite() {
        let series = [ChartSeries::new(
            "a",
            vec![1.0, f32::NAN, 5.0, f32::INFINITY, -2.0],
        )];
        assert_eq!(series_bounds(&series), Some((-2.0, 5.0)));
    }

    #[test]
    fn series_bounds_empty_is_none() {
        assert_eq!(series_bounds(&[]), None);
        assert_eq!(
            series_bounds(&[ChartSeries::new("a", vec![f32::NAN])]),
            None
        );
    }

    #[test]
    fn nice_ticks_cover_domain_with_round_steps() {
        let ticks = nice_ticks(3.0, 97.0, 5);
        assert_eq!(ticks, vec![0.0, 20.0, 40.0, 60.0, 80.0, 100.0]);
    }

    #[test]
    fn nice_ticks_flat_series_pads_domain() {
        let ticks = nice_ticks(5.0, 5.0, 5);
        assert!(ticks.len() >= 2);
        assert!(*ticks.first().unwrap() <= 4.0);
        assert!(*ticks.last().unwrap() >= 6.0);
    }

    #[test]
    fn format_tick_trims_and_abbreviates() {
        assert_eq!(format_tick(40.0), "40");
        assert_eq!(format_tick(2.5), "2.5");
        assert_eq!(format_tick(1_500.0), "1.5k");
        assert_eq!(format_tick(2_000_000.0), "2M");
        assert_eq!(format_tick(-1_500.0), "-1.5k");
    }

    #[test]
    fn map_y_inverts_axis() {
        // Highest value maps to the plot top, lowest to the bottom.
        assert_eq!(map_y(10.0, 0.0, 10.0, PLOT), PLOT.y);
        assert_eq!(map_y(0.0, 0.0, 10.0, PLOT), PLOT.bottom());
    }

    #[test]
    fn line_path_needs_two_finite_points() {
        assert!(line_path(&[1.0], 0.0, 1.0, PLOT).is_none());
        assert!(line_path(&[1.0, f32::NAN], 0.0, 1.0, PLOT).is_none());
        let d = line_path(&[0.0, 10.0], 0.0, 10.0, PLOT).unwrap();
        assert!(d.starts_with('M'));
        assert!(d.contains(" L"));
    }

    #[test]
    fn area_path_closes_to_floor() {
        let d = area_path(&[0.0, 10.0], 0.0, 10.0, PLOT).unwrap();
        assert!(d.ends_with('Z'));
    }

    #[test]
    fn bar_geometry_measures_from_zero_baseline() {
        let bar = bar_geometry(SINGLE_SLOT, 10.0, 0.0, 10.0, PLOT).unwrap();
        assert_eq!(bar.y, PLOT.y);
        assert_eq!(bar.height, PLOT.height);
    }

    #[test]
    fn bar_geometry_negative_value_hangs_below_baseline() {
        let bar = bar_geometry(SINGLE_SLOT, -5.0, -5.0, 5.0, PLOT).unwrap();
        let baseline = map_y(0.0, -5.0, 5.0, PLOT);
        assert_eq!(bar.y, baseline);
        assert!(bar.height > 0.0);
    }

    #[test]
    fn bar_geometry_skips_non_finite() {
        assert!(bar_geometry(SINGLE_SLOT, f32::NAN, 0.0, 1.0, PLOT).is_none());
    }

    #[test]
    fn series_class_cycles_six_colors() {
        assert_eq!(series_class(0), "ui-chart-series--1");
        assert_eq!(series_class(5), "ui-chart-series--6");
        assert_eq!(series_class(6), "ui-chart-series--1");
    }

    #[test]
    fn clamp_progress_handles_garbage() {
        assert_eq!(clamp_progress(0.5), 0.5);
        assert_eq!(clamp_progress(-1.0), 0.0);
        assert_eq!(clamp_progress(2.0), 1.0);
        assert_eq!(clamp_progress(f32::NAN), 1.0);
    }

    #[test]
    fn chart_tone_class_suffixes() {
        assert_eq!(ChartTone::Primary.class_suffix(), "primary");
        assert_eq!(ChartTone::Neutral.class_suffix(), "neutral");
        assert_eq!(ChartTone::default(), ChartTone::Primary);
    }
}
