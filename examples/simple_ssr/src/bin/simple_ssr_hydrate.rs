use simple_ssr::App;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    let document = gloo::utils::document();
    let output = document.get_element_by_id("output").unwrap();
    yew::Renderer::<App>::with_root(output).hydrate();
}
