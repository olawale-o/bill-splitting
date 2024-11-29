use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>Hello world</h1>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
