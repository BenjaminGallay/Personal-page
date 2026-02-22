use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub selected_page: u8,
    pub lang: u8,
}

#[function_component(Content)]
pub fn content(properties: &Properties) -> Html {
    match properties.selected_page {
        0 => html! {
            <div class="whoami unpressed">
                if properties.lang == 0 {
                    <p>{"My name is Benjamin Gallay, and I am currently starting my studies in the Grande Ecole Centrale Supélec to study computer science."}</p>
                } else {
                    <p>{"Je m'appelle Benjamin Gallay, et je suis commence mes études au sein de la Grande Ecole de Centrale Supélec pour y étudier l'informatique."}</p>
                }

                <p>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}</p>
            </div>
        },
        1 => html! {
            <div class="studies">
                <div class="vertical_groove pressed"></div>
                <ul class="chronology">
                    if properties.lang == 0 {
                        <li>
                            <h3>{"Currently : Studying at Grande Ecole Centrale Supélec"}</h3>
                        </li>
                        <li>
                            <h3>{"2022 - 2024 : MP Preparatory Class"}</h3>
                            <p>{"At Lycée Champollion of Grenoble, France"}</p>
                            <p>{"Computer Science option"}</p>
                        </li>
                    } else {
                        <li>
                            <h3>{"En cours : Etudes à la Grande Ecole Centrale Supélec"}</h3>
                        </li>
                        <li>
                            <h3>{"2022 - 2024 : Classe préparatoire MP"}</h3>
                            <p>{"Au Lycée Champollion of Grenoble, France"}</p>
                            <p>{"option informatique"}</p>
                        </li>
                    }

                </ul>
            </div>
        },
        2 => html! {
            <div class="projects">
            <a class="project" tabindex="0" href="https://github.com/BenjaminGallay/Personal-page">
                <h3>{"Stupid Project"}</h3>
                <p>{"this is a short description of a Rust project experimenting with front-end web apps directly compiled from Rust"}</p>
            </a>
            <a class="project" tabindex="0" href="https://github.com/BenjaminGallay/Personal-page">
                <h3>{"Stupid Project"}</h3>
                <p>{"this is a short description of a Rust project experimenting with front-end web apps directly compiled from Rust"}</p>
            </a>
            <a class="project" tabindex="0" href="https://github.com/BenjaminGallay/Personal-page">
                <h3>{"Stupid Project"}</h3>
                <p>{"this is a short description of a Rust project experimenting with front-end web apps directly compiled from Rust"}</p>
            </a>
            <a class="project" tabindex="0" href="https://github.com/BenjaminGallay/Personal-page">
                <h3>{"Stupid Project"}</h3>
                <p>{"this is a short description of a Rust project experimenting with front-end web apps directly compiled from Rust"}</p>
            </a>
            <a class="project" tabindex="0" href="https://github.com/BenjaminGallay/Personal-page">
                <h3>{"Stupid Project"}</h3>
                <p>{"this is a short description of a Rust project experimenting with front-end web apps directly compiled from Rust"}</p>
            </a>
        </div>
        },
        _ => html! {
            <h1>{"Error"}</h1>
        },
    }
}
