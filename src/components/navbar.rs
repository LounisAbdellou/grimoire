use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "px-8 py-4",
            h1 {
                class: "text-xl primary-content",
                "Library"
            }
        }

        Outlet::<Route> {}
    }
}
