use yew::prelude::*;

#[function_component(Discord)]
pub fn discord() -> Html {
    use_effect(move || {
        _ = gloo::utils::window().location().assign("https://discord.gg/PprHXwV2VB");
    });

    html! {
        <p>{"Redirecting to https://discord.gg/PprHXwV2VB"}</p>
    }
}
