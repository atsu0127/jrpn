use yew::{Component, Context, Html, Properties, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub value: String,
}

pub struct MyListRow {}

impl Component for MyListRow {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let value = &ctx.props().value;
        html! {
            <li>{ value }</li>
        }
    }
}