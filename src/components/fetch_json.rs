use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub type_box: String
}

use crate::{
    switch::{AppAnchor, AppRoute},
};

#[derive(Deserialize, Debug, Clone)]
pub struct Name {
    name: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Struture {
    anime: String,
    episodes: u64,
    description: String,
    genres: Vec<String>,
    studios: Vec<Name>,
    popularity: u64,
    averageScore: u64,
}

#[derive(Debug)]
pub enum Msg {
    GetInfo(String),
    ReceiveResponse(Result<Struture, anyhow::Error>),
}

#[derive(Debug)]
pub struct LoadInfo {
    props: Props,
    toggle_view: bool,
    fetch_task: Option<FetchTask>,
    json: Option<Struture>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl LoadInfo {
    fn view_json(&self) -> Html {
        let mut labels: Vec<Html> = Vec::new();
        match self.json {
            Some(ref content) => {
                for i in content.genres.iter()
                {
                    labels.push(html!{<span class="tag is-black" style="margin: 5px; border-radius: 18px"><strong>{i}</strong></span>});
                }
                if self.toggle_view
                {
                    if self.props.type_box.clone() == "post"
                    {
                        return html! {
                        <>
                        <div class="notransition" style="background-color: rgba(0, 0, 0, 40%); color: white; display: inline-block;
                                        font-size: 1rem;
                                        text-decoration: none;
                                        overflow: hidden;
                                        box-shadow: 0 0 4rem -1rem rgba(0, 0, 0, 1);
                                        border-radius: 18px;">
                        <header class="card-header">
                            <h2 class="card-header-title" style="color: white">{format!(" Informações sobre {}:", content.anime.clone())}<span class="icon"></span></h2>
                        </header>
                        <div class="card-content">
                        // <h1>{content.anime.clone()}</h1>
                            <h1><strong>{"Eps: "}</strong>{content.episodes.clone()}{"\n"}</h1>
                            <h1 style="padding-top: 5px"><strong>{"Descrição: "}</strong></h1>
                            <h1 style="height: 100px; padding-top: 5px; overflow: auto;">{content.description.clone()}{"\n"}</h1>
                            <h1><strong>{"Generos: "}</strong></h1>{for labels}
                            // <h1>{format!("{:?}", content.studios.clone())}</h1>
                            <h1 style="padding-top: 5px"><strong>{"Popularidade: "}</strong>{content.popularity.clone()}{"\n"}</h1>
                            <h1 style="padding-top: 5px"><strong>{"Nota: "}</strong>{content.averageScore.clone()}</h1>
                        </div>
                    </div>
                        </>
                    };
                
                    }
                    else
                    {
                        return html! {
                        <>
                        <div class="notransition" style="background-color: rgba(0, 0, 0, 30%);
                        color: white;
                        display: inline-block;
                        position: absolute;
                        top: 60px;
                        left: -100px;
                        height: 140px;
                        width: 350px;
                        /* overflow: auto; */
                        font-size: 1rem;
                        text-decoration: none;
                        overflow: auto;
                        box-shadow: 0 0 4rem -1rem rgb(0 0 0);
                        border-radius: 18px;">
                        // <header class="card-header">
                        //     <h2 class="card-header-title" style="color: white">{format!(" Informações sobre {}:", content.anime.clone())}<span class="icon"></span></h2>
                        // </header>
                        <div class="card-content">
                        // <h1>{content.anime.clone()}</h1>
                            <h1><strong>{"Eps: "}</strong>{content.episodes.clone()}{"\n"}</h1>
                            <h1 style="padding-top: 5px"><strong>{"Descrição: "}</strong></h1>
                            <h1 style="height: 100px; padding-top: 5px; overflow: auto;">{content.description.clone()}{"\n"}</h1>
                            <h1><strong>{"Gêneros: "}</strong></h1>{for labels}
                            // <h1>{format!("{:?}", content.studios.clone())}</h1>
                            <h1 style="padding-top: 5px"><strong>{"Popularidade: "}</strong>{content.popularity.clone()}{"\n"}</h1>
                            <h1 style="padding-top: 5px"><strong>{"Nota: "}</strong>{content.averageScore.clone()}</h1>
                        </div>
                    </div>
                        </>
                    };
                
                    
                    }
                }
                else
                {
                    return html!{};
                }
            }
            None => {
                html! {}
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { 
            }
        } else {
            html! {}
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! {}
        } else {
            html! {}
        }
    }
}
impl Component for LoadInfo {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            toggle_view: false,
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
            GetInfo(id) => {
                self.toggle_view = !self.toggle_view;
                let request = Request::get(format!("https://lowstreamcast-default-rtdb.firebaseio.com/info/array/{}.json", id))
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
        let id = self.props.id.clone();
        let style_button = if self.props.type_box.clone() == "post" {"width: 10px; margin: 5px; background: #00000057".to_string()} else {"width: 10px; margin: 5px; background: #00000000".to_string()};
        html! {
            <>
                <a class="button is-black is-rounded" onclick=self.link.callback(move |_| Msg::GetInfo(id.clone())) style=style_button>
                    <span class="icon"><i aria-hidden="true" class="fa fa-info"></i></span>
                </a>
                { self.view_fetching() }
                { self.view_json() }
                { self.view_error() }
            </>
        }
    }
}