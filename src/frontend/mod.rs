use crate::common::models::*;

use anyhow::Error;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::macros::html;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;
use yew::{Component, ComponentLink, Html, ShouldRender};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    fetch_task: Option<FetchTask>,
}

enum Msg {
    AddOne,
    FetchReady(Vec<Festival>),
    FetchFailed,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(text) => {
                warn!("response: {:?}", text);
                self.fetch_task = None;
                false
            }
            Msg::FetchFailed => false,
            Msg::AddOne => {
                if self.fetch_task.is_none() {
                    let request = Request::get("/api/festivals").body(Nothing).unwrap();
                    let task = FetchService::fetch(
                        request,
                        self.link.callback(
                            |response: Response<Json<Result<Vec<Festival>, Error>>>| match response
                                .into_parts()
                            {
                                (_meta, Json(Ok(data))) => Msg::FetchReady(data),
                                (_meta, Json(Err(error))) => {
                                    error!("Fetch failed: {:?}", error);
                                    Msg::FetchFailed
                                }
                            },
                        ),
                    );
                    self.fetch_task = task.ok();
                }
                self.value += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <h1 class="mt-5">{ "Event Travel Planner" }</h1>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_frontend() {
    web_logger::init();

    let app_root = yew::utils::document()
        .query_selector("#main")
        .expect("can't get app root node for rendering")
        .expect("can't unwrap app root node");

    yew::App::<Model>::new().mount(app_root);
}
