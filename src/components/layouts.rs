use maud::{html, Markup};
use crate::constants::{DATASTAR_SRC};

pub fn to_html(body: Markup) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link rel="stylesheet"
                 href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css";
            script
                src=(DATASTAR_SRC)
                defer {} }
        body { (body) } } }