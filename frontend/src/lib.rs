#![recursion_limit = "512"]

#[macro_use]
extern crate log;

use wasm_bindgen::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

//use common::Article;

//mod router;
//mod views;

struct App {
    title: &'static str,
}

pub enum Msg {
    Router(&'static str),
    Title(&'static str),
    UpdateComponent,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            title: "Welcome to Koumakan",
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Router(_) => true,
            Msg::Title(t) => {
                self.title = t;
                true
            }
            Msg::UpdateComponent => true,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
            {"hhhh"}
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), wasm_bindgen::JsValue> {
    let _window = web_sys::window().ok_or("get window failed")?;

    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Debug).map_err(|e| e.to_string())?;
    #[cfg(not(debug_assertions))]
    console_log::init_with_level(log::Level::Info).map_err(|e| e.to_string())?;

    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");

    yew::App::<App>::new().mount_to_body();
    Ok(())
}
