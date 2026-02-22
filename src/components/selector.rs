use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub selected_page: u8,
    pub page_selector: Callback<u8>,
}

#[function_component(Selector)]
pub fn selector(properties: &Properties) -> Html {

    let onclick = |i:u8| {
        let page_selector = properties.page_selector.clone();
        move |_| {
            page_selector.emit(i);
        }
    };

    html! {
        <div class="selector">
            <div class="spacer"></div>

            if properties.selected_page == 0 {
                <button class="pressed"><p>{"Who am I ?"}</p></button>
            } else {
                <button class="unpressed" onclick={onclick(0)}><p>{"Who am I ?"}</p></button>
            }

            <div class="inner_spacer"></div>

            if properties.selected_page == 1 {
                <button class="pressed"><p>{"Studies"}</p></button>
            } else {
                <button class="unpressed" onclick={onclick(1)}><p>{"Studies"}</p></button>
            }
            
            <div class="inner_spacer"></div>

            if properties.selected_page == 2 {
                <button class="pressed"><p>{"Projects"}</p></button>
            } else {
                <button class="unpressed" onclick={onclick(2)}><p>{"Projects"}</p></button>
            }

            <div class="spacer"></div>
        </div>
    }
}