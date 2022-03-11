use yew::{Component, Context, Html, Properties, html, Callback};
use web_sys::{InputEvent};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title_text: String,
    pub bold: String,
}

pub struct MyDefaultTitle { }

impl Component for MyDefaultTitle {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = &ctx.props().title_text;
        html! {
            <>
                <h2>{ title }</h2>
            </>
        }
    }
}