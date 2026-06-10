use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Shared item vocabulary
// ---------------------------------------------------------------------------

/// One entry in a `SortableList` or `KanbanBoard` column.
#[derive(Clone, Debug, PartialEq)]
pub struct SortableItem {
    pub id: String,
    pub label: String,
    pub description: String,
}

impl SortableItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: String::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

// ---------------------------------------------------------------------------
// Pure reorder logic (unit-tested)
// ---------------------------------------------------------------------------

/// Moves `id` to `to_index` within `order`, shifting neighbours. Returns
/// `None` when the id is absent or already in place, so callers can skip
/// redundant `on_reorder` emissions.
fn move_id_to_index(order: &[String], id: &str, to_index: usize) -> Option<Vec<String>> {
    let from = order.iter().position(|entry| entry == id)?;
    let to = to_index.min(order.len().saturating_sub(1));
    if from == to {
        return None;
    }
    let mut next = order.to_vec();
    let moved = next.remove(from);
    next.insert(to, moved);
    Some(next)
}

/// Moves `id` one step up (`delta = -1`) or down (`+1`). `None` at the ends.
fn shift_id(order: &[String], id: &str, delta: i32) -> Option<Vec<String>> {
    let from = order.iter().position(|entry| entry == id)?;
    let to = from.checked_add_signed(delta as isize)?;
    if to >= order.len() {
        return None;
    }
    move_id_to_index(order, id, to)
}

/// "Position 2 of 5" style announcement fragments for the live region.
fn position_phrase(index: usize, count: usize) -> String {
    format!("position {} of {}", index + 1, count)
}

fn grab_announcement(label: &str, index: usize, count: usize) -> String {
    format!(
        "{label} grabbed at {}. Use arrow keys to move, space to drop, escape to cancel.",
        position_phrase(index, count)
    )
}

fn move_announcement(label: &str, index: usize, count: usize) -> String {
    format!("{label} moved to {}.", position_phrase(index, count))
}

fn drop_announcement(label: &str, index: usize, count: usize) -> String {
    format!("{label} dropped at {}.", position_phrase(index, count))
}

fn cancel_announcement(label: &str) -> String {
    format!("Reordering {label} cancelled.")
}

// ---------------------------------------------------------------------------
// SortableList
// ---------------------------------------------------------------------------

