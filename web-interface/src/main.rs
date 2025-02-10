use parser::prelude::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let output_state = use_state(|| String::new());
    let output_state_clone = output_state.clone();
    let handle_input = Callback::from(move |event: Event| {
        let input = event.target_unchecked_into::<web_sys::HtmlInputElement>();
        let value = input.value();
        let mut output = String::new();
        let _ = Parser::new(&value).describe(&mut output, false);
        output_state_clone.set(output);
    });
    let description = (*output_state).clone();

    html! {
        <>
        <h1>{"METAR parser"}</h1>
        <div>
            <input type="text" onchange={handle_input} />
            <div>{ description }</div>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
