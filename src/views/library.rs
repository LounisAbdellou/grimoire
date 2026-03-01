use dioxus::prelude::*;

use crate::components::echo::Echo;

#[component]
pub fn Library() -> Element {
    rsx! {
        div {
            class: "px-8",
            Echo {}
        }
    }
}
