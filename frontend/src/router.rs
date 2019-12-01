use stdweb::web::{window, Window};
use yew::services::ConsoleService;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::views;

pub struct Router {
    window: Window,
    console: ConsoleService,
    child: Child,
    on_signal: Callback<crate::Msg>,
}

pub enum Msg {
    Reload,
    UpdateComponent,
    Title(&'static str),
}

pub enum Child {
    Index,
    Admin,
}

#[derive(Properties, PartialEq)]
pub struct RouterProps {
    #[props(required)]
    pub on_signal: Callback<crate::Msg>,
}

impl Component for Router {
    type Message = Msg;
    type Properties = RouterProps;

    fn create(p: Self::Properties, _: ComponentLink<Self>) -> Self {
        let r = Router {
            child: Child::Index,
            console: ConsoleService::new(),
            window: window(),
            on_signal: p.on_signal,
        };
        // window().add_event_listener(move |event: PopStateEvent| {
        //     r.update(Msg::Reload);
        // });
        r
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Reload => {
                if let Some(l) = self.window.location() {
                    if let Ok(p) = l.pathname() {
                        let routes = p.trim_matches('/').split("/").collect::<Vec<&str>>();
                        match routes[0] {
                            "index" => self.child = Child::Index,
                            "admin" => self.child = Child::Admin,
                            _ => self.child = Child::Index,
                        }
                        true
                    } else {
                        self.console.error("get path error!");
                        false
                    }
                } else {
                    self.console.error("get location error!");
                    false
                }
            }
            Msg::Title(t) => {
                self.on_signal.emit(crate::Msg::Title(t));
                false
            }
            Msg::UpdateComponent => {
                self.on_signal.emit(crate::Msg::UpdateComponent);
                false
            }
        }
    }

    fn view(&self) -> Html<Self> {
        match self.child {
            Child::Index => html! {
                <views::Index on_signal=|t| t/>
            },
            Child::Admin => html! {
                <views::Admin on_signal=|t| t/>
            },
        }
    }
}
