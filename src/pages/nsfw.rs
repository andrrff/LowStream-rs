use serde::Deserialize;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use crate::{
    log,
    components::interval_task::ProgressDelay
};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

use rand::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Struture {
    array: Vec<String>
}

#[derive(Debug)]
pub enum Msg {
    GetInfo,
    GetYPosition,
    ReceiveResponse(Result<Struture, anyhow::Error>),
}

pub struct Nsfw
{
    fetch_task: Option<FetchTask>,
    json: Option<Struture>,
    position_y: f64,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Nsfw
{
    fn view_json(&self) -> Html {
        let mut background: Vec<Html> = Vec::new();
        match self.json
        {
            Some(ref content) => {
                let mut rng = rand::thread_rng();
                for i in 0..self.position_y as u64 / 120.0 as u64
                {
                    background.push(html!{
                        <img src=content.array[i as usize].clone()/>//rng.gen_range(0, content.array.len()
                    });
                }

                html!{
                    <>
                        <section class="hero is-medium is-dark is-bold has-background">
                            <img src=content.array[rng.gen_range(0, content.array.len())].clone() class="hero-background img-fluid is-transparent" style="height: 540px"/>
                            <div class="hero-body">
                                <div class="container" style="padding-top: 60px">
                                </div>
                            </div>
                        </section>
                        <div class="cover-image-header__overlay" style="top:200px">
                            <div class="cover-image-header__rows">
                                <h2>
                                    {"🚧Em desenvolvimento🚧"}
                                </h2>
                            </div>
                        </div>
                        <div style="padding-top: 10pc;">
                            <section id="photos" >
                                { for background }
                            </section>
                        </div>
                    </>
                }
            }
            None => {
                html!{

                }
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { 
                <>
                <section class="hero is-medium is-dark is-bold ">
                            <div class="hero-body">
                                <div class="container">
                                    <h1 class="title" style="padding-top: 40px;">
                                        {"Loading..."}
                                    </h1>
                                </div>
                            </div>
                        </section>
                        <section style="background-color: #25262F;">
                            <ul class="card-list">
                                <h1>{"..."}</h1>
                            </ul>
                        </section>
                // <div class="position-absolute top-90 start-50 translate-middle">
                //     <div class="d-flex justify-content-center">
                //         <div class="spinner-border is-white" role="status">
                //             <span class="visually-hidden">{"Loading..."}</span>
                //         </div>
                //     </div>
                // </div>
                </> 
            }
        } else {
            html! { <p></p> }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}

impl Component for Nsfw {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        callback.emit(Msg::GetInfo);
        Self{
            fetch_task: None,
            json: None,
            position_y: 500.0,
            link,
            error: None
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            GetYPosition => {
                self.position_y = window().scroll_y().unwrap();
                if window().scroll_y().unwrap() < 500.0
                {
                    self.position_y = 500.0;
                }
                true
            }
            GetInfo => {
                let request = Request::get("https://lowstreamcast-default-rtdb.firebaseio.com/nsfw/.json")
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

    fn view(&self) -> Html
    {
        html!{
            <>
                { self.view_fetching() }
                { self.view_json() }
                { self.view_error() } 
                <ProgressDelay duration_ms=450u64 on_complete=self.link.callback(|_| Msg::GetYPosition) />
            </>
        }
    }
}


fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn run() -> Result<i32, JsValue> {
    console_log!("log is on ✅");
    // body().append_child(&val)?;
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i > 2 || body().offset_height() as f64 - window().scroll_y().unwrap() <= 700.0 && window().scroll_y().unwrap() != 0.0{
            // body().set_text_content(Some("All done!"));
            console_log!("Fetching...");

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        console_log!("tamanho = {} X {} == {}", body().offset_height(), window().scroll_y().unwrap(), body().offset_height() as f64 - window().scroll_y().unwrap());

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        // console_log!("{}", window().scroll_y().unwrap());
        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(50)
}