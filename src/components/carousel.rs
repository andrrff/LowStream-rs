use yew::{
    prelude::*,
};

use yewtil::NeqAssign;

use rand::prelude::*;

use crate::{
    switch::{AppAnchor, AppRoute},
    components::fetch_json
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub background: Vec<String>,
    pub name: Vec<String>,
    pub id: Vec<u64>,
    pub page: String
}

pub struct Model {
    props: Props,
    toggleInfo: bool,
    open_or_close: String,
    link: ComponentLink<Self>,
    pub value: usize,
    pub conteudo: Html,
}

#[derive(Debug)]
pub enum Msg {
    OpenInfo,
    MoveToLeft,
    MoveToRight,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::MoveToRight);
        callback.emit(Msg::MoveToRight);
        Self {
            props,
            link,
            toggleInfo: false,
            open_or_close: " ".to_string(),
            value: 0,
            conteudo: html! {},
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenInfo => {
                self.toggleInfo = !self.toggleInfo;
                true
            }
            Msg::MoveToRight => {
                let mut rng = rand::thread_rng();
                self.value = rng.gen_range(0, self.props.background.len());

                if self.props.page == "animes".to_string()
                {
                    self.conteudo = html! {
                        <>
                            // <AppAnchor route=AppRoute::Eps(self.props.id[self.value].clone())>
                                <section class="hero is-medium is-dark is-bold has-background">
                                    <img src=format!("{}", self.props.background[self.value.clone()].clone()) class="hero-background img-fluid is-transparent" style="height: 544px" />
                                    <div class="cover-image-header__overlay" style="top:200px">
                                        <div class="cover-image-header__rows">
                                        </div>
                                    </div>
                                    <div class="hero-body">
                                        <div class="container" style="padding-top: 60px">
                                            <h1 class="title" style="text-shadow: 1px 1px #363636;">
                                                {self.props.name[self.value].clone()}
                                                <nav class="menu">
                                                    <input type="checkbox" href="#" class="menu-open" name="menu-open" id="menu-open"/>
                                                    <label class="menu-open-button" for="menu-open">
                                                        <span class="hamburger hamburger-1"></span>
                                                        <span class="hamburger hamburger-2"></span>
                                                        <span class="hamburger hamburger-3"></span>
                                                    </label>
                                                    <a class="menu-item" onclick=self.link.callback(|_| Msg::OpenInfo)> <i class="fa fa-info"></i> </a>
                                                    <a class="menu-item">
                                                        <AppAnchor route=AppRoute::Eps(self.props.id[self.value].clone())>
                                                            <i class="fa fa-play"></i> 
                                                        </AppAnchor>
                                                    </a>
                                                    <a class="menu-item"> <i class="fa fa-link"></i> </a>
                                                </nav>
                                            </h1>
                                        </div>
                                    </div>
                                </section>
                                </>
                            // </AppAnchor>
                        };
                }
                else
                {
                    self.conteudo = html! {
                            // <AppAnchor route=AppRoute::Eps(self.props.id[self.value].clone())>
                                <section class="hero is-medium is-dark is-bold has-background">
                                    <img src=format!("{}", self.props.background[self.value.clone()].clone()) class="hero-background img-fluid is-transparent"/>
                                    <div class="cover-image-header__overlay" style="top:200px">
                                        <div class="cover-image-header__rows">
                                        </div>
                                    </div>
                                    <div class="hero-body">
                                        <div class="container" style="padding-top: 60px">
                                            <h1 class="title" style="text-shadow: 1px 1px #363636;">
                                                {self.props.name[self.value].clone()}
                                            </h1>
                                        </div>
                                    </div>
                                </section>
                            // </AppAnchor>
                        };
                    }
                    true
            }
            Msg::MoveToLeft => {
                if self.value == 0 {
                    self.value = self.props.background.len() - 1;
                } else {
                    self.value -= 1;
                }
                self.conteudo = html! {
                    <img src=format!("{}", self.props.background[self.value].clone())/>
                };
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // self.props.neq_assign(props)
        false
    }

    fn view(&self) -> Html {
        html! {

            <>
                {self.model()}
                {self.conteudo.clone()}
            </>
        }
    }
}

impl Model
{
    fn model(&self) -> Html
    {
        let active_class = if self.toggleInfo { "is-active" } else { "" };

        html!{
            <div id="modal-bis" class=format!("modal {}", active_class)>
                <div class="modal-background"></div>
                <div class="modal-content" style="background: black">
                    <h1 style="color: white; text-align: center;">{"🚧Em produção🚧"}</h1>
                </div>
                <button onclick=self.link.callback(|_| Msg::OpenInfo) class="modal-close is-large" aria-label="close"></button>
            </div>
        }
    }
}