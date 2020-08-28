use crate::common::models::Event;
use crate::frontend::routes::Table;
use yew::{html, Html};

impl Table for Event {
    fn title() -> &'static str {
        "Events"
    }
    fn render_header() -> Html {
        html! {
            <tr>
                <th>{ "#" }</th>
                <th>{ "Name" }</th>
            </tr>
        }
    }
    fn render_row(&self) -> Html {
        html! {
            <tr>
                <td>{ self.id }</td>
                <td>{ &self.name.as_deref().unwrap_or("") }</td>
            </tr>
        }
    }
}
