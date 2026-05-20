use ui_gsap::{GsapBackend, GsapCapability};

#[test]
fn gsap_backend_declares_web_only_capabilities() {
    let backend = GsapBackend::default();

    assert_eq!(
        backend.capabilities(),
        &[
            GsapCapability::Timeline,
            GsapCapability::Scroll,
            GsapCapability::Flip
        ]
    );
    assert_eq!(backend.target(), "web");
}
