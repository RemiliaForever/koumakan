use http::{Request, Response};
use log::debug;
use stdweb::web::{window, History};
use yew::{
    format::{Nothing, Text},
    html,
    services::{FetchService, Task},
    Callback,
    Component,
    ComponentLink,
    Html,
    Properties,
    ShouldRender,
};

use crate::router;

pub struct Admin {
    history: History,
    on_signal: Callback<router::Msg>,
    link: ComponentLink<Admin>,
    fetch: FetchService,
    task: Option<Box<dyn Task>>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[props(required)]
    pub on_signal: Callback<router::Msg>,
}

pub enum Msg {
    Do,
    Done,
}

impl Component for Admin {
    type Message = Msg;
    type Properties = Props;

    fn create(p: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut a = Admin {
            history: window().history(),
            on_signal: p.on_signal,
            link: link,
            fetch: FetchService::new(),
            task: None,
        };
        let r = Request::get("/index.html").body(Nothing).unwrap();
        let t = a.fetch.fetch(
            r,
            a.link.send_back(|res: Response<Text>| {
                debug!("{:?}", res);
                Msg::Done
            }),
        );
        a.task = Some(Box::new(t));
        a
    }

    fn mounted(&mut self) -> ShouldRender {
        self.on_signal.emit(router::Msg::Title("Admin"));
        self.on_signal.emit(router::Msg::UpdateComponent);
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Do => {
                self.history.push_state((), "", Some("/index"));
                self.on_signal.emit(router::Msg::Reload);
                false
            }
            Msg::Done => false,
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <button id="btn-admin" onclick=|_| Msg::Do class="mdl-button mdl-js-button mdl-button--primary mdl-js-ripple-effect">{ "Admin" }</button>
        }
    }
}
