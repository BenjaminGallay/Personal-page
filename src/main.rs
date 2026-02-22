mod components;
use yew::prelude::*;
use components::bar::Bar;
use components::slice::Slice;
use components::selector::Selector;
use components::content::Content;
use web_sys::window;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let lang = use_state(|| 0_u8);
    let cloned_lang = lang.clone();
    let new_lang = Callback::from({
        move |new_lang: u8| {
            cloned_lang.set(new_lang);
        }
    });

    let dark_theme = use_state(|| false);
    let cloned_theme = dark_theme.clone();
    let new_theme = Callback::from({
        move |new_theme: bool| {
            cloned_theme.set(new_theme);
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        if new_theme { //dark theme
                            html.set_attribute("style", "--bg-color: #202020;
                                                    --primary: orange;
                                                    --secondary: purple;
                                                    --color: white;
                                                    --shadow: #000000;
                                                    --light: #404040;
                                                    --inversion: 100%;
        
                            ").unwrap();
                        } else { //light theme
                            html.set_attribute("style", "--bg-color: #dfdfdf;
                                                        --primary: orange;
                                                        --secondary: purple;
                                                        --color: black;
                                                        --shadow: #bfbfbf;
                                                        --light: #ffffff;
                                                        --inversion: 0%;
        
                            ").unwrap();
                        }
                    }
                }
            }
        }
    });

    let selected_page = use_state(|| 0_u8);
    let cloned_page = selected_page.clone();
    let page_selector = Callback::from({
        move |new_page: u8| {
            cloned_page.set(new_page)
        }
    });

    html!{
        <>
            <Bar dark_theme={*dark_theme} lang={*lang} lang_changer={new_lang} theme_changer={new_theme} />
            <Slice lang={*lang} />
            <Selector selected_page={*selected_page} page_selector={page_selector} />
            <div class="content">
                <Content selected_page={*selected_page} lang={*lang} />
            </div>
        </>
    }
}
