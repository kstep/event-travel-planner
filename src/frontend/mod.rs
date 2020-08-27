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
    fetch_task: Option<FetchTask>,
    festivals: Vec<Festival>,
}

enum Msg {
    LoadFestivals,
    FetchFestivals(Vec<Festival>),
    FetchFailed,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
            festivals: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchFestivals(festivals) => {
                warn!("response: {:?}", festivals);
                self.fetch_task = None;
                self.festivals = festivals;
                true
            }
            Msg::FetchFailed => false,
            Msg::LoadFestivals => {
                if self.fetch_task.is_none() {
                    let request = Request::get("/api/festivals").body(Nothing).unwrap();
                    let task = FetchService::fetch(
                        request,
                        self.link.callback(
                            |response: Response<Json<Result<Vec<Festival>, Error>>>| match response
                                .into_parts()
                            {
                                (_meta, Json(Ok(data))) => Msg::FetchFestivals(data),
                                (_meta, Json(Err(error))) => {
                                    error!("Fetch failed: {:?}", error);
                                    Msg::FetchFailed
                                }
                            },
                        ),
                    );
                    self.fetch_task = task.ok();
                }
                false
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
        let festival_row = |festival: &Festival| html! {
            <tr>
                <td>{ festival.id }</td>
                <td>{ &festival.name }</td>
            </tr>
        };

        html! {
            <div class="container">
                <h1 class="mt-5">{ "Event Travel Planner" }</h1>
                <button onclick=self.link.callback(|_| Msg::LoadFestivals)>{ "load fests" }</button>

                <table>
                    { for self.festivals.iter().map(festival_row) }
                </table>
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
