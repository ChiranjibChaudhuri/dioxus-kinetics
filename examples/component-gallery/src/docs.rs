use dioxus::prelude::Element;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentCategory {
    Actions,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentStatus {
    Ready,
    ComingSoon,
}

#[derive(Clone, Copy)]
pub struct ComponentDoc {
    pub name: &'static str,
    pub category: ComponentCategory,
    pub status: ComponentStatus,
    pub summary: &'static str,
    pub snippet: &'static str,
    pub render: Option<fn() -> Element>,
}

pub fn categories() -> &'static [ComponentCategory] {
    &[ComponentCategory::Actions]
}

pub fn component_docs() -> &'static [ComponentDoc] {
    &[]
}
