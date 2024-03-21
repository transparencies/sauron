use sauron::{
    html::text, html::units::px, jss, node, skip_if, wasm_bindgen, Application, Cmd, Node, Program,
    SkipDiff,
    view,
};

enum Msg {
    Increment,
    Decrement,
    Reset,
}

#[derive(Clone)]
struct App {
    count: i32,
}

impl App {
    fn new() -> Self {
        App { count: 0 }
    }
}

impl Application for App {
    type MSG = Msg;


    fn update(&mut self, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
            Msg::Reset => self.count = 0,
        }
        Cmd::none()
    }

    view! {
        <main>
            <input type="button"
                value="+"
                on_click=|_| {
                    Msg::Increment
                }
            />
            <button class="count" on_click=|_|{Msg::Reset} >{text(self.count)}</button>
            <input type="button"
                value="-"
                on_click=|_| {
                    Msg::Decrement
                }
            />
        </main>
    }

    fn stylesheet() -> Vec<String> {
        vec![jss! {
            "body":{
                font_family: "verdana, arial, monospace",
            },

            "main":{
                width:px(30),
                height: px(100),
                margin: "auto",
                text_align: "center",
            },

            "input, .count":{
                font_size: px(40),
                padding: px(30),
                margin: px(30),
            }
        }]
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
