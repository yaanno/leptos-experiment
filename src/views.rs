use leptos::*;
use crate::components::{Button, ProgressBar};
#[component]
pub fn Home() -> impl IntoView {
     let (count, set_count) = create_signal(0);
    view! {
        <div class="jumbotron">
            <h1>Leptos & Bootstrap</h1>
            <p>
                This example is a quick exercise to illustrate how the default,
                static navbar and fixed to top navbar work. Leptos provies reactivity.
            </p>
            <p>
                <ProgressBar progress=count/>
            </p>
            <Button count=count set_count=set_count/>
        </div>
    }
}
