use solidjs_compiler::CompileErrorKind;
use solidjs_compiler::refresh::{TransformRefreshOptions, transform_refresh};

fn options() -> TransformRefreshOptions {
    TransformRefreshOptions {
        filename: Some("Counter.tsx".to_string()),
        bundler: Some("vite".to_string()),
        import_source: Some("solid-js/refresh".to_string()),
        fix_render: Some(false),
        source_map: Some(true),
        ..TransformRefreshOptions::default()
    }
}

#[test]
fn refresh_registers_components_without_a_node_host() {
    let output = transform_refresh(
        "export function Counter() { return <div>Count</div>; }".to_string(),
        Some(options()),
    )
    .unwrap();
    assert!(output.code.contains("solid-js/refresh"));
    assert!(output.code.contains("$$component"));
    assert!(output.code.contains("import.meta.hot"));
    assert!(output.map.is_some());
}

#[test]
fn refresh_leaves_skipped_modules_unchanged() {
    let source = "/* @refresh skip */ export function Counter() { return <div />; }";
    let output = transform_refresh(source.to_string(), Some(options())).unwrap();
    assert_eq!(output.code, source);
    assert!(output.map.is_none());
}

#[test]
fn refresh_leaves_plain_modules_unchanged() {
    let source = "export const value = 42;";
    let output = transform_refresh(source.to_string(), Some(options())).unwrap();
    assert_eq!(output.code, source);
    assert!(output.map.is_none());
}

#[test]
fn refresh_reports_configuration_errors() {
    let error = transform_refresh(
        "export const value = 42;".to_string(),
        Some(TransformRefreshOptions {
            bundler: Some("unknown".to_string()),
            ..options()
        }),
    )
    .unwrap_err();
    assert_eq!(error.kind(), CompileErrorKind::Configuration);
    let error = transform_refresh(
        "".to_string(),
        Some(TransformRefreshOptions {
            jsx: Some(true),
            ..options()
        }),
    )
    .unwrap_err();
    assert_eq!(error.kind(), CompileErrorKind::Configuration);
}

#[test]
fn refresh_reports_parse_errors() {
    let error =
        transform_refresh("export function Broken( {".to_string(), Some(options())).unwrap_err();
    assert_eq!(error.kind(), CompileErrorKind::Parse);
}

#[cfg(feature = "node")]
#[test]
fn node_refresh_adapter_preserves_the_output_shape() {
    let source = "export function Counter() { return <div />; }";
    let rust = transform_refresh(source.to_string(), Some(options())).unwrap();
    let node = solidjs_compiler::transform_refresh(source.to_string(), Some(options())).unwrap();
    assert_eq!(rust.code, node.code);
    assert_eq!(rust.map, node.map);
    assert!(node.css.is_none());
    assert!(node.css_hash.is_none());
}
