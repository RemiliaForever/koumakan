use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Text {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub text: &'static str,
}

pub enum Message {
    Click,
}

impl Component for Text {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Text { props, link }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Click => true,
        }
    }

    fn view(&self) -> Html {
        let on_click = self.link.callback(|_| {
            info!("click");
            Message::Click
        });
        html! {
            <p>
                <span>{ self.props.text }</span>
                <button onclick=on_click>{ "change color" }</button>
            </p>
        }
    }
}
