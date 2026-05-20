use component_gallery::{categories, component_docs, ComponentCategory, ComponentStatus};

#[test]
fn registry_groups_components_by_product_category() {
    let categories = categories();

    assert_eq!(
        categories,
        &[
            ComponentCategory::Actions,
            ComponentCategory::Inputs,
            ComponentCategory::Layout,
            ComponentCategory::Surfaces,
            ComponentCategory::Feedback,
            ComponentCategory::Motion,
        ]
    );
}

#[test]
fn registry_contains_ready_and_coming_soon_components() {
    let docs = component_docs();

    assert!(docs
        .iter()
        .any(|doc| doc.name == "Button" && doc.status == ComponentStatus::Ready));
    assert!(docs
        .iter()
        .any(|doc| doc.name == "Surface" && doc.status == ComponentStatus::Ready));
    assert!(docs
        .iter()
        .any(|doc| doc.name == "GlassSurface" && doc.status == ComponentStatus::Ready));
    assert!(docs
        .iter()
        .any(|doc| doc.name == "Stack" && doc.status == ComponentStatus::Ready));
    assert!(docs
        .iter()
        .any(|doc| doc.name == "TextField" && doc.status == ComponentStatus::ComingSoon));
    assert!(docs
        .iter()
        .any(|doc| doc.name == "SharedElement" && doc.status == ComponentStatus::ComingSoon));
}

#[test]
fn coming_soon_entries_do_not_have_live_renderers() {
    for doc in component_docs() {
        if doc.status == ComponentStatus::ComingSoon {
            assert!(
                doc.render.is_none(),
                "{} should not render unavailable components",
                doc.name
            );
        }
    }
}
