use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="h-screen flex items-center justify-center">
            <div class="flex-1 bg-purple-200 rounded-fully">
                <button
                    on:click=move |_| { set_count.update(|n| *n+=1); }
                    class:text-red-600=move || count() % 2 == 1
                >
                    "Click me: " {count}
                </button>
            </div>
        </div>
    }
}
