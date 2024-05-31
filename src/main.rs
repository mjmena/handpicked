use leptos::*;
use leptos_dom::logging::console_log;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    let (touches, set_touches) = create_signal(Vec::<TouchPoint>::new());

    view! {
        <div class="h-screen flex items-center justify-center" on:click= move |event| {
        console_log("help");
        set_touches.update(|touches| {
            touches.push(TouchPoint {
                x: 0,
                y: 0,
                initial_time: chrono::Utc::now(),
            });
        });
    }>
            <For each=touches key=|touch| touch.initial_time children=move |touch|{
                view!{<div class="flex-1"><Touch/></div>}
            }/>
        </div>
    }
}
#[component]
fn Touch() -> impl IntoView {
    view! {
         <svg height="100" width="100">
          <circle r="45" cx="50" cy="50" fill="red" />
        </svg>
    }
}
#[derive(Clone)]
struct TouchPoint {
    x: isize,
    y: isize,
    initial_time: chrono::DateTime<chrono::Utc>,
}
