use yew::prelude::*;
use gloo_storage::{
    LocalStorage,
    Storage,
};
use web_sys::{
    Window,
    Navigator,
};
use gloo_console::{
    log as console_log,
    error as console_error,
};

mod components;
use components::{
	welcome::Welcome,
	field::Field,
};



pub enum LanguageAction {
	English,
	French,
}

#[derive(PartialEq, Debug)]
struct LanguageState {
    current: &'static str,
}


impl Default for LanguageState {
    fn default() -> Self {
		// Get the lang stored in local storage
		let window: Window = web_sys::window().expect("No Window Object!");
		let navigator: Navigator = window.navigator();
		let browser_language: Option<String> = navigator.language();

		let ls_lang: &str = match browser_language {
	    	Some(lang) => match lang.as_str() {
	    		"fr" | "fr-be" | "fr-ca" | "fr-ch" | "fr-lu" => "fr",
			    "en-US" | "en" | _ => "en",
	    	},
		    None => "en",
		};
		
		match ls_lang {
	    	"fr" => Self { current: "fra" },
	    	 _ => {
			 	Self { current: "eng" }
	    	}
		}
    }
}

impl Reducible for LanguageState {
	type Action = LanguageAction;
	fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
    	let next_lang =  match action {
    		LanguageAction::English => "fra",
    		LanguageAction::French => "eng"
    	};
    	
    	Self { current: next_lang, }.into()
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct AppContext {
	language: UseReducerHandle<LanguageState>,
}

#[function_component(App)]
fn app() -> Html {
    let language: UseReducerHandle<LanguageState> = use_reducer(LanguageState::default);
    
    {
    	let language: UseReducerHandle<LanguageState> = language.clone();
    	use_effect_with_deps(move |language: &UseReducerHandle<LanguageState>| {
    		match LocalStorage::set("lang", &language.current) {
    			Ok(()) => console_log!(format!("Language set to {}", &language.current)),
    			_ => console_error!("Couldn't set LocalStorage. Please turn off the feature in your browser if possible"),
			};
    		|| ()
    	}, language.clone())
    }
    
    html!(
    	<ContextProvider<AppContext> context={AppContext {
	    	language: language,
		}}>
			<div class="main_container">
				<Welcome />
				<Field title="About me" fr_title="A propos de moi" 
					fr_contents="Moi" contents="Me" />
				<div class="fields-container">
					<Field title="Programming" fr_title="Programmation"
						fr_contents="pro" contents="prog" />
					<Field title="FOSS" fr_title="FOSS"
						fr_contents="Logi" contents="software" />
				</div>
			</div>
		</ContextProvider<AppContext>>
    )
}

fn main() {
    yew::start_app::<App>();
}