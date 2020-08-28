use anyhow::Error;
use serde::de::DeserializeOwned;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::components::RouterAnchor;
use yew_router::router::Router as YewRouter;

#[derive(Debug, Switch, Clone, Copy)]
pub enum AppRoute {
    #[to = "/festivals"]
    Festivals,
    #[to = "/events"]
    Events,
    #[to = "/"]
    Home,
}

pub type Anchor = RouterAnchor<AppRoute>;
pub type Router = YewRouter<AppRoute>;
pub type JsonResponse<T> = Response<Json<Result<T, Error>>>;

pub enum FetchMsg<T> {
    FetchStart,
    FetchSucceed(T),
    FetchFailed,
}

pub struct FetchModel<T: 'static + DeserializeOwned + Table> {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    objects: Vec<T>,
    url: String,
}

#[derive(Properties, Clone)]
pub struct FetchProps {
    pub url: String,
}

pub trait Table {
    fn title() -> &'static str;
    fn render_header() -> Html;
    fn render_row(&self) -> Html;
}

impl<T: 'static + DeserializeOwned + Table> Component for FetchModel<T> {
    type Message = FetchMsg<Vec<T>>;
    type Properties = FetchProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
            objects: Vec::new(),
            url: props.url,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FetchMsg::FetchSucceed(objects) => {
                self.fetch_task = None;
                self.objects = objects;
                true
            }
            FetchMsg::FetchFailed => false,
            FetchMsg::FetchStart => {
                if self.fetch_task.is_none() {
                    let request = Request::get(&*self.url).body(Nothing).unwrap();
                    let task = FetchService::fetch(
                        request,
                        self.link.callback(|response: JsonResponse<Vec<T>>| {
                            match response.into_parts() {
                                (_meta, Json(Ok(data))) => FetchMsg::FetchSucceed(data),
                                (_meta, Json(Err(_error))) => FetchMsg::FetchFailed,
                            }
                        }),
                    );
                    self.fetch_task = task.ok();
                }
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <h1>{ <T as Table>::title() }</h1>
                <table class="table">
                    <thead>
                        { <T as Table>::render_header() }
                    </thead>
                    <tbody>
                        { for self.objects.iter().map(<T as Table>::render_row) }
                    </tbody>
                </table>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(FetchMsg::FetchStart);
        }
    }
}