/// A reorderable list. Items move by pointer drag (HTML5 drag-and-drop with
/// an insertion indicator) or fully by keyboard: the grip button grabs an
/// item with Space/Enter, ArrowUp/ArrowDown move it, Space drops, Escape
/// cancels and restores the order at grab time. Every move is announced
/// through an assertive live region.
///
/// The component is controlled: it renders `items` as given and reports the
/// complete new id order through `on_reorder`.
#[component]
pub fn SortableList(
    label: String,
    items: Vec<SortableItem>,
    on_reorder: EventHandler<Vec<String>>,
    #[props(default)] disabled: bool,
) -> Element {
    let order: Vec<String> = items.iter().map(|item| item.id.clone()).collect();
    let count = items.len();

    let mut dragging = use_signal(|| None::<String>);
    let mut drop_index = use_signal(|| None::<usize>);
    // Keyboard grab state: the grabbed id plus the order to restore on Escape.
    let mut grabbed = use_signal(|| None::<(String, Vec<String>)>);
    let mut announcement = use_signal(String::new);

    let order_for_drop = order.clone();

    rsx! {
        div { class: "ui-sortable",
            ul {
                class: "ui-sortable-list",
                "aria-label": "{label}",
                for (index, item) in items.iter().enumerate() {
                    {
                        let id = item.id.clone();
                        let item_label = item.label.clone();
                        let is_dragging = dragging.read().as_deref() == Some(id.as_str());
                        let is_grabbed = grabbed
                            .read()
                            .as_ref()
                            .is_some_and(|(grabbed_id, _)| grabbed_id == &id);
                        let is_drop_target = *drop_index.read() == Some(index)
                            && dragging.read().is_some()
                            && !is_dragging;
                        let row_class = format!(
                            "ui-sortable-item{}{}{}",
                            if is_dragging { " ui-sortable-item--dragging" } else { "" },
                            if is_grabbed { " ui-sortable-item--grabbed" } else { "" },
                            if is_drop_target { " ui-sortable-item--drop-target" } else { "" },
                        );
                        let drag_id = id.clone();
                        let over_id = id.clone();
                        let drop_order = order_for_drop.clone();
                        let key_order = order.clone();
                        let key_id = id.clone();
                        let key_label = item_label.clone();

                        rsx! {
                            li {
                                key: "{id}",
                                class: "{row_class}",
                                draggable: if disabled { "false" } else { "true" },
                                ondragstart: move |_| {
                                    if !disabled {
                                        dragging.set(Some(drag_id.clone()));
                                    }
                                },
                                ondragover: move |evt| {
                                    evt.prevent_default();
                                    let _ = &over_id;
                                    drop_index.set(Some(index));
                                },
                                ondrop: move |evt| {
                                    evt.prevent_default();
                                    if let Some(source) = dragging.take() {
                                        if let Some(next) =
                                            move_id_to_index(&drop_order, &source, index)
                                        {
                                            on_reorder.call(next);
                                        }
                                    }
                                    drop_index.set(None);
                                },
                                ondragend: move |_| {
                                    dragging.set(None);
                                    drop_index.set(None);
                                },
                                button {
                                    class: "ui-sortable-handle",
                                    r#type: "button",
                                    disabled,
                                    "aria-label": "Reorder {item_label}, {position_phrase(index, count)}",
                                    "aria-pressed": if is_grabbed { "true" } else { "false" },
                                    onkeydown: move |evt| {
                                        if disabled {
                                            return;
                                        }
                                        let is_space = matches!(
                                            evt.key(),
                                            Key::Character(ref c) if c.as_str() == " "
                                        );
                                        let held = grabbed
                                            .read()
                                            .as_ref()
                                            .is_some_and(|(held_id, _)| held_id == &key_id);
                                        match evt.key() {
                                            Key::Enter => {
                                                evt.prevent_default();
                                                toggle_grab(
                                                    &mut grabbed,
                                                    &mut announcement,
                                                    &key_order,
                                                    &key_id,
                                                    &key_label,
                                                );
                                            }
                                            _ if is_space => {
                                                evt.prevent_default();
                                                toggle_grab(
                                                    &mut grabbed,
                                                    &mut announcement,
                                                    &key_order,
                                                    &key_id,
                                                    &key_label,
                                                );
                                            }
                                            Key::ArrowUp if held => {
                                                evt.prevent_default();
                                                if let Some(next) = shift_id(&key_order, &key_id, -1) {
                                                    let at = next
                                                        .iter()
                                                        .position(|entry| entry == &key_id)
                                                        .unwrap_or(0);
                                                    announcement.set(move_announcement(
                                                        &key_label,
                                                        at,
                                                        next.len(),
                                                    ));
                                                    on_reorder.call(next);
                                                }
                                            }
                                            Key::ArrowDown if held => {
                                                evt.prevent_default();
                                                if let Some(next) = shift_id(&key_order, &key_id, 1) {
                                                    let at = next
                                                        .iter()
                                                        .position(|entry| entry == &key_id)
                                                        .unwrap_or(0);
                                                    announcement.set(move_announcement(
                                                        &key_label,
                                                        at,
                                                        next.len(),
                                                    ));
                                                    on_reorder.call(next);
                                                }
                                            }
                                            Key::Escape if held => {
                                                evt.prevent_default();
                                                if let Some((_, original)) = grabbed.take() {
                                                    announcement
                                                        .set(cancel_announcement(&key_label));
                                                    on_reorder.call(original);
                                                }
                                            }
                                            _ => {}
                                        }
                                    },
                                    span { class: "ui-sortable-grip", "aria-hidden": "true",
                                        svg {
                                            view_box: "0 0 10 16",
                                            circle { cx: "3", cy: "3", r: "1.4" }
                                            circle { cx: "7", cy: "3", r: "1.4" }
                                            circle { cx: "3", cy: "8", r: "1.4" }
                                            circle { cx: "7", cy: "8", r: "1.4" }
                                            circle { cx: "3", cy: "13", r: "1.4" }
                                            circle { cx: "7", cy: "13", r: "1.4" }
                                        }
                                    }
                                }
                                div { class: "ui-sortable-body",
                                    span { class: "ui-sortable-label", "{item.label}" }
                                    if !item.description.is_empty() {
                                        span { class: "ui-sortable-description", "{item.description}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "visually-hidden",
                "aria-live": "assertive",
                "aria-atomic": "true",
                "{announcement}"
            }
        }
    }
}

/// Grab/drop toggle shared by the Space and Enter branches: announces the
/// transition and stores the at-grab order so Escape can restore it.
fn toggle_grab(
    grabbed: &mut Signal<Option<(String, Vec<String>)>>,
    announcement: &mut Signal<String>,
    order: &[String],
    id: &str,
    label: &str,
) {
    let index = order.iter().position(|entry| entry == id).unwrap_or(0);
    let held = grabbed
        .read()
        .as_ref()
        .is_some_and(|(held_id, _)| held_id == id);
    if held {
        grabbed.set(None);
        announcement.set(drop_announcement(label, index, order.len()));
    } else {
        grabbed.set(Some((id.to_string(), order.to_vec())));
        announcement.set(grab_announcement(label, index, order.len()));
    }
}

// ---------------------------------------------------------------------------
// KanbanBoard
// ---------------------------------------------------------------------------

/// One column of a `KanbanBoard`.
#[derive(Clone, Debug, PartialEq)]
pub struct KanbanColumn {
    pub id: String,
    pub title: String,
    pub items: Vec<SortableItem>,
}

impl KanbanColumn {
    pub fn new(id: impl Into<String>, title: impl Into<String>, items: Vec<SortableItem>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            items,
        }
    }
}

/// A requested card move, reported through `KanbanBoard::on_move`. Apply it
/// to your column state with [`apply_kanban_move`].
#[derive(Clone, Debug, PartialEq)]
pub struct KanbanMove {
    pub item_id: String,
    pub from_column: String,
    pub to_column: String,
    pub to_index: usize,
}

/// Finds `(column_index, item_index)` of a card across columns.
fn locate_item(columns: &[KanbanColumn], item_id: &str) -> Option<(usize, usize)> {
    columns.iter().enumerate().find_map(|(col, column)| {
        column
            .items
            .iter()
            .position(|item| item.id == item_id)
            .map(|row| (col, row))
    })
}

/// Applies a [`KanbanMove`] to a column set, returning the new columns.
/// Returns the input unchanged when the move references unknown ids, so it
/// is always safe to call with an event payload.
pub fn apply_kanban_move(columns: &[KanbanColumn], mv: &KanbanMove) -> Vec<KanbanColumn> {
    let mut next = columns.to_vec();
    let Some((from_col, from_row)) = locate_item(&next, &mv.item_id) else {
        return next;
    };
    if next[from_col].id != mv.from_column {
        return next;
    }
    let Some(to_col) = next.iter().position(|column| column.id == mv.to_column) else {
        return next;
    };
    let card = next[from_col].items.remove(from_row);
    let to_index = mv.to_index.min(next[to_col].items.len());
    next[to_col].items.insert(to_index, card);
    next
}

/// Resolves a keyboard move request into a [`KanbanMove`]: vertical deltas
/// reorder within the column, horizontal deltas send the card to the same
/// (clamped) row of the neighbouring column. `None` when the move would
/// leave the board.
fn keyboard_move(
    columns: &[KanbanColumn],
    item_id: &str,
    col_delta: i32,
    row_delta: i32,
) -> Option<KanbanMove> {
    let (from_col, from_row) = locate_item(columns, item_id)?;
    let to_col = from_col.checked_add_signed(col_delta as isize)?;
    if to_col >= columns.len() {
        return None;
    }
    let to_index = if col_delta == 0 {
        let to_row = from_row.checked_add_signed(row_delta as isize)?;
        if to_row >= columns[from_col].items.len() {
            return None;
        }
        to_row
    } else {
        from_row.min(columns[to_col].items.len())
    };
    if to_col == from_col && to_index == from_row {
        return None;
    }
    Some(KanbanMove {
        item_id: item_id.to_string(),
        from_column: columns[from_col].id.clone(),
        to_column: columns[to_col].id.clone(),
        to_index,
    })
}

fn kanban_move_announcement(columns: &[KanbanColumn], label: &str, mv: &KanbanMove) -> String {
    let column_title = columns
        .iter()
        .find(|column| column.id == mv.to_column)
        .map(|column| column.title.as_str())
        .unwrap_or(mv.to_column.as_str());
    format!(
        "{label} moved to {column_title}, position {}.",
        mv.to_index + 1
    )
}

/// A multi-column kanban board. Cards move by pointer drag across columns or
/// by keyboard: Space/Enter grabs a card, ArrowUp/ArrowDown reorder within
/// the column, ArrowLeft/ArrowRight send it to the neighbouring column,
/// Space drops, Escape cancels. Moves are announced via a live region and
/// reported through `on_move`; apply them with [`apply_kanban_move`].
#[component]
pub fn KanbanBoard(
    label: String,
    columns: Vec<KanbanColumn>,
    on_move: EventHandler<KanbanMove>,
    #[props(default)] disabled: bool,
) -> Element {
    let mut dragging = use_signal(|| None::<(String, String)>);
    let mut drop_target = use_signal(|| None::<(String, usize)>);
    let mut grabbed = use_signal(|| None::<String>);
    let mut announcement = use_signal(String::new);

    let board = columns.clone();

    rsx! {
        div { class: "ui-kanban", role: "group", "aria-label": "{label}",
            div { class: "ui-kanban-columns",
                for column in columns.iter() {
                    {
                        let column_id = column.id.clone();
                        let column_len = column.items.len();
                        let drop_board = board.clone();
                        let over_column = column_id.clone();
                        let drop_column = column_id.clone();

                        rsx! {
                            section {
                                key: "{column.id}",
                                class: "ui-kanban-column",
                                "aria-label": "{column.title}",
                                ondragover: move |evt| {
                                    evt.prevent_default();
                                    // Falls back to appending when the pointer is over
                                    // column chrome rather than a specific card.
                                    if drop_target.read().as_ref().map(|(col, _)| col)
                                        != Some(&over_column)
                                    {
                                        drop_target.set(Some((over_column.clone(), column_len)));
                                    }
                                },
                                ondrop: move |evt| {
                                    evt.prevent_default();
                                    if let Some((item_id, from_column)) = dragging.take() {
                                        let to_index = drop_target
                                            .take()
                                            .filter(|(col, _)| col == &drop_column)
                                            .map(|(_, index)| index)
                                            .unwrap_or(column_len);
                                        let mv = KanbanMove {
                                            item_id,
                                            from_column,
                                            to_column: drop_column.clone(),
                                            to_index,
                                        };
                                        announcement.set(kanban_move_announcement(
                                            &drop_board,
                                            "Card",
                                            &mv,
                                        ));
                                        on_move.call(mv);
                                    }
                                },
                                header { class: "ui-kanban-column-header",
                                    h4 { class: "ui-kanban-column-title", "{column.title}" }
                                    span { class: "ui-kanban-column-count", "{column.items.len()}" }
                                }
                                ul { class: "ui-kanban-cards",
                                    for (index, item) in column.items.iter().enumerate() {
                                        {
                                            let item_id = item.id.clone();
                                            let item_label = item.label.clone();
                                            let is_dragging = dragging
                                                .read()
                                                .as_ref()
                                                .is_some_and(|(id, _)| id == &item_id);
                                            let is_grabbed =
                                                grabbed.read().as_deref() == Some(item_id.as_str());
                                            let is_drop_target = drop_target
                                                .read()
                                                .as_ref()
                                                .is_some_and(|(col, at)| {
                                                    col == &column_id && *at == index
                                                })
                                                && dragging.read().is_some()
                                                && !is_dragging;
                                            let card_class = format!(
                                                "ui-kanban-card{}{}{}",
                                                if is_dragging { " ui-kanban-card--dragging" } else { "" },
                                                if is_grabbed { " ui-kanban-card--grabbed" } else { "" },
                                                if is_drop_target { " ui-kanban-card--drop-target" } else { "" },
                                            );
                                            let drag_item = item_id.clone();
                                            let drag_column = column_id.clone();
                                            let over_card_column = column_id.clone();
                                            let key_board = board.clone();
                                            let key_item = item_id.clone();
                                            let key_label = item_label.clone();

                                            rsx! {
                                                li { key: "{item.id}", class: "{card_class}",
                                                    div {
                                                        class: "ui-kanban-card-surface",
                                                        role: "button",
                                                        tabindex: if disabled { "-1" } else { "0" },
                                                        draggable: if disabled { "false" } else { "true" },
                                                        "aria-pressed": if is_grabbed { "true" } else { "false" },
                                                        "aria-label": "{item_label}, {column.title}, {position_phrase(index, column_len)}",
                                                        ondragstart: move |_| {
                                                            if !disabled {
                                                                dragging.set(Some((
                                                                    drag_item.clone(),
                                                                    drag_column.clone(),
                                                                )));
                                                            }
                                                        },
                                                        ondragover: move |evt| {
                                                            evt.prevent_default();
                                                            evt.stop_propagation();
                                                            drop_target.set(Some((
                                                                over_card_column.clone(),
                                                                index,
                                                            )));
                                                        },
                                                        ondragend: move |_| {
                                                            dragging.set(None);
                                                            drop_target.set(None);
                                                        },
                                                        onkeydown: move |evt| {
                                                            if disabled {
                                                                return;
                                                            }
                                                            let is_space = matches!(
                                                                evt.key(),
                                                                Key::Character(ref c) if c.as_str() == " "
                                                            );
                                                            let held = grabbed.read().as_deref()
                                                                == Some(key_item.as_str());
                                                            let arrow = match evt.key() {
                                                                Key::ArrowUp => Some((0, -1)),
                                                                Key::ArrowDown => Some((0, 1)),
                                                                Key::ArrowLeft => Some((-1, 0)),
                                                                Key::ArrowRight => Some((1, 0)),
                                                                _ => None,
                                                            };
                                                            if evt.key() == Key::Enter || is_space {
                                                                evt.prevent_default();
                                                                if held {
                                                                    grabbed.set(None);
                                                                    announcement.set(format!(
                                                                        "{key_label} dropped."
                                                                    ));
                                                                } else {
                                                                    grabbed.set(Some(key_item.clone()));
                                                                    announcement.set(format!(
                                                                        "{key_label} grabbed. Arrows move, space drops, escape cancels."
                                                                    ));
                                                                }
                                                            } else if evt.key() == Key::Escape && held {
                                                                evt.prevent_default();
                                                                grabbed.set(None);
                                                                announcement.set(cancel_announcement(
                                                                    &key_label,
                                                                ));
                                                            } else if let (Some((dc, dr)), true) =
                                                                (arrow, held)
                                                            {
                                                                evt.prevent_default();
                                                                if let Some(mv) = keyboard_move(
                                                                    &key_board, &key_item, dc, dr,
                                                                ) {
                                                                    announcement.set(
                                                                        kanban_move_announcement(
                                                                            &key_board, &key_label,
                                                                            &mv,
                                                                        ),
                                                                    );
                                                                    on_move.call(mv);
                                                                }
                                                            }
                                                        },
                                                        span { class: "ui-kanban-card-label", "{item.label}" }
                                                        if !item.description.is_empty() {
                                                            span {
                                                                class: "ui-kanban-card-description",
                                                                "{item.description}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "visually-hidden",
                "aria-live": "assertive",
                "aria-atomic": "true",
                "{announcement}"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order(ids: &[&str]) -> Vec<String> {
        ids.iter().map(|s| s.to_string()).collect()
    }

    fn board() -> Vec<KanbanColumn> {
        vec![
            KanbanColumn::new(
                "todo",
                "To do",
                vec![
                    SortableItem::new("a", "Task A"),
                    SortableItem::new("b", "Task B"),
                ],
            ),
            KanbanColumn::new("doing", "Doing", vec![SortableItem::new("c", "Task C")]),
            KanbanColumn::new("done", "Done", vec![]),
        ]
    }

    #[test]
    fn move_id_shifts_neighbours() {
        let next = move_id_to_index(&order(&["a", "b", "c"]), "a", 2).unwrap();
        assert_eq!(next, order(&["b", "c", "a"]));
    }

    #[test]
    fn move_id_to_same_index_is_none() {
        assert!(move_id_to_index(&order(&["a", "b"]), "a", 0).is_none());
        assert!(move_id_to_index(&order(&["a", "b"]), "missing", 0).is_none());
    }

    #[test]
    fn move_id_clamps_index() {
        let next = move_id_to_index(&order(&["a", "b"]), "a", 99).unwrap();
        assert_eq!(next, order(&["b", "a"]));
    }

    #[test]
    fn shift_id_stops_at_ends() {
        assert!(shift_id(&order(&["a", "b"]), "a", -1).is_none());
        assert!(shift_id(&order(&["a", "b"]), "b", 1).is_none());
        assert_eq!(
            shift_id(&order(&["a", "b"]), "a", 1).unwrap(),
            order(&["b", "a"])
        );
    }

    #[test]
    fn announcements_are_one_based() {
        assert!(grab_announcement("Task", 0, 3).contains("position 1 of 3"));
        assert!(move_announcement("Task", 2, 3).contains("position 3 of 3"));
        assert!(drop_announcement("Task", 1, 3).contains("position 2 of 3"));
    }

    #[test]
    fn apply_kanban_move_across_columns() {
        let mv = KanbanMove {
            item_id: "a".into(),
            from_column: "todo".into(),
            to_column: "doing".into(),
            to_index: 0,
        };
        let next = apply_kanban_move(&board(), &mv);
        assert_eq!(next[0].items.len(), 1);
        assert_eq!(next[1].items[0].id, "a");
        assert_eq!(next[1].items[1].id, "c");
    }

    #[test]
    fn apply_kanban_move_clamps_index_and_ignores_unknown() {
        let mv = KanbanMove {
            item_id: "a".into(),
            from_column: "todo".into(),
            to_column: "done".into(),
            to_index: 99,
        };
        let next = apply_kanban_move(&board(), &mv);
        assert_eq!(next[2].items[0].id, "a");

        let bogus = KanbanMove {
            item_id: "zzz".into(),
            from_column: "todo".into(),
            to_column: "done".into(),
            to_index: 0,
        };
        assert_eq!(apply_kanban_move(&board(), &bogus), board());
    }

    #[test]
    fn keyboard_move_vertical_stays_in_column() {
        let mv = keyboard_move(&board(), "a", 0, 1).unwrap();
        assert_eq!(mv.to_column, "todo");
        assert_eq!(mv.to_index, 1);
        assert!(keyboard_move(&board(), "a", 0, -1).is_none());
        assert!(keyboard_move(&board(), "b", 0, 1).is_none());
    }

    #[test]
    fn keyboard_move_horizontal_clamps_row() {
        let mv = keyboard_move(&board(), "b", 1, 0).unwrap();
        assert_eq!(mv.to_column, "doing");
        // Row 1 clamps to the single-card column's append position.
        assert_eq!(mv.to_index, 1);
        assert!(keyboard_move(&board(), "a", -1, 0).is_none());
        let to_empty = keyboard_move(&board(), "c", 1, 0).unwrap();
        assert_eq!(to_empty.to_column, "done");
        assert_eq!(to_empty.to_index, 0);
    }

    #[test]
    fn kanban_announcement_names_target_column() {
        let mv = KanbanMove {
            item_id: "a".into(),
            from_column: "todo".into(),
            to_column: "doing".into(),
            to_index: 0,
        };
        let text = kanban_move_announcement(&board(), "Task A", &mv);
        assert!(text.contains("Doing"));
        assert!(text.contains("position 1"));
    }
}
