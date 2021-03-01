use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, ImageBitmap};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client {
    accumulated_time: f64,
    frame_count: u8,
    context: CanvasRenderingContext2d,
    player: Player,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let document = window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        // ImageBitmap::
        /*let callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            log(&format!("{}", event.key()));
        }) as Box<dyn Fn(_)>);

        window()
            .unwrap()
            .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
            .unwrap();
        callback.forget();*/

        Self {
            accumulated_time: 0.,
            frame_count: 0,
            context,
            player: Player::new(),
        }
    }

    #[wasm_bindgen]
    pub fn render(&mut self, current_time: f64, last_time: f64) {
        // log(&format!("{}", self.accumulated_time));
        self.accumulated_time += current_time - last_time;
        self.frame_count += 1;

        if self.accumulated_time >= 1000.0 {
            // log(&format!("frames: {}", self.frame_count));
            self.frame_count = 0;
            self.accumulated_time = 0.0;
        }

        self.context.set_fill_style(&JsValue::from("#fff")); // use .into()
        self.context.fill_rect(0., 0., 480., 640.);
        self.context.set_fill_style(&JsValue::from("#000"));
        self.context
            .fill_rect(self.player.x, self.player.y, 10.0, 10.0);

        // self.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh
    }

    #[wasm_bindgen]
    pub fn move_player_right(&mut self) {
        self.player.move_right();
    }

    #[wasm_bindgen]
    pub fn move_player_left(&mut self) {
        self.player.move_left();
    }

    #[wasm_bindgen]
    pub fn move_player_up(&mut self) {
        self.player.move_up();
    }

    #[wasm_bindgen]
    pub fn move_player_down(&mut self) {
        self.player.move_down();
    }
}

struct Player {
    x: f64,
    y: f64,
}

impl Player {
    fn new() -> Self {
        Self { x: 1., y: 1. }
    }

    fn move_right(&mut self) {
        self.x += 1.;
    }

    fn move_left(&mut self) {
        self.x -= 1.;
    }

    fn move_up(&mut self) {
        self.y -= 1.;
    }

    fn move_down(&mut self) {
        self.y += 1.;
    }
}
