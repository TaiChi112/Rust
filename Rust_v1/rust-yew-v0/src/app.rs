use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button style={format!("background-color: {}; color: {};", if *counter % 2 == 0 { "#4CAF50" } else { "#f44336" }, "#fff")} {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}