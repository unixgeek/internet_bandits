use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn say_hello() {
    log("Rust says fuck ysfafou");
}

#[wasm_bindgen]
pub struct Client {
    count: u64,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { count: 0 }
    }

    #[wasm_bindgen]
    pub fn animate(&self, count: u8) {
        // self.count += 1;
        log(format!("{}", count).as_str());
    }
}
