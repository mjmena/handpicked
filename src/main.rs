use crate::leptos_dom::helpers::TimeoutHandle;
use ev::PointerEvent;
use handpicked::*;
use leptos::*;
use log::info;
use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    let (state, set_state) = create_signal(State::Preparing);
    let (touches, set_touches) = create_signal(Vec::<RwSignal<TouchPoint>>::new());
    let (timer, set_timer) = create_signal::<Option<TimeoutHandle>>(None);

    create_effect(move |_| {
        //clear timer from previous group of touches
        if let Some(timer) = timer() {
            timer.clear();
        };

        if state() == State::Revealing {
            return;
        }

        if touches().len() > 1 {
            set_state(State::Selecting);
            let mut hasher = DefaultHasher::new();
            touches().hash(&mut hasher);
            let random = hasher.finish() % touches().len() as u64;

            let result = set_timeout_with_handle(
                move || {
                    info!("revealing");
                    set_state(State::Revealing);
                    set_touches.update(|touches| {
                        let selected_touch = touches[random as usize];
                        info!("Selected: {}", selected_touch());
                        *touches = vec![touches[random as usize]];
                    });
                    set_timeout(
                        move || {
                            set_state(State::Preparing);
                            set_touches(vec![]);
                        },
                        core::time::Duration::new(5, 0),
                    );
                },
                core::time::Duration::new(3, 0),
            );
            set_timer(Some(result.unwrap()));
        } else {
            set_state(State::Preparing);
        }
    });

    let width = window().inner_width().unwrap().as_f64().unwrap();
    let height = window().inner_height().unwrap().as_f64().unwrap();
    let radius = (f64::min(width, height) * 0.1) as i32;

    let handle_pointer_down = move |event: PointerEvent| {
        if state() == State::Revealing {
            return;
        }
        let signal = create_rw_signal(TouchPoint {
            id: event.pointer_id(),
            x: event.x(),
            y: event.y(),
            color: "#ffbe0b".to_string(),
        });
        set_touches.update(|touches| {
            touches.push(signal);
        });
    };

    let handle_pointer_move = move |event: PointerEvent| {
        if let Some(pos) = touches()
            .iter()
            .position(|touch| touch().id == event.pointer_id())
        {
            let touch = touches()[pos];
            touch.update(|touch| {
                *touch = TouchPoint {
                    x: event.x(),
                    y: event.y(),
                    ..touch.clone()
                }
            });
        };
    };

    let handle_pointer_up = move |event: PointerEvent| {
        if state() == State::Revealing {
            return;
        }
        set_touches.update(|touches| {
            if let Some(pos) = touches
                .iter()
                .position(|touch| touch().id == event.pointer_id())
            {
                let touch = touches.remove(pos);
                touch.dispose();
            };
        });
    };

    view! {
        <div style="position:absolute">{move || state().to_string() }</div>
        <svg class="h-screen w-screen" on:pointerdown=handle_pointer_down on:pointerup=handle_pointer_up on:pointermove=handle_pointer_move style=move || format!("background-color:{}", state().get_color()) >
            <For each=touches key=|touch| touch().id children=move |touch|{
                view!{<Touch touch_point={touch} radius={radius}/>}
            }/>
        </svg>
    }
}

#[component]
fn Touch(touch_point: RwSignal<TouchPoint>, radius: i32) -> impl IntoView {
    let size = move || radius * 2;

    view! {
        <svg x={move || touch_point().x-radius} y={move || touch_point().y-radius} height={size()} width={size()} >
            <circle r=radius cx=radius cy=radius fill=move || touch_point().color />
        </svg>
    }
}
