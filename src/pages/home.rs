use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rand::prelude::*;
use web_sys::Element;
use gloo::{events::EventListener};

use crate::{
    switch::{AppAnchor, AppRoute},
    components::{carousel, view_content, view_ecchi, view_romance, view_shounen, view_cards, card, fetch_json},
    log
};

#[derive(Deserialize, Debug, Clone)]
pub struct Ep {
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    eps: Vec<Ep>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Anime {
    anime: String,
    sticker: String,
    background: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Struture {
    array: Vec<Anime>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Name {
    name: String
}

#[derive(Debug)]
pub enum Msg {
    GetInfo,
    ReceiveResponse(Result<Struture, anyhow::Error>),
    Payload(String),
}

#[derive(Debug)]
pub struct LoadPosts {
    payload: String,
    debugged_payload: String,
    fetch_task: Option<FetchTask>,
    json: Option<Struture>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl LoadPosts {
    fn view_json(&self) -> Html {
        fn search(card_name: String, writing: String) -> bool
        {
                let chars_name: Vec<char> = card_name.chars().collect();
                let writing_chars: Vec<char> = writing.chars().collect();
                {
                    for (j, c) in writing_chars.iter().enumerate()
                    {
                        if c != &chars_name[j]
                        {
                            return false;
                        }
                    }
                }
                true
        }
        let mut cards: Vec<Html> = Vec::new();
        let mut random: Vec<Html> = Vec::new();
        let mut background: Vec<String> = Vec::new();
        let mut names_animes: Vec<String> = Vec::new();
        let mut id_animes: Vec<u64> = Vec::new();
        let mut value: usize = 0;
        let mut rng = rand::thread_rng();
        
        let mut count = 0;
        match self.json {
            Some(ref content) => {
                // console_log!("posts::view_json() - {}", "Loading done ✔️");
                for i in 0..content.array.len()
                {
                    names_animes.push(content.array[i].anime.clone());
                    id_animes.push(i as u64);
                    value = rng.gen_range(0, content.array.len());
                    if i < 9
                    {
                        random.push(html!{
                            <div class="padding-40px">
                                <AppAnchor route=AppRoute::Eps(value as u64)>
                                    <card::Card 
                                        score="4.9".to_string() 
                                        sticker=content.array[value].sticker.clone()
                                        name={
                                            let mut name: Vec<char> = Vec::new();
                                            let mut name_string: String = content.array[value].anime.clone();
                                            let chars_name = name_string.chars();
                                            for (i, e) in chars_name.enumerate()
                                            {
                                                if i < 35
                                                {
                                                    name.push(e);
                                                }
                                                else
                                                {
                                                    break;
                                                }
                                            }
                                            if content.array[value].anime.clone().chars().count() > 35
                                            {
                                                for i in 0..3
                                                {
                                                    name.push('.')
                                                }
                                            }
                                            let name_string: String = name.into_iter().collect();
                                            name_string
                                        }
                                    />
                                </AppAnchor>
                            </div>
                        })
                    }

                    if search(content.array[i].anime.clone().to_lowercase(), self.debugged_payload.clone().to_lowercase()) && count < 9
                    {
                        count += 1;
                        background.push(content.array[i].background.clone());
                        cards.push(html!{
                            <li class="card" style="background: black">
                                <AppAnchor route=AppRoute::Eps(i as u64)>
                                    <a class="card-image" style=format!("background-image: url({});", content.array[i].background.clone()) loading="lazy">
                                        <a class="card-description">
                                            <strong><h2>{content.array[i].anime.clone()}</h2></strong>
                                            <p>{"Assistir agora"}</p>
                                        </a>
                                    </a>
                                </AppAnchor>
                            </li>
                        })
                    }
                }

                html! {
                    <>
                        <carousel::Model background=self.export_info() name=names_animes id=id_animes.clone() page="animes".to_string() />
                        <section style="background-color: #25262F;">
                        <div class="mx-auto" style="width: 250px;">
                                <div class="field has-addons" style="padding-top: 87px;">
                                    <input class="input is-rounded" type="text" oninput=self.link.callback(|input: InputData| Msg::Payload(input.value)) value=&self.payload placeholder="Encontre seu anime"/>
                                </div>
                            </div>
                            <ul class="card-list con-cards" >
                                {for cards.clone()}
                            </ul>
                            <div>
                            </div>
                            <view_cards::View name="Random".to_string() content=random/>
                        </section>
                    </>
                }
            }
            None => {
                // console_log!("posts::view_json() - {}", "Error ❌ | posts::view_fetching() load is failed ❗");
                html! {}
            }
        }
    }

    fn view_fetching(&self) -> Html {
        let names_animes: Vec<String> = vec!["Loading...".to_string()];
        let id_animes: Vec<u64> = vec![99999999999];
        if self.fetch_task.is_some() {
            // console_log!("posts::view_fetching() - {}", "Loading progress 💬");
            html! { 
                <>
                    <carousel::Model background=self.export_info() name=names_animes id=id_animes page="animes".to_string() />
                    <section style="background-color: #25262F;">
                        <div class="mx-auto" style="width: 250px;">
                            <div class="control is-loading field has-addons">
                                <input class="input is-rounded" type="text" placeholder="Carregando"/>
                            </div>
                        </div>
                    </section>
                </> 
            }
        } else {
            // console_log!("posts::view_fetching() - {}", "Loading done ✔️");
            html! {}
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            // console_log!("posts::view_error() - {}", format!("Error - {} ❌", error.clone()));
            html! { <p>{ error.clone() }</p> }
        } else {
            // console_log!("posts::view_error() - {}", "None error occorred in fetch ✔️");
            html! {}
        }
    }

    fn export_info(&self) -> Vec<String>
    {
        let mut background: Vec<String> = Vec::new();
        match self.json
        {
            Some(ref content) => 
            {
                for i in 0..content.array.len()
                {
                    background.push(content.array[i].background.clone());
                }
                // console_log!("posts::export_info() - {}", "Process is done ✔️");
            },
            None => {
                // console_log!("posts::export_info() - {}", "Process is failure ❌");
                background.push("https://3.bp.blogspot.com/-bNbqH1Ll5BY/XD97Ife_ioI/AAAAAAAA9Mk/ipwUBBWtGgoEUNu7m7AaYGyvw1DxBR97QCLcBGAs/s1600/Fundo%2Btransparente%2B1900x1900.png".to_string())
            }
        }
        background
    }
}
impl Component for LoadPosts {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        callback.emit(Msg::GetInfo);
        Self {
            payload:          String::default(),
            debugged_payload: format!("{}", "none"),
            fetch_task:       None,
            json:             None,
            link,
            error:            None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            Msg::Payload(payload) => {
                if payload != self.payload {
                    self.debugged_payload = format!("{}", payload);
                    if self.debugged_payload == ""
                    {
                        self.debugged_payload = format!("{}", "none");
                    }
                    self.payload = payload;
                    true
                } else {
                    false
                }
            }
            GetInfo => {
                let request = Request::get("https://lowstreamcast-default-rtdb.firebaseio.com/cards.json")
                    .body(Nothing)
                    .expect("Não foi possível efetuar o request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Struture, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                let task = FetchService::fetch(request, callback).expect("Falha ao iniciar o request");
                self.fetch_task = Some(task);
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(dados) => {
                        self.json = Some(dados);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_json() }
                { self.view_error() }
                // <h1>{format!("querySelector( {:?} )", con_cards)}</h1>
                <view_content::Content/>
                <view_ecchi::Ecchi    />
                <view_shounen::Shounen/> 
                <view_romance::Romance/>
            </>
        }
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}