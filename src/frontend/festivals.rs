use crate::common::models::Festival;
use crate::frontend::routes::Table;
use yew::{html, Html};

impl Table for Festival {
    fn title() -> &'static str {
        "Festivals"
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
                <td>{ &self.name }</td>
            </tr>
        }
    }
}
