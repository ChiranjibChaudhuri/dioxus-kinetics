use ui_core::{A11yContract, ComponentContract, ComponentRole, FocusPolicy, TargetSize};

#[test]
fn button_contract_requires_action_role_and_touch_target() {
    let contract = ComponentContract::button("save-button");

    assert_eq!(contract.id.as_str(), "save-button");
    assert_eq!(contract.a11y.role, ComponentRole::Button);
    assert_eq!(contract.a11y.label.as_deref(), Some("save-button"));
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

    assert_eq!(
        contract.validate().unwrap_err(),
        "interactive component needs an accessible label"
    );
}

#[test]
fn whitespace_only_interactive_label_is_invalid() {
    let contract = ComponentContract {
        a11y: A11yContract {
            label: Some("   \t\n".to_string()),
            ..ComponentContract::button("icon-only").a11y
        },
        ..ComponentContract::button("icon-only")
    };

    assert_eq!(
        contract.validate().unwrap_err(),
        "interactive component needs an accessible label"
    );
}

#[test]
fn below_minimum_target_size_is_invalid() {
    let contract = ComponentContract {
        target_size: TargetSize {
            min_width_px: TargetSize::minimum_pointer().min_width_px - 1.0,
            min_height_px: TargetSize::minimum_pointer().min_height_px,
        },
        ..ComponentContract::button("small-button")
    };

    assert_eq!(
        contract.validate().unwrap_err(),
        "target size is too small for pointer interaction"
    );
}

#[test]
fn minimum_pointer_target_size_is_valid() {
    let contract = ComponentContract {
        target_size: TargetSize::minimum_pointer(),
        ..ComponentContract::button("compact-button")
    };

    assert!(contract.validate().is_ok());
}

#[test]
fn non_finite_target_size_is_invalid() {
    let contract = ComponentContract {
        target_size: TargetSize {
            min_width_px: f32::NAN,
            min_height_px: TargetSize::minimum_pointer().min_height_px,
        },
        ..ComponentContract::button("nan-button")
    };

    assert_eq!(
        contract.validate().unwrap_err(),
        "target size is too small for pointer interaction"
    );
}
