#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

use lunar_logger::Builder;
fn main() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|e| {
        console::error_1(&JsValue::from(format!("{}", e)))
    }));

    Builder::new().create().enable_logger().unwrap();

    log::info!("This is info");
    log::warn!("This is warn");
    log::error!("This is error");
}
