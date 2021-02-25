#![recursion_limit = "512"]

#[macro_use]
extern crate log;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::{Route, Router, RouterButton};

//use common::Article;

mod component;
mod router;
mod view;

use router::AppRouter;

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
                <nav>
                    <RouterButton<AppRouter> route=AppRouter::Root>{"root"}</RouterButton<AppRouter>>
                    <RouterButton<AppRouter> route=AppRouter::Index>{"index"}</RouterButton<AppRouter>>
                    <RouterButton<AppRouter> route=AppRouter::Indexr>{"indexr"}</RouterButton<AppRouter>>
                </nav>
                <Router<AppRouter>
                    render = Router::render(|route: AppRouter| {
                        debug!("jump to {:?}", route);
                        match route {
                            AppRouter::Index => html!{ <view::Index /> },
                            AppRouter::Indexr => html!{ <view::Indexr /> },
                            _ => html!{{format!("{:?}", route)}}
                        }
                    })
                    redirect = Router::redirect(|_route: Route| AppRouter::Index)
                />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), wasm_bindgen::JsValue> {
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Debug).map_err(|e| e.to_string())?;
    #[cfg(not(debug_assertions))]
    console_log::init_with_level(log::Level::Info).map_err(|e| e.to_string())?;

    yew::App::<App>::new().mount_to_body();
    Ok(())
}
