use leptos::*;

use crate::components::cat::Cat;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"STUPID CAT APP"</h1>
        <Cat/>
    }
}