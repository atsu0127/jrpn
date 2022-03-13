use yew::{Component, Context, Html, Properties, html, Children};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

pub struct MyList {}

impl Component for MyList {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ol>
                { for ctx.props().children.iter() }
            </ol>
        }
    }
}