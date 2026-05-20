#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentId(String);

impl ComponentId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentRole {
    Button,
    TextField,
    Checkbox,
    RadioGroup,
    Switch,
    Select,
    Combobox,
    Tabs,
    Dialog,
    Drawer,
    Popover,
    Tooltip,
    Menu,
    Table,
    List,
    Tree,
    Surface,
    Status,
}

impl ComponentRole {
    pub fn is_interactive(self) -> bool {
        !matches!(self, ComponentRole::Surface | ComponentRole::Status)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusPolicy {
    NotFocusable,
    Focusable,
    FocusTrap,
    RestoreOnClose,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct A11yContract {
    pub role: ComponentRole,
    pub label: Option<String>,
    pub focus_policy: FocusPolicy,
    pub modal: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TargetSize {
    pub min_width_px: f32,
    pub min_height_px: f32,
}

impl TargetSize {
    pub const fn minimum_touch() -> Self {
        Self {
            min_width_px: 44.0,
            min_height_px: 44.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentContract {
    pub id: ComponentId,
    pub a11y: A11yContract,
    pub target_size: TargetSize,
}

impl ComponentContract {
    pub fn button(id: impl Into<String>) -> Self {
        Self {
            id: ComponentId::new(id),
            a11y: A11yContract {
                role: ComponentRole::Button,
                label: Some("Button".to_string()),
                focus_policy: FocusPolicy::Focusable,
                modal: false,
            },
            target_size: TargetSize::minimum_touch(),
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.a11y.role.is_interactive() && self.a11y.label.as_deref().unwrap_or("").is_empty() {
            return Err("interactive component needs an accessible label");
        }

        if self.target_size.min_width_px < 24.0 || self.target_size.min_height_px < 24.0 {
            return Err("target size is too small for pointer interaction");
        }

        Ok(())
    }
}
