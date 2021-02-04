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
                                                <nav class="menu" style="width: 0px">
                                                    <input type="checkbox" href="#" class="menu-open" name="menu-open" id="menu-open"/>
                                                    <label class="menu-open-button" for="menu-open">
                                                        <span class="hamburger hamburger-1"></span>
                                                        <span class="hamburger hamburger-2"></span>
                                                        <span class="hamburger hamburger-3"></span>
                                                    </label>
                                                    <a class="menu-item"> <fetch_json::LoadInfo id=self.props.id[self.value.clone() as usize].clone().to_string() type_box="home".to_string()/> </a>
                                                    <a class="menu-item">
                                                        <AppAnchor route=AppRoute::Eps(self.props.id[self.value].clone())>
                                                            <i class="fa fa-play"></i> 
                                                        </AppAnchor>
                                                    </a>
                                                    <a href="/" class="menu-item"> <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" style="    fill: rgba(255, 255, 255, 1);padding-bottom: 10px;height: 62px;-ms-filter:"><path d="M10 11H7.101c0-.003 0-.006.001-.009.065-.319.163-.634.291-.937.126-.297.281-.583.461-.85.178-.264.384-.513.61-.74C8.691 8.238 8.94 8.032 9.206 7.853c.266-.18.551-.334.848-.46.302-.128.617-.226.938-.291.658-.135 1.357-.135 2.018 0 .318.065.634.163.937.291.296.125.581.281.85.461.266.179.514.384.738.609l1.416-1.412c-.314-.316-.664-.604-1.036-.855-.373-.252-.773-.47-1.188-.646-.425-.18-.868-.317-1.315-.408-.923-.189-1.899-.189-2.819 0-.449.092-.892.229-1.316.409C8.858 5.727 8.458 5.944 8.086 6.196 7.716 6.445 7.368 6.733 7.05 7.05S6.445 7.716 6.197 8.085c-.252.373-.47.773-.646 1.19-.18.424-.317.867-.408 1.315C5.115 10.725 5.1 10.863 5.08 11H2l4 4L10 11zM14 13h2.899c-.001.003 0 .006-.001.008-.066.324-.164.639-.292.938-.123.293-.278.579-.459.848-.179.264-.385.514-.613.742-.225.225-.473.43-.739.61-.268.18-.553.335-.849.461-.303.128-.618.226-.938.291-.657.135-1.357.135-2.017 0-.319-.065-.634-.163-.937-.291-.297-.126-.583-.281-.85-.461-.264-.178-.513-.384-.74-.61L7.05 16.95c.317.317.666.605 1.035.854.373.252.773.47 1.19.646.424.18.867.317 1.315.408C11.051 18.952 11.525 19 12 19s.949-.048 1.408-.142c.449-.091.893-.229 1.317-.409.415-.176.815-.393 1.188-.645.372-.251.722-.54 1.035-.854.317-.317.605-.666.855-1.037.254-.377.472-.777.645-1.187.178-.42.315-.863.408-1.316.027-.135.043-.273.063-.41H22l-4-4L14 13z"></path></svg> </a>
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
                {self.conteudo.clone()}
            </>
        }
    }
}