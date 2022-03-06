use yew::{Component, Context, Html, Properties, html, Callback};
use web_sys::{InputEvent};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub button_text: String,
    pub action: Callback<i64>
}

pub enum Msg {
    Tap,
    OnInput(InputEvent),
}

pub struct MyInterValForm {
    pub value: i64,
}

impl Component for MyInterValForm {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 1
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tap => {
                let action = &ctx.props().action;
                action.emit(self.value);
                true
            },
            Msg::OnInput(e) => {
                let old_value = self.value.to_string();
                if let Some(value) = e.data() {
                    if let Ok(value) = value.parse::<i64>() {
                        let new_value = format!("{}{}", old_value, value);
                        log::info!("Update: {:?}", new_value);
                        self.value = new_value.parse().unwrap();
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Tap);
        let oninput = ctx.link().callback(|e: InputEvent| Msg::OnInput(e));
        let value = self.value.to_string();
        html! {
            <>
                <input type="number" { oninput } { value }/>
                <button { onclick }>{ &ctx.props().button_text }</button>
            </>
        }
    }
}