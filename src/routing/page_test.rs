use dioxus::prelude::*;
use icons::{AArrowDown, AlignCenterVertical, Check, CircleDollarSign};

use crate::components::demos::demo_tags_animated::DemoTagsAnimated;

#[component]
pub fn PageTest() -> Element {
    rsx! {
        div { class: "p-4 flex flex-col gap-4",
            h1 { "Test Page" }

            DemoTagsAnimated {}

            AArrowDown { class: "text-red-500" }
            Check { class: "text-green-500" }
            AlignCenterVertical { class: "size-14" }
            CircleDollarSign { class: "text-sky-500 size-10" }
        }
    }
}
