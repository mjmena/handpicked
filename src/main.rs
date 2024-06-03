use ev::PointerEvent;
use leptos::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    let (touches, set_touches) =
        create_signal(Vec::<(ReadSignal<TouchPoint>, WriteSignal<TouchPoint>)>::new());
    let handle_pointer_down = move |event: PointerEvent| {
        let signal = create_signal(TouchPoint {
            id: event.pointer_id(),
            x: event.x(),
            y: event.y(),
            initial_time: chrono::Utc::now(),
        });
        set_touches.update(|touches| {
            touches.push(signal);
        });
    };

    let handle_pointer_move = move |event: PointerEvent| {
        log::debug!("moving {}", event.pointer_id());
        if let Some(pos) = touches()
            .iter()
            .position(|(touch, _)| touch().id == event.pointer_id())
        {
            let (touch, set_touch) = touches()[pos];
            set_touch.set(TouchPoint {
                id: event.pointer_id(),
                x: event.x(),
                y: event.y(),
                initial_time: touch().initial_time,
            })
        };
    };

    let handle_pointer_up = move |event: PointerEvent| {
        set_touches.update(|touches| {
            if let Some(pos) = touches
                .iter()
                .position(|(touch, _)| touch().id == event.pointer_id())
            {
                touches.remove(pos);
            };
        });
    };

    view! {
        <svg class="h-screen w-screen" on:pointerdown=handle_pointer_down on:pointerup=handle_pointer_up on:pointermove=handle_pointer_move >
            <For each=touches key=|(touch,_)| touch().id children=move |(touch,_)|{
                view!{<Touch touch_point={touch}/>}
            }/>
        </svg>
    }
}
#[component]
fn Touch(touch_point: ReadSignal<TouchPoint>) -> impl IntoView {
    let size = 50;
    let radius = size / 2;
    log::debug!("touching");

    view! {
        <svg x={move || touch_point().x-radius} y={move || touch_point().y-radius} height={size} width={size}>
          <circle r={size/2} cx={size/2} cy={size/2} fill="red" />
        </svg>
    }
}
#[derive(Clone)]
struct TouchPoint {
    id: i32,
    x: i32,
    y: i32,
    initial_time: chrono::DateTime<chrono::Utc>,
}
