use tracing::info;
use tracing_wasm;
use wedding_site::App;

fn main() {
    if std::env!("ENVIRONMENT") == "DEV" {
        tracing_wasm::set_as_global_default();
    }
    yew::Renderer::<App>::new().render();
    info!("Yew app mounted");
}
