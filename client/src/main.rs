use dioxus::prelude::*;
use views::*;

use crate::api::AcsVehicleInfo;

mod api;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico", ImageAssetOptions::new().with_avif());
const THEME_CSS: Asset = asset!("/assets/theme.css");
const STYLE_CSS: Asset = asset!("/assets/style.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Debug, Clone)]
pub struct TelemetryCtx(pub Signal<Option<AcsVehicleInfo>>);

#[component]
fn App() -> Element {
    use_context_provider(|| TelemetryCtx(Signal::new(None)));
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: THEME_CSS }
        document::Link { rel: "stylesheet", href: STYLE_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div { class: "main-container",
            div { class: "content-container",
                Outlet::<Route> {}
            }
        }
    }
}
