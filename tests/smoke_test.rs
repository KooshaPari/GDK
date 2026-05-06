// Traces to: FR-001
#[test]
fn smoke_test_loads() {
    // Verify the gdk crate is accessible and compiles without errors
    let _ = std::any::type_name::<gdk::GdkResult<()>>();
}
