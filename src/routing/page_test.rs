use dioxus::prelude::*;
use icons::Check;

use crate::components::demos::demo_tags_animated::DemoTagsAnimated;

#[component]
pub fn PageTest() -> Element {
    rsx! {
        div { class: "p-4 flex flex-col gap-4",
            h1 { "Test Page" }

            DemoTagsAnimated {}

            Check { class: "text-red-500" }
        }
    }
}
