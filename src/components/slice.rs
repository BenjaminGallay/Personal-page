use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub lang: u8,
}

#[function_component(Slice)]
pub fn slice(properties: &Properties) -> Html {
    html! {
        <div class="slice">
            <div class="card unpressed"><img src="ressources/photo_crop.png"/></div>
            <div class="description">
                <h2 class="name">{"Benjamin Gallay"}</h2>

                if properties.lang == 0 {
                    <h1 class="line1">{"FRONTEND"}</h1>
                    <h1 class="line2">{"DEVELOPPER"}</h1>
                } else {
                    <h1 class="line1">{"DEVELOPPEUR"}</h1>
                    <h1 class="line2">{"FRONTEND"}</h1>
                }

            </div>
        </div>
    }
}
