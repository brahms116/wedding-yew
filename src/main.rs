use tracing::info;
use tracing_wasm;
use wedding_site::App;

fn main() {
    tracing_wasm::set_as_global_default();
    yew::start_app::<App>();
    info!("Yew app mounted");
}
