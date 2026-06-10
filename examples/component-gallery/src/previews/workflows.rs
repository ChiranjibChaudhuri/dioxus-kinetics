use dioxus::prelude::*;
use kinetics::prelude::*;

// ---------------------------------------------------------------------------
// SortableList
// ---------------------------------------------------------------------------

pub fn sortable_list_preview() -> Element {
    rsx! { SortableListPreviewBody {} }
}

#[component]
fn SortableListPreviewBody() -> Element {
    let mut items = use_signal(|| {
        vec![
            SortableItem::new("triage", "Triage inbox")
                .with_description("Label and route new reports"),
            SortableItem::new("review", "Review escalations")
                .with_description("Anything older than 24 hours first"),
            SortableItem::new("retro", "Prepare retro notes")
                .with_description("Collect wins and incidents"),
            SortableItem::new("pairing", "Pairing session")
                .with_description("Rotate with the on-call engineer"),
        ]
    });

    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label",
                    "Drag rows or grab the grip with Space, then use arrow keys"
                }
            }
            SortableList {
                label: "Today's priorities".to_string(),
                items: items.read().clone(),
                on_reorder: move |order: Vec<String>| {
                    let current = items.read().clone();
                    let next: Vec<SortableItem> = order
                        .iter()
                        .filter_map(|id| current.iter().find(|item| &item.id == id).cloned())
                        .collect();
                    items.set(next);
                },
            }
        }
    }
}

// ---------------------------------------------------------------------------
// KanbanBoard
// ---------------------------------------------------------------------------

pub fn kanban_board_preview() -> Element {
    rsx! { KanbanBoardPreviewBody {} }
}

#[component]
fn KanbanBoardPreviewBody() -> Element {
    let mut columns = use_signal(|| {
        vec![
            KanbanColumn::new(
                "backlog",
                "Backlog",
                vec![
                    SortableItem::new("kb-1", "Audit color tokens")
                        .with_description("Contrast pass for the dark ramp"),
                    SortableItem::new("kb-2", "Spike: voice levels API"),
                ],
            ),
            KanbanColumn::new(
                "doing",
                "In progress",
                vec![SortableItem::new("kb-3", "Chart draw-in capture")
                    .with_description("Pin progress from the scene clock")],
            ),
            KanbanColumn::new(
                "done",
                "Done",
                vec![SortableItem::new("kb-4", "Roving focus primitive")],
            ),
        ]
    });

    rsx! {
        div { class: "gallery-demo-frame",
            div { class: "gallery-demo-frame-header",
                span { class: "gallery-variant-label",
                    "Drag cards between columns, or grab with Space and use arrows"
                }
            }
            KanbanBoard {
                label: "Sprint board".to_string(),
                columns: columns.read().clone(),
                on_move: move |mv: KanbanMove| {
                    let next = apply_kanban_move(&columns.read(), &mv);
                    columns.set(next);
                },
            }
        }
    }
}
