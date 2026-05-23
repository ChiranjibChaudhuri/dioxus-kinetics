use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn sidebar_preview() -> Element {
    rsx! { SidebarPreviewBody {} }
}

#[component]
fn SidebarPreviewBody() -> Element {
    let mut selected = use_signal(|| "settings".to_string());
    rsx! {
        Sidebar {
            selected: selected.read().clone(),
            sections: vec![SidebarSection::new(
                "Workspace",
                vec![
                    SidebarItem::new("home", "Home", "#home"),
                    SidebarItem::new("settings", "Settings", "#settings"),
                    SidebarItem::new("billing", "Billing", "#billing"),
                ],
            )],
            onnavigate: move |id: String| selected.set(id),
        }
    }
}

pub fn breadcrumb_preview() -> Element {
    let items = vec![
        BreadcrumbItem::link("Workspaces", "#"),
        BreadcrumbItem::link("Acme Ops", "#"),
        BreadcrumbItem::link("Reports", "#"),
        BreadcrumbItem::current("Q1 revenue"),
    ];
    rsx! { Breadcrumb { items, aria_label: "Page breadcrumb" } }
}

pub fn pagination_preview() -> Element {
    rsx! { PaginationPreviewBody {} }
}

#[component]
fn PaginationPreviewBody() -> Element {
    let mut page = use_signal(|| 3u32);
    rsx! {
        div { class: "gallery-variant-grid gallery-variant-grid--stack",
            div { class: "gallery-variant-tile",
                span { class: "gallery-variant-label", "Page {page.read()} / 12" }
                Pagination {
                    page: *page.read(),
                    total_pages: 12,
                    on_select: move |p: u32| page.set(p),
                }
            }
        }
    }
}

pub fn stepper_preview() -> Element {
    rsx! { StepperPreviewBody {} }
}

#[component]
fn StepperPreviewBody() -> Element {
    let mut current = use_signal(|| "review".to_string());
    let steps = vec![
        StepperStep::new("plan", "Plan").with_description("Pick your seats"),
        StepperStep::new("checkout", "Checkout").with_description("Payment + delivery"),
        StepperStep::new("review", "Review").with_description("Confirm and place order"),
        StepperStep::new("done", "Done").with_description("Confirmation"),
    ];
    rsx! {
        Stepper {
            steps,
            current: current.read().clone(),
            on_select: move |id: String| current.set(id),
        }
    }
}

pub fn segmented_control_preview() -> Element {
    rsx! { SegmentedControlPreviewBody {} }
}

#[component]
fn SegmentedControlPreviewBody() -> Element {
    let mut view = use_signal(|| "grid".to_string());
    let options = vec![
        SegmentItem::new("grid", "Grid"),
        SegmentItem::new("list", "List"),
        SegmentItem::new("calendar", "Calendar"),
    ];
    rsx! {
        SegmentedControl {
            options,
            selected: view.read().clone(),
            group_label: "View mode".to_string(),
            on_select: move |v: String| view.set(v),
        }
    }
}
