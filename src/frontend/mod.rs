mod events;
mod festivals;
mod routes;

use crate::common::models::*;
use crate::frontend::routes::{Anchor, FetchModel, Router};
use wasm_bindgen::prelude::*;
use yew::macros::html;
use yew::{Component, ComponentLink, Html, ShouldRender};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use crate::frontend::routes::AppRoute;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <nav class="menu">
                    <ul>
                        <li><Anchor route = AppRoute::Home>{ "Home" }</Anchor></li>
                        <li><Anchor route = AppRoute::Festivals>{ "Festivals" }</Anchor></li>
                        <li><Anchor route = AppRoute::Events>{ "Events" }</Anchor></li>
                    </ul>
                </nav>
                <Router render = Router::render(Self::route) />
            </>
        }
    }
}

impl Model {
    fn route(switch: AppRoute) -> Html {
        match switch {
            AppRoute::Festivals => html! {<FetchModel<Festival> url = "/api/festivals" />},
            AppRoute::Events => html! {<FetchModel<Event> url = "/api/events" />},
            AppRoute::Home => html! {<div></div>},
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
