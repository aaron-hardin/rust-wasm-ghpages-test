mod event_bus;
mod producer;
mod subscriber;

use chrono::Local;
use producer::Producer;
use subscriber::Subscriber;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use yew::virtual_dom::VChild;

enum Msg {
    AddOne,
    UpdateText(String),
}

struct Model {
    value: i64,
    listy: Vec<i64>,
    content: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            listy: vec![1,2,3],
            content: "".to_owned()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.listy.push(self.value);
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Msg::UpdateText(content) => {
                self.content = content.to_uppercase();
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        let items: Vec<VChild<SimpleComponent>> = self
            .listy
            .iter()
            .enumerate()
            .map(|(ix, item)| {
                html_nested! {
                    <SimpleComponent value={item.clone()} include_nbsp={if ix == 0 {false} else {true}} />
                }
            }).collect();

        html! {
            <div>
                <HelloWorld />
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
                <div>
                    {
                        items
                    }
                </div>
                <input
                    type="text"
                    oninput={link.callback(|event: InputEvent| {
                        let input: HtmlInputElement = event.target_unchecked_into();
                        Msg::UpdateText(input.value())
                    })}
                    value={self.content.clone()}
                />
                <Producer />
                <Subscriber />
            </div>
        }
    }
}

#[function_component(HelloWorld)]
fn hello_world() -> Html {
    html! {
        <div>
            { "Hello world" }
            <RenderedAt time={Local::now().format("%Y-%m-%d %H:%M:%S").to_string()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SimpleComponentProps {
    pub value: i64,
    pub include_nbsp: bool,
}

#[function_component(SimpleComponent)]
pub fn simple_component(props: &SimpleComponentProps) -> Html {
    html! {
        <p style="display:inline-block;">
            { if props.include_nbsp { "\u{00a0}" } else { "" } }
            { &props.value }
        </p>
    }
}

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub time: String,
}

#[function_component(RenderedAt)]
pub fn rendered_at(props: &RenderedAtProps) -> Html {
    html! {
        <p>
            <b>{ "Rendered at: " }</b>
            { &props.time }
        </p>
    }
}

fn main() {
    //yew::start_app::<Model>();
    
    let document = gloo_utils::document();
    let mount_point = document.query_selector("div.mount_point").unwrap().unwrap();
    yew::start_app_in_element::<Model>(mount_point);
}

