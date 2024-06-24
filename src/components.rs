use error::Error;
use leptos::*;

/// Button component
/// It is a nice one!
#[component]
pub fn Button(count: ReadSignal<i32>, set_count: WriteSignal<i32>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 20);
            }

            class="btn btn-lg btn-default"
        >
            "Count: "
            {move || count()}
        </button>
    }
}

/// Progress bar component
#[component]
pub fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}

#[derive(Debug, Clone)]
struct NavItem {
    url: Option<String>,
    name: Option<String>,
}

impl Default for NavItem {
    fn default() -> Self {
        Self {
            url: Some("/".to_string()),
            name: Some("Home".to_string()),
        }
    }
}

impl NavItem {
    pub fn new(url: String, name: String) -> Result<Self, Error> {
        let nav_item = Ok(Self {
            url: Some(url),
            name: Some(name),
        });
        nav_item.to_owned()
    }
}

/// Top navigation component
#[component]
pub fn Navigation() -> impl IntoView {
    let navitems = vec![
        NavItem::new("/".to_string(), "Leptos".to_string()).unwrap(),
        NavItem::new("/bootstrap".to_string(), "Bootstrap".to_string()).unwrap(),
    ];
    view! {
        <nav class="navbar navbar-default">
            <div class="container-fluid">
                <div class="navbar-header">
                    <a class="navbar-brand" href="/">
                        Leptos
                    </a>
                </div>
                <div id="navbar" class="collapse navbar-collapse">
                    <ul class="nav navbar-nav">

                        {navitems
                            .into_iter()
                            .map(|n| {
                                view! {
                                    <li>
                                        <a href=n.url>{n.name}</a>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </div>
            </div>
        </nav>
    }
}
