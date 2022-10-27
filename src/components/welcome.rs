use yew::prelude::*;

use crate::AppContext;
use crate::components::icons::{
    Anki,
    Rust,
    Python,
    HTML,
    CSS,
    Asm,
    Wasm,
    Linux,
    Archlinux,
    Shell,
    Github,
    Lapce,
    VsCode,
    Neovim,
};

#[function_component(Welcome)]
pub fn welcome() -> Html {
    html! {
        <div class="welcome">
            <WelcomeBox />
            <Anki />
            <Rust />
            <Python />
            <HTML />
            <CSS />
            <Asm />
            <Wasm />
            <Linux />
            <Archlinux />
            <Shell />
            <Github />
            <Lapce />
            <VsCode />
            <Neovim />
        </div>
    }
}

#[function_component(WelcomeBox)]
fn welcome_box() -> Html {
    let app_context = use_context::<AppContext>().expect("No AppContext found");
    
    fn handle_welcome_message(app_context: AppContext) -> String {
        match app_context.language.current {
            "fra" => String::from("Bienvenue !"),
            "eng" | _ => String::from("Welcome !"),
        }
    }
    
    html! {
        <div class="welcome_box">
            <p>{ handle_welcome_message(app_context) }</p>
        </div>
    }
}