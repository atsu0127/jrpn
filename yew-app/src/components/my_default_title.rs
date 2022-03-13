use yew::{Component, Context, Html, Properties, html};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or("default".to_string())]
    pub title_text: String,

    #[prop_or("on".to_string())]
    pub italic: String,
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
        let italic = if &ctx.props().italic == "on" { true } else { false };
        html! {
            <>
                {
                    if italic {
                        html! {
                            <i><h2>{ title }</h2></i>
                        }
                    } else {
                        html! {
                            <h2>{ title }</h2>
                        }
                    }
                }
            </>
        }
    }
}