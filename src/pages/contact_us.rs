use yew::prelude::*;
use crate::{
    log,
    components::interval_task::ProgressDelay
};

#[derive(Debug)]
pub enum Msg
{
    AddOne
}

use wasm_bindgen::prelude::*;
pub struct Contact
{
    link: ComponentLink<Self>,
    number: u64
}
impl Component for Contact {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self
        {
            link,
            number: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg
        {
            Msg::AddOne => {
                self.number += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html
    {
        // run();
        html!{
            <>
            <div class="level-item google" style="overflow: hidden; padding-top: 80px;">
                <iframe src="https://docs.google.com/forms/d/e/1FAIpQLSdW5Mvu6yxv8udeACh1anwWpGRK3510nbt-uwlZPYxddesZ_Q/viewform?embedded=true" width="640" height="1000" frameborder="0" marginheight="0" marginwidth="0" style="overflow: hidden">{"A carregar…"}</iframe>
            </div>
                <div class="imgs">
                <a href="https://source.unsplash.com/600x600/?sig=1" data-lightbox="homePortfolio">
                    <img src="https://source.unsplash.com/600x600/?sig=1"/>
                </a>
                
                <a href="https://source.unsplash.com/600x800/?sig=12" data-lightbox="homePortfolio" class="vertical">
                    <img src="https://source.unsplash.com/600x800/?sig=12"/>
                </a>
                
                <a href="https://source.unsplash.com/800x600/?sig=71" data-lightbox="homePortfolio" class="horizontal">
                    <img src="https://source.unsplash.com/800x600/?sig=71"/>
                </a>
                
                <a href="https://source.unsplash.com/600x600/?sig=40" data-lightbox="homePortfolio">
                    <img src="https://source.unsplash.com/600x600/?sig=40"/>
                </a>
                
                <a href="https://source.unsplash.com/600x600/?sig=32" data-lightbox="homePortfolio">
                    <img src="https://source.unsplash.com/600x600/?sig=32"/>
                </a>
                
                <a href="https://source.unsplash.com/800x800/?sig=7" data-lightbox="homePortfolio" class="big">
                    <img src="https://source.unsplash.com/800x800/?sig=7"/>
                </a>
                
                <a href="https://source.unsplash.com/600x600/?sig=111" data-lightbox="homePortfolio">
                    <img src="https://source.unsplash.com/600x600/?sig=111"/>
                </a>
                
                <a href="https://source.unsplash.com/600x800/?sig=94" data-lightbox="homePortfolio" class="vertical">
                    <img src="https://source.unsplash.com/600x800/?sig=94"/>
                </a>
                
                <a href="https://source.unsplash.com/600x600/?sig=11" data-lightbox="homePortfolio">
                    <img src="https://source.unsplash.com/600x600/?sig=11"/>
                </a>
                
                <a href="https://source.unsplash.com/800x600/?sig=68" data-lightbox="homePortfolio" class="horizontal">
                    <img src="https://source.unsplash.com/800x600/?sig=68"/>
                </a>
                
                <a href="https://source.unsplash.com/600x600/?sig=24" data-lightbox="homePortfolio">
                    <img src="https://source.unsplash.com/600x600/?sig=24"/>
                </a>
                
                <a href="https://source.unsplash.com/800x800/?sig=55" data-lightbox="homePortfolio" class="big">
                    <img src="https://source.unsplash.com/800x800/?sig=55"/>
                </a>
                
                <a href="https://source.unsplash.com/600x600/?sig=56" data-lightbox="homePortfolio">
                    <img src="https://source.unsplash.com/600x600/?sig=56"/>
                </a>
                
                <a href="https://source.unsplash.com/800x600/?sig=186" data-lightbox="homePortfolio" class="horizontal">
                    <img src="https://source.unsplash.com/800x600/?sig=186"/>
                </a>
                
                <a href="https://source.unsplash.com/600x600/?sig=117" data-lightbox="homePortfolio">
                    <img src="https://source.unsplash.com/600x600/?sig=117"/>
                </a>
                
                <a href="https://source.unsplash.com/800x800/?sig=157" data-lightbox="homePortfolio" class="big">
                    <img src="https://source.unsplash.com/800x800/?sig=157"/>
                </a>
                
                <a href="https://source.unsplash.com/600x600/?sig=287" data-lightbox="homePortfolio">
                    <img src="https://source.unsplash.com/600x600/?sig=287"/>
                </a>
                
                <a href="https://source.unsplash.com/600x800/?sig=307" data-lightbox="homePortfolio" class="vertical">
                    <img src="https://source.unsplash.com/600x800/?sig=307"/>
                </a>
                </div>
            // <ProgressDelay duration_ms=15u64 on_complete=self.link.callback(|_| Msg::AddOne) />
            // <h1>{self.number}</h1>
                // <div class="content">
                //     <div class="tabs">
                //         <input type="radio" id="radio-1" name="tabs" checked=true />
                //         <label class="tab" for="radio-1">{"Episódios"}<span class="notification">{"25"}</span></label>
                //         <input type="radio" id="radio-2" name="tabs" />
                //         <label class="tab" for="radio-2">{"Info"}</label>
                //         <input type="radio" id="radio-3" name="tabs" />
                //         <label class="tab" for="radio-3">{"Relacionados"}</label>
                //         <span class="glider"></span>
                //     </div>
                // </div>
            </>
            // <video id="my-video" class="video-js" controls=true preload="auto" data-setup="\0">
            //     <source src="https://5.orezraey.workers.dev/0:/Animes/Ergo%20Proxy/Ergo%20Proxy%20-%2001%20%5B1080p%5D%5BM%C3%BAltiplas%20Legendas%5D.mkv" type="video/x-motroska"/>
            // </video>
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

pub fn run() -> Result<(), JsValue> {
    console_log!("log is on ✅");
    console_log!("{}", body().offset_height());

    // log.unwrap().set_text_content(Some("cu"));
    // window().onscroll() = Function.call1();
    // document().onscroll();
    // console_log!("{}", format!("{}", google_area.unwrap().unwrap().scroll_top()));
    // body().append_child(&val)?;
    Ok(())
}