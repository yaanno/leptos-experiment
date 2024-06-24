use leptos::*;
use leptos_router::*;
mod components;
mod views;
use crate::components::Navigation;
use crate::views::Home;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}

/** Main application view*/
#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="container">
                <Navigation/>
                <main>
                    <Routes>
                        <Route path="/" view=Home/>
                        // <Route path="/bootstrap" view=Bootstrap/>
                        // <Route path="/users" view=Users/>
                        // <Route path="/users/:id" view=UserProfile/>
                        <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                    </Routes>
                </main>

            </div>
        </Router>
    }
}
