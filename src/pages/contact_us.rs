use yew::prelude::*;
pub struct Contact;
impl Component for Contact {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div class="level-item" style="overflow: hidden; padding-top: 80px;">
                <iframe src="https://docs.google.com/forms/d/e/1FAIpQLSdW5Mvu6yxv8udeACh1anwWpGRK3510nbt-uwlZPYxddesZ_Q/viewform?embedded=true" width="640" height="1000" frameborder="0" marginheight="0" marginwidth="0" style="overflow: hidden">{"A carregar…"}</iframe>
            </div>
        }        
    }
}
