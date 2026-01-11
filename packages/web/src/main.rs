use dioxus::prelude::*;
use ui::Route;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    println!("Dioxus Web App Starting... REBUILT!");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Playfair+Display:ital,wght@0,400;0,700;1,400&display=swap"
        }

        Router::<Route> {}
    }
}
