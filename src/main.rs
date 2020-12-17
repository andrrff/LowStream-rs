#![recursion_limit = "1024"]
use yew_router::{route::Route, switch::Permissive};
use yew::prelude::*;

macro_rules! classes {
    ($classe:expr, $classe_condition:expr) => {
        format!("{} {}", $classe, $classe_condition)
    };
}

mod pages;
use pages::{
    home::Home, page_not_found::PageNotFound, register::Register, login::Login, player::Player
};

mod switch;
use switch::{AppAnchor, AppRoute, AppRouter, PublicUrlSwitch};

pub enum Msg
{
    ToggleNav,
    ActionBottom
}

struct Model
{
    link: ComponentLink<Self>,
    navbar: bool,
    notification: bool
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self
        {
            link,
            navbar: false,
            notification: true
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg
        {
            Msg::ToggleNav => 
            {
                self.navbar = !self.navbar;
                true
            }
            Msg::ActionBottom => 
            {
                self.notification = !self.notification;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                {self.nav()}
                
                <main>
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                        redirect=AppRouter::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
                        })
                    />
                </main>

                {self.notification()}

                 <footer class="footer">
                    <div class="content has-text-centered">
                    <strong>{ "Powered by " }</strong>
                    <a>{ "LowStream Community " }</a>
                    <a href="https://github.com/lowstream-community/LowStream">
                            <span class="icon">
                            <i class="fab fa-github"></i>
                            </span>
                            <strong>{"GitHub"}</strong>
                        </a>
                    </div>
                </footer>
            </>
        }
    }
}

impl Model {
    fn nav(&self) -> Html {

        let Self {
            ref link,
            navbar,
            ..
        } = *self;

        let active_class = if navbar {"is-active"} else {""};

        html! {
                <>
            <nav class="navbar is-transparent is-fixed-top">
                <div class="navbar-brand is-rounded">
                    <AppAnchor classes="navbar-item" route=AppRoute::Home>
                    <span class="spinner-grow spinner-grow-sm badge bg-danger container d-flex h-0 align-items-center">{""}</span>
                    // <img src="https://www.pngkey.com/png/full/308-3085243_logo-rust-programming-language-logo.png" width="28" height="28" alt="LowStream"/>
                    <h3><strong>{"LowStream"}</strong></h3>
                    </AppAnchor>
                    <div class="navbar-burger" data-target="navbarExampleTransparentExample" onclick=link.callback(|_| Msg::ToggleNav) style="position: absolute; top: 8px; right: 16px;">
                    <span></span>
                    <span></span>
                    <span></span>
                    </div>
                </div>

                <div id="navbarExampleTransparentExample" class=classes!("navbar-menu ", active_class) style="position: absolute; top: 0px; right: 16px;">
                    <div class="navbar-start">
                    <a class=classes!("navbar-burger", active_class) onclick=link.callback(|_| Msg::ToggleNav)>
                        <span></span>
                        <span></span>
                        <span></span>
                    </a>
                    <AppAnchor classes="navbar-item" route=AppRoute::Home>
                            <a onclick=link.callback(|_| Msg::ToggleNav)>{ "Home" }</a>
                    </AppAnchor>
                    <AppAnchor classes="navbar-item" route=AppRoute::Home>
                            <a onclick=link.callback(|_| Msg::ToggleNav)>{ "Animes" }</a>
                    </AppAnchor>
                    <AppAnchor classes="navbar-item" route=AppRoute::Home>
                            <a onclick=link.callback(|_| Msg::ToggleNav)>{ "Filmes" }</a>
                    </AppAnchor>
                    <AppAnchor classes="navbar-item" route=AppRoute::Home>
                            <a onclick=link.callback(|_| Msg::ToggleNav)>{ "Series" }</a>
                    </AppAnchor>
                    <div class="navbar-item has-dropdown is-hoverable">
                        <a class="navbar-link">
                        {"Mais"}
                        </a>
                        <div class="navbar-dropdown is-boxed">
                        <AppAnchor classes="navbar-item" route=AppRoute::Player>
                            <a onclick=link.callback(|_| Msg::ToggleNav)>{ "Player Teste" }</a>
                        </AppAnchor>
                        <a class="navbar-item" onclick=link.callback(|_| Msg::ToggleNav)>
                            {"Contact us"}
                        </a>
                        <a class="navbar-item" href="https://github.com/LowStream-Community/LowStream/issues/new/choose" onclick=link.callback(|_| Msg::ToggleNav)>
                            {"Issues"}
                        </a>
                        <hr class="navbar-divider"/>
                        <a class="navbar-item" href="https://github.com/lowstream-community/LowStream" onclick=link.callback(|_| Msg::ToggleNav)>
                            {"GitHub"}
                        </a>
                        </div>
                    </div>
                    </div>

                    <div class="navbar-end">
                    <div class="navbar-item">
                        <div class="buttons" onclick=link.callback(|_| Msg::ToggleNav)>
                            <AppAnchor classes="button is-light is-rounded" route=AppRoute::Register>
                                { "Sign up" }
                            </AppAnchor>
                            <AppAnchor classes="button is-dark is-rounded" route=AppRoute::Login>
                                { "Login" }
                            </AppAnchor>
                        </div>
                    </div>
                    </div>
                </div>
                </nav>
                </>
        }
    }

    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Player => {
                html! { <Player /> }
            }
            AppRoute::Login => {
                html! { <Login /> }
            }
            AppRoute::Register => {
                html! { <Register /> }
            }
            AppRoute::Home => {
                html! { <Home /> }
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                html! { <PageNotFound route=route /> }
            }
        }
    }

    fn notification(&self) -> Html
    {
        if self.notification
        {
            return html!{
                <div class="notification is-danger is-light">
                    <button class="delete" onclick=self.link.callback(|_| Msg::ActionBottom)></button>
                        <strong>{"Atenção! "}</strong>
                        {"A equipe ainda está trabalhando no site, ainda há vários bugs, 
                        e não temos ainda uma data de previsão pra entrega do site. "}<a>
                        {"Porém possivelmente estará pronto para o ano de 2021."}</a> 
                        {" Obrigado por sua visita volte outra hora. uwu"}
                    </div>
            }
        }
        html!{}
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    App::<Model>::new().mount_to_body();
}
