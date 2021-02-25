use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Index {}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        debug!("create index");
        Index {}
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
            {"asdfasdf"}
            </div>
        }
    }
}
