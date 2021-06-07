use yew::prelude::*;

use super::fetcher::Fetcher;

pub enum Msg {
    AddOne,
}

pub struct App {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container mx-auto bg-green-600">
                <p>{"Hello Nerds; what's up"}</p>
                <button class="" onclick=self.link.callback(|_| Msg::AddOne)>{ "+1 "}</button>
                <p>{ self.value }</p>
                <hr />
                <Fetcher />
            </div>
        }
    }
}