use yew::prelude::*;
use crate::{
    switch::{AppAnchor, AppRoute},
};
use std::{thread, time};

// mod components;
// use components::{card_post::Card};

fn sleep()
{
    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
}

macro_rules! classes {
    ($classe:expr, $classe_condition:expr) => {
        {format!("{} {}", $classe, $classe_condition)}
    };
}

pub enum Msg
{
    Blur
}

pub struct Home
{
    string: String,
    iterator: i32,
    link: ComponentLink<Self>
}
impl Component for Home
{
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self
        {
            string: String::default(),
            iterator: 8,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::Blur => {
                for i in self.iterator..0
                {
                    self.string = format!("filter: blur({}px)", i);
                    sleep();
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <>
                <section class="hero is-medium is-dark is-bold has-background" style="padding-top: 40px">
                    <img src="https://i.pinimg.com/originals/30/94/e0/3094e0fd1114787639e8e334a840ca02.jpg" class="hero-background is-transparent"/>
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">
                                {"Seja muito Bem-Vindo(a)"}
                                <span class="tag is-white">
                                        {"new"}
                                </span>
                            </h1>
                            <h2 class="subtitle">
                                {"Somos uma plataforma de streaming simples, performática  e funcional."}
                            </h2>
                        </div>
                    </div>
                </section>                

                 <section style="background-color: #25262F;">
                 <div class="container has-text-centered" style="padding-top: 10px">
                        <h1 class="title" style="color: white">
                            <strong>{"Example de cards"}</strong>
                        </h1>
                </div>
                 <ul class="card-list">
                            <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Player>
                                <a class="card-image" style="background-image: url(https://patrullaroja.cl/wp-content/uploads/2020/05/shingeki-no-kyojin-temporada-final.jpg);">
                                </a>
                                <a class="card-description">
                                    <strong><h2>{"Shingeki no Kyojin"}</h2></strong>
                                    <p>{"Assista agora"}</p>
                                </a>
                            </AppAnchor>
                        </li>
            
                        <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Player>
                                <a class="card-image" style="background-image: url(https://somoskudasai.com/wp-content/uploads/2020/12/sgk_kv3_logo_web.jpg);">
                                </a>
                                <a class="card-description">
                                    <strong><h2>{"Shingeki no Kyojin"}</h2></strong>
                                    <p>{"Assista agora"}</p>
                                </a>
                            </AppAnchor>
                        </li>

                        
                        
                        <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Player>
                                <a class="card-image" style="background-image: url(https://blogs.opovo.com.br/bancadoanime/wp-content/uploads/sites/59/2020/09/Attack-on-Titan-The-Final-Season-anime-image.jpg);">
                                </a>
                                <a class="card-description">
                                    <strong><h2>{"Shingeki no Kyojin"}</h2></strong>
                                    <p>{"Assista agora"}</p>
                                </a>
                            </AppAnchor>
                        </li>
                        
                    </ul>
                </section>                
            </>
        }
    }
}