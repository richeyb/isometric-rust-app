//use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

extern crate data;
use data::DataResponse;

#[derive(Debug)]
pub enum Msg {
    InitialFetch,
    ReceiveResponse(Result<DataResponse, anyhow::Error>),
}

pub struct Fetcher {
    fetch_task: Option<FetchTask>,
    data: Option<DataResponse>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Fetcher {
    fn view_status(&self) -> Html {
        match self.data {
            Some(ref data) => {
                if data.status == "Ok" {
                    html! {
                        <p>{"Got a response!"}</p>
                    }
                } else {
                    html! {
                        <p>{"Server error!"}</p>
                    }
                }
            }
            None => {
                html! {
                    <p>{"Server is no good!"}</p>
                }
            }
        }
    }
}

impl Component for Fetcher {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let me = Self {
            fetch_task: None,
            data: None,
            link,
            error: None,
        };
        me.link.send_message(Msg::InitialFetch);
        me
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InitialFetch => {
                let request = Request::get("/api/data")
                    .body(Nothing)
                    .expect("Could not build request.");

                let callback = self.link
                    .callback(|res: Response<Json<Result<DataResponse, anyhow::Error>>>| {
                        let Json(data) = res.into_body();
                        Msg::ReceiveResponse(data)
                    });

                let task = FetchService::fetch(request, callback)
                    .expect("Failed to start request");

                self.fetch_task = Some(task);
                true
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(data) => {
                        self.data = Some(data);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string());
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {self.view_status()}
            </div>
        }
    }
}