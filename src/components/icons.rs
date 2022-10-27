use yew::prelude::*;

#[function_component(Anki)]
pub fn anki() -> Html {
    html! {
        <div id="anki-cont">
            <img class="icon" src="../assets/icons8-anki.svg" alt="Anki icon" />
        </div>
    }
}

#[function_component(Rust)]
pub fn rust() -> Html {
    html! {
        <div id="rust-cont">
            <img class="icon" src="../assets/rust-svgrepo-com.svg" alt="Rust" />
        </div>
    }
}

#[function_component(Python)]
pub fn python() -> Html {
    html! {
        <div id="python-cont">
            <img class="icon" src="../assets/python-svgrepo-com.svg" alt="Python" />
        </div>
    }
}

#[function_component(HTML)]
pub fn html() -> Html {
    html! {
        <div id="html-cont">
            <img class="icon" src="../assets/html-svgrepo-com.svg" alt="HTML" />
        </div>
    }
}

#[function_component(CSS)]
pub fn css() -> Html {
    html! {
        <div id="css-cont">
            <img class="icon" src="../assets/css-3-logo-svgrepo-com.svg" alt="CSS" />
        </div>
    }
}

#[function_component(Asm)]
pub fn asm() -> Html {
    html! {
        <div id="asm-cont">
            <img class="icon" src="../assets/asm.svg" alt="x86-64 Assembly" />
        </div>
    }
}
#[function_component(Wasm)]
pub fn wasm() -> Html {
    html! {
        <div id="wasm-cont">
            <img class="icon" src="../assets/wasm-svgrepo-com.svg" alt="Web Assembly" />
        </div>
    }
}

#[function_component(Linux)]
pub fn linux() -> Html {
    html! {
        <div id="linux-cont">
            <img class="icon" src="../assets/linux-tux-svgrepo-com.svg" alt="Linux" />
        </div>
    }
}

#[function_component(Archlinux)]
pub fn archlinux() -> Html {
    html! {
        <div id="archlinux-cont">
            <img class="icon" src="../assets/archlinux-svgrepo-com.svg" alt="ArchLinux" />
        </div>
    }
}

#[function_component(Shell)]
pub fn shell() -> Html {
    html! {
        <div id="shell-cont">
            <img class="icon" src="../assets/bash-icon-svgrepo-com.svg" alt="Shell" />
        </div>
    }
}

#[function_component(Github)]
pub fn github() -> Html {
    html! {
        <div id="github-cont">
            <img class="icon" src="../assets/github-svgrepo-com.svg" alt="Github" />
        </div>
    }
}

#[function_component(Lapce)]
pub fn lapce() -> Html {
    html! {
        <div id="lapce-cont">
            <img class="icon" src="../assets/lapce.svg" alt="Lapce" />
        </div>
    }
}

#[function_component(VsCode)]
pub fn vscode() -> Html {
    html! {
        <div id="vscode-cont">
            <img class="icon" src="../assets/vscode-svgrepo-com.svg" alt="VSCode" />
        </div>
    }
}

#[function_component(Neovim)]
pub fn neovim() -> Html {
    html! {
        <div id="neovim-cont">
            <img class="icon" src="../assets/neovimio-icon.svg" alt="Neovim" />
        </div>
    }
}
