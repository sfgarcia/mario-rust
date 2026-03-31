/// Verifica que index_itchio.html llama __wbg_init y no la variable
/// inexistente `init`, lo que causaba "ReferenceError: init is not defined".
#[test]
fn itchio_html_calls_wbg_init_not_bare_init() {
    let html = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("index_itchio.html"),
    )
    .expect("no se encontró index_itchio.html");

    let bare_init_calls: Vec<_> = html
        .lines()
        .enumerate()
        .filter(|(_, l)| {
            let t = l.trim();
            t.contains("await init(") && !t.contains("await __wbg_init(")
        })
        .collect();

    assert!(
        bare_init_calls.is_empty(),
        "index_itchio.html usa `await init(` en lugar de `await __wbg_init(` en líneas: {:?}",
        bare_init_calls.iter().map(|(n, _)| n + 1).collect::<Vec<_>>()
    );

    assert!(
        html.contains("await __wbg_init("),
        "index_itchio.html no contiene ninguna llamada a `await __wbg_init(`"
    );
}
