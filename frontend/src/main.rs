#![recursion_limit = "512"]
use common::Article;
use log::debug;
use stdweb::{
    js,
    web::{window, History},
};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod router;
mod views;

struct App {
    title: &'static str,
    history: History,
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
            history: window().history(),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        js! {
            document.querySelectorAll("[class*='mdl-js-']").forEach(e => {
                componentHandler.upgradeElement(e);
            });
        }
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Router(r) => {
                self.history.push_state((), "Welcome to Koumakan", Some(r));
                self.mounted()
            }
            Msg::Title(t) => {
                self.title = t;
                true
            }
            Msg::UpdateComponent => self.mounted(),
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div id="app" class="mdl-layout mdl-js-layout mdl-layout--fixed-drawer mdl-layout--fixed-header">
                <header id="header" class="mdl-layout__header">
                     <div class="mdl-layout__header-row">
                        <span class="mdl-layout-title">{ self.title.clone() }</span>
                        <div class="mdl-layout-spacer" />
                        <div id="input" class="mdl-textfield mdl-js-textfield mdl-textfield--expandable mdl-textfield--floating-label mdl-textfield--align-right">
                            <label class="mdl-button mdl-js-button mdl-button--icon mdl-js-ripple-effect" for="fixed-header-drawer-exp">
                                <i class="material-icons">{ "search" }</i>
                            </label>
                            <div class="mdl-textfield__expandable-holder">
                                <input class="mdl-textfield__input" type="text" name="sample" id="fixed-header-drawer-exp" />
                            </div>
                        </div>
                     </div>
                </header>
                <div id="main-sidebar" class="mdl-layout__drawer">
                    <div id="avatar">
                        <img src="/avatar.jpg" alt="RemiliaForever"/>
                        <span>{ "RemiliaForever" }</span>
                    </div>
                    <nav class="mdl-navigation">
                         <a class="mdl-navigation__link mdl-js-ripple-effect" onclick=|_| Msg::Router("/index")>
                             <div class="">
                                 <span class="material-icons">{ "home" }</span>
                                 <span>{ "首页" }</span>
                             </div>
                         </a>
                         <div class="devide" />
                         // <div>
                         //     <a href="/category/it"><div class="material-icons">{ "phonelink" }</div><span>{ "IT技术" }</span></a>
                         // </div>
                         // <div>
                         //     <div><div class="material-icons">{ "games" }</div><span>{ "ACG见闻" }</span></div>
                         // </div>
                         // <div>
                         //     <div><div class="material-icons">{ "today" }</div><span>{ "生活琐记" }</span></div>
                         // </div>
                         // <div/>
                         // <div>
                         //     <div>{ "bookmark" }</div>
                         //     <span>{ "标签" }</span>
                         //     <div>
                         //         <div>
                         //             <div>
                         //                 <div>
                         //                     <span class="label">{ "key" }</span>
                         //                     <span class="chip">{ "value" }</span>
                         //                 </div>
                         //             </div>
                         //         </div>
                         //     </div>
                         // </div>
                         // <div>
                         //     <div>{ "archive" }</div>
                         //     <span>{ "文章归档" }</span>
                         //     <div>
                         //         <div>
                         //             <div>
                         //                 <div>
                         //                     <span class="label">{ "key" }</span>
                         //                     <span class="chip">{ "value" }</span>
                         //                 </div>
                         //             </div>
                         //         </div>
                         //     </div>
                         // </div>
                         // <div />
                         // <div>
                         //     <div><md-icon>{ "people" }</md-icon><span>{ "友情链接" }</span></div>
                         // </div>
                         // <div>
                         //     <div><md-icon>{ "link" }</md-icon><span>{ "关于" }</span></div>
                         // </div>
                         // <div>
                         //     <div><md-icon>{ "rss_feed" }</md-icon><span>{ "RSS" }</span></div>
                         // </div>
                    </nav>
                </div>

                <main id="main" class="mdl-layout__content">
                    <router::Router on_signal=|t| t/>
                    {
                        let a = Article {
                            id: None,
                            title: "".to_owned(),
                            brief: "".to_owned(),
                            content: "".to_owned(),
                            category: "".to_owned(),
                            labels: "".to_owned(),
                            date: chrono::Local::now().naive_local(),
                        };
                        serde_json::to_string(&a).unwrap()
                     }
                </main>
            </div>
        }
    }
}

fn main() {
    web_logger::init();
    yew::start_app::<App>();
}
