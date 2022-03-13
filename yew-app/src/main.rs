mod components;

use yew::prelude::*;

struct Top {
    value: i64,
}

enum Msg {
    Add(i64),
}

impl Component for Top {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(v) => {
                self.value += v;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use crate::components::my_interval_form::MyInterValForm;
        use crate::components::my_default_title::MyDefaultTitle;
        use crate::components::my_list::MyList;
        use crate::components::my_list_row::MyListRow;
        let link = ctx.link();
        let add_action = link.callback(|v| Msg::Add(v));
        let minus_action = link.callback(|v: i64| Msg::Add(-v));
        let values = (1..=10).collect::<Vec<_>>();
        html! {
            <>
                <MyDefaultTitle title_text="Counter"/>
                <p>{ self.value }</p>
                <MyInterValForm button_text="add" action={ add_action } /><br/>
                <MyInterValForm button_text="minus" action={ minus_action } />
                <MyList>
                    { for values.into_iter().map(|num| {
                        html! {
                            <MyListRow value={ format!("ROW {}", num) } />
                        }
                    })}
                </MyList>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Top>();
}
