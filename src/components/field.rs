use crate::AppContext;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct FieldsProps {
    pub title: &'static str,
    pub fr_title: &'static str,
    pub contents: &'static str,
    pub fr_contents: &'static str,
    #[prop_or(None)]
    pub additional_info: Option<&'static str>,
    #[prop_or(None)]
    pub fr_additional_info: Option<&'static str>,
}

#[function_component(Field)]
pub fn whoami(props: &FieldsProps) -> Html {
    let context = use_context::<AppContext>().expect("No AppContext found");
    match context.language.current {
        "fra" => {
            html! {
                <div class="field-no-add" id={ props.title.to_lowercase() }>
                    <div class="title">
                        <p>{ props.fr_title }</p>
                    </div>
                    <div class="contents">
                        <p>{ props.fr_contents }</p>
                    </div>
                </div>
            }
        },
        "eng" | _ => {
            html! {
                <div class="field-no-add" id={ props.title.to_lowercase()}>
                    <div class="title">
                        <p>{ props.title }</p>
                    </div>
                    <div class="contents">
                        <p>{ props.contents }</p>
                    </div>
                </div>
            }
        }
    }
}