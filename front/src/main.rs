#![allow(non_snake_case)]
// Import the Dioxus prelude to gain access to the `rsx!` macro and the `Scope` and `Element` types.
use dioxus::prelude::*;

fn main() {
    // Launch the web application using the App component as the root.
    dioxus_web::launch(App);
}

// Define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
       div {
            style: "text-align: center; font-family: monospace;",
            b {
                style: "font-size: 50px",
                "<lithium>"
            }
       }
        div {
            class: "hyp",
            style: "padding-top: 200px;",

            a {
                href: "https://github.com/1ith1um",
                "<git>"
            }

        div {
            class: "hyp",

            a {
                href: "https://discord.com/users/435162411729944591",
                "<discord>"
            }
        }

        div {
            class: "hyp",

            a {
                href: "https://matrix.to/#/@paper101:matrix.org",
                "<matrix>"
            }
        }
        }
    })
}
