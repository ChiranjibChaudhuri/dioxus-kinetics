use dioxus::prelude::*;
use ui_dioxus::{
    ArcGauge, AreaChart, AreaTrend, ChartSeries, ConversionFunnel, DensityGrid, FunnelChart,
    FunnelStage, GaugeChart, Heatmap, HeatmapRow, ProportionMap, Treemap, TreemapItem,
};

#[test]
fn area_chart_renders_filled_area_and_sr_table() {
    let html = dioxus_ssr::render_element(rsx! {
        AreaChart {
            label: "Signups".to_string(),
            series: vec![
                ChartSeries::new("A", vec![1.0, 3.0, 2.0]),
                ChartSeries::new("B", vec![0.5, 2.0, 4.0]),
            ],
        }
    });
    assert!(html.contains("ui-chart--area"), "{html}");
    assert!(html.contains("ui-chart-area"), "{html}");
    assert!(html.contains("ui-chart-line"), "{html}");
    assert!(html.contains(r#"role="img""#), "{html}");
    assert!(html.contains("Signups"), "{html}");
    assert!(html.contains("visually-hidden"), "{html}");
}

#[test]
fn area_chart_empty_state() {
    let html = dioxus_ssr::render_element(rsx! {
        AreaChart { label: "X".to_string(), series: vec![] }
    });
    assert!(html.contains("No data"), "{html}");
}

#[test]
fn area_trend_alias_renders_identically() {
    let a = dioxus_ssr::render_element(rsx! {
        AreaTrend { label: "X".to_string(), series: vec![ChartSeries::new("a", vec![1.0, 2.0])] }
    });
    let b = dioxus_ssr::render_element(rsx! {
        AreaChart { label: "X".to_string(), series: vec![ChartSeries::new("a", vec![1.0, 2.0])] }
    });
    assert_eq!(a, b);
}

#[test]
fn funnel_chart_renders_centered_stages_and_values() {
    let html = dioxus_ssr::render_element(rsx! {
        FunnelChart {
            label: "Checkout".to_string(),
            stages: vec![
                FunnelStage::new("Viewed", 1000.0),
                FunnelStage::new("Added", 400.0),
                FunnelStage::new("Paid", 100.0),
            ],
        }
    });
    assert!(html.contains("ui-chart--funnel"), "{html}");
    assert!(html.contains("ui-chart-funnel-stage"), "{html}");
    assert!(html.contains("Viewed"), "{html}");
    assert!(html.contains("1k"), "{html}");
    assert!(html.contains("visually-hidden"), "{html}");
}

#[test]
fn funnel_progress_pins_widths() {
    let zero = dioxus_ssr::render_element(rsx! {
        FunnelChart {
            label: "F".to_string(),
            progress: Some(0.0),
            stages: vec![FunnelStage::new("a", 100.0), FunnelStage::new("b", 50.0)],
        }
    });
    // progress=0 collapses every stage width to the 1px floor.
    assert!(zero.contains(r#"width="1""#), "{zero}");

    let full = dioxus_ssr::render_element(rsx! {
        FunnelChart {
            label: "F".to_string(),
            progress: Some(1.0),
            stages: vec![FunnelStage::new("a", 100.0)],
        }
    });
    assert!(!full.contains(r#"width="1""#), "{full}");
}

#[test]
fn conversion_funnel_alias_renders_identically() {
    let stages = vec![FunnelStage::new("a", 1.0)];
    let a = dioxus_ssr::render_element(rsx! {
        ConversionFunnel { label: "F".to_string(), stages: stages.clone() }
    });
    let b = dioxus_ssr::render_element(rsx! {
        FunnelChart { label: "F".to_string(), stages }
    });
    assert_eq!(a, b);
}

#[test]
fn gauge_chart_renders_semicircle_meter() {
    let html = dioxus_ssr::render_element(rsx! {
        GaugeChart { label: "CPU".to_string(), value: 0.42 }
    });
    assert!(html.contains("ui-gauge"), "{html}");
    assert!(html.contains(r#"role="meter""#), "{html}");
    assert!(html.contains(r#"aria-valuenow="42""#), "{html}");
    assert!(html.contains(r#"aria-valuetext="42%""#), "{html}");
    assert!(html.contains("ui-gauge-arc"), "{html}");
}

#[test]
fn arc_gauge_alias_renders_identically() {
    let a = dioxus_ssr::render_element(rsx! { ArcGauge { label: "G".to_string(), value: 0.5 } });
    let b = dioxus_ssr::render_element(rsx! { GaugeChart { label: "G".to_string(), value: 0.5 } });
    assert_eq!(a, b);
}

#[test]
fn heatmap_renders_grid_cells_and_intensity() {
    let html = dioxus_ssr::render_element(rsx! {
        Heatmap {
            label: "Activity".to_string(),
            rows: vec![
                HeatmapRow::new("Mon", vec![0.0, 5.0, 10.0]),
                HeatmapRow::new("Tue", vec![3.0, 7.0, 1.0]),
            ],
            column_labels: vec!["M".to_string(), "A".to_string(), "E".to_string()],
        }
    });
    assert!(html.contains("ui-chart--heatmap"), "{html}");
    assert!(html.contains("ui-chart-heat-cell"), "{html}");
    assert!(html.contains("fill-opacity"), "{html}");
    assert!(html.contains("Mon"), "{html}");
    assert!(html.contains("visually-hidden"), "{html}");
}

#[test]
fn density_grid_alias_renders_identically() {
    let rows = vec![HeatmapRow::new("r", vec![1.0, 2.0])];
    let a = dioxus_ssr::render_element(
        rsx! { DensityGrid { label: "H".to_string(), rows: rows.clone() } },
    );
    let b = dioxus_ssr::render_element(rsx! { Heatmap { label: "H".to_string(), rows } });
    assert_eq!(a, b);
}

#[test]
fn treemap_renders_tiles_and_shares() {
    let html = dioxus_ssr::render_element(rsx! {
        Treemap {
            label: "Budget".to_string(),
            items: vec![
                TreemapItem::new("Eng", 60.0),
                TreemapItem::new("Sales", 30.0),
                TreemapItem::new("Ops", 10.0),
            ],
        }
    });
    assert!(html.contains("ui-chart--treemap"), "{html}");
    assert!(html.contains("ui-chart-tile"), "{html}");
    assert!(html.contains("Eng"), "{html}");
    assert!(html.contains("%"), "{html}");
    assert!(html.contains("visually-hidden"), "{html}");
}

#[test]
fn proportion_map_alias_renders_identically() {
    let items = vec![TreemapItem::new("a", 1.0), TreemapItem::new("b", 2.0)];
    let a = dioxus_ssr::render_element(rsx! {
        ProportionMap { label: "T".to_string(), items: items.clone() }
    });
    let b = dioxus_ssr::render_element(rsx! {
        Treemap { label: "T".to_string(), items }
    });
    assert_eq!(a, b);
}
