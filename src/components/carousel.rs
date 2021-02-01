use yew::{
    prelude::*,
};

use yewtil::NeqAssign;

use rand::prelude::*;

use crate::switch::{AppAnchor, AppRoute};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub background: Vec<String>,
    pub name: Vec<String>,
    pub id: Vec<u64>,
    pub page: String
}

pub struct Model {
    props: Props,
    link: ComponentLink<Self>,
    pub value: usize,
    pub conteudo: Html,
}

#[derive(Debug)]
pub enum Msg {
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
            value: 0,
            conteudo: html! {},
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MoveToRight => {
                let mut rng = rand::thread_rng();
                self.value = rng.gen_range(0, self.props.background.len());

                if self.props.page == "animes".to_string()
                {
                    self.conteudo = html! {
                            <AppAnchor route=AppRoute::Eps(self.props.id[self.value].clone())>
                                <section class="hero is-medium is-dark is-bold has-background">
                                    <img src=format!("{}", self.props.background[self.value.clone()].clone()) class="hero-background img-fluid is-transparent" style="height: 544px" />
                                    <div class="hero-body">
                                        <div class="container" style="padding-top: 60px">
                                            <h1 class="title" style="text-shadow: 1px 1px #363636;">
                                                {self.props.name[self.value].clone()}
                                            </h1>
                                        </div>
                                    </div>
                                </section>
                                <div class="cover-image-header__overlay" style="top:200px">
                                    <div class="cover-image-header__rows">
                                    </div>
                                </div>
                            </AppAnchor>
                        };
                }
                else
                {
                    self.conteudo = html! {
                            <AppAnchor route=AppRoute::Eps(self.props.id[self.value].clone())>
                                <section class="hero is-medium is-dark is-bold has-background">
                                    <img src=format!("{}", self.props.background[self.value.clone()].clone()) class="hero-background img-fluid is-transparent"/>
                                    <div class="hero-body">
                                        <div class="container" style="padding-top: 60px">
                                            <h1 class="title" style="text-shadow: 1px 1px #363636;">
                                                {self.props.name[self.value].clone()}
                                            </h1>
                                        </div>
                                    </div>
                                </section>
                                <div class="cover-image-header__overlay" style="top:200px">
                                    <div class="cover-image-header__rows">
                                    </div>
                                </div>
                            </AppAnchor>
                        };
                    }
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
                
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {

            <>
                {self.conteudo.clone()}
            </>
        }
    }
}