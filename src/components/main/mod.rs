use crate::{main::sidebar::Sidebar, components::{main::compose::Compose}};
use dioxus::prelude::*;
use sir::global_css;
use warp::{tesseract::Tesseract};

pub mod sidebar;
pub mod compose;

#[derive(PartialEq, Props)]
pub struct Props {
    tesseract: Tesseract,
}

#[allow(non_snake_case)]
pub fn Main(cx: Scope) -> Element {
    // Start UI
    global_css! {"
    .main {
        display: flex;
        text-align: center;
        width: 100%;
        height: 100%;
        flex-direction: row;
    }
    "}

    cx.render(rsx!{
        div {
            class: "main",
            Sidebar {},
            Compose {},
        }
    })
}