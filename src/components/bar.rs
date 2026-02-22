use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub dark_theme: bool,
    pub lang: u8,
    pub lang_changer: Callback<u8>,
    pub theme_changer: Callback<bool>,
}

#[function_component(Bar)]
pub fn bar(properties: &Properties) -> Html {
    let change_lang = |new_lang: u8| {
        let lang_changer = properties.lang_changer.clone();
        move |_| {
            lang_changer.emit(new_lang);
        }
    };
    
    let change_theme = |new_theme: bool| {
        let theme_changer = properties.theme_changer.clone();
        move |_| {
            theme_changer.emit(new_theme);
        }
    };

    html!{
        <div class="bar">

            if properties.dark_theme {
                <button class="unpressed" onclick={change_theme(false)}><img src="ressources/light.svg" class="icon toinvert" /></button>
                <button class="pressed"><img src="ressources/dark.svg" class="icon toinvert" /></button>
            } else {
                <button class="pressed"><img src="ressources/light.svg" class="icon toinvert" /></button>
                <button class="unpressed" onclick={change_theme(true)}><img src="ressources/dark.svg" class="icon toinvert" /></button>
            }
            
            <div class="filler"><div class="horizontal_groove pressed"></div></div>

            if properties.lang == 0 { //English: 0, French: 1
                <button class="pressed"><img src="ressources/english.svg" class="icon" /></button>
                <button class="unpressed" onclick={change_lang(1)}><img src="ressources/french.svg" class="icon" /></button>
            } else {
                <button class="unpressed" onclick={change_lang(0)}><img src="ressources/english.svg" class="icon" /></button>
                <button class="pressed"><img src="ressources/french.svg" class="icon" /></button>
            }
            
            
        </div>
    }
}