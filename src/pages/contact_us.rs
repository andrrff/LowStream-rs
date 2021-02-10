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
        run();
        html!{
            <>
            <div class="level-item google" style="overflow: hidden; padding-top: 80px;">
                <iframe src="https://docs.google.com/forms/d/e/1FAIpQLSdW5Mvu6yxv8udeACh1anwWpGRK3510nbt-uwlZPYxddesZ_Q/viewform?embedded=true" width="640" height="1000" frameborder="0" marginheight="0" marginwidth="0" style="overflow: hidden">{"A carregar…"}</iframe>
            </div>

            // <ProgressDelay duration_ms=15u64 on_complete=self.link.callback(|_| Msg::AddOne) />
            // <h1>{self.number}</h1>
                // <div class="container_changing">
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

// fn media() -> web_sys::HtmlMediaElement {
//     web_sys::HtmlMediaElement
// }

pub fn run() -> Result<(), JsValue> {
    console_log!("log is on ✅");
    console_log!("{}", body().offset_height());
    // let val: web_sys::HtmlMediaElement;
    // val.set_src("hello.mp4");
    

    // log.unwrap().set_text_content(Some("cu"));
    // window().onscroll() = Function.call1();
    // document().onscroll();
    // console_log!("{}", format!("{}", google_area.unwrap().unwrap().scroll_top()));
    // body().append_child(&val)?;
    Ok(())
}