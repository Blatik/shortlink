use leptos::*;
use leptos_router::*;
use crate::pages::home::Home;
use crate::pages::analytics::Analytics;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/analytics/:code" view=Analytics/>
                </Routes>
            </main>
        </Router>
    }
}
