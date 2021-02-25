use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Indexr {}

impl Component for Indexr {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        debug!("create indexr");
        Indexr {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
            {"a"}
            </div>
        }
    }
}
