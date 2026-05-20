use ui_core::{A11yContract, ComponentContract, ComponentRole, FocusPolicy, TargetSize};

#[test]
fn button_contract_requires_action_role_and_touch_target() {
    let contract = ComponentContract::button("save-button");

    assert_eq!(contract.id.as_str(), "save-button");
    assert_eq!(contract.a11y.role, ComponentRole::Button);
    assert_eq!(contract.target_size, TargetSize::minimum_touch());
    assert!(contract.validate().is_ok());
}

#[test]
fn unlabeled_interactive_contract_is_invalid() {
    let contract = ComponentContract {
        a11y: A11yContract {
            role: ComponentRole::Button,
            label: None,
            focus_policy: FocusPolicy::Focusable,
            modal: false,
        },
        ..ComponentContract::button("icon-only")
    };

    assert_eq!(contract.validate().unwrap_err(), "interactive component needs an accessible label");
}
