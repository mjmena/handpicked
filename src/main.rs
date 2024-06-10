mod notices;

use crate::notices::*;
use core::time::Duration;
use ev::PointerEvent;
use handpicked::*;
use leptos::*;
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
    let state = create_rw_signal(State::Preparing);
    let touches = create_rw_signal(Vec::<RwSignal<TouchPoint>>::new());

    let width = window().inner_width().unwrap().as_f64().unwrap();
    let height = window().inner_height().unwrap().as_f64().unwrap();
    let initial_radius = (f64::min(width, height) * 0.25) as i32;
    let radius = create_rw_signal(initial_radius);

    #[cfg(debug_assertions)]
    touches.update(|touches| {
        touches.push(create_rw_signal(TouchPoint {
            id: 199,
            x: 200,
            y: 200,
            color: "#3a86ff".to_string(),
        }));
    });
    create_effect(move |_| {
        log::info!("{}", state());
    });

    let state_view = move || match state() {
        State::Preparing => view! {
            <PreparingTouchZone state touches radius/>
        },
        State::Selecting => view! {
            <SelectingTouchZone state touches radius/>
        },
        State::Revealing => {
            view! {
                <RevealingTouches state touches radius />
            }
        }
        State::Resetting => {
            view! {
                <ResettingTouches state touches radius initial_radius />
            }
        }
    };

    let colors = ["#ffbe0b", "#fb5607", "#ff006e", "#8338ec", "#3a86ff"];
    let handle_pointer_down = move |event: PointerEvent| {
        if state() == State::Revealing || state() == State::Resetting {
            return;
        }
        let signal = create_rw_signal(TouchPoint {
            id: event.pointer_id(),
            x: event.x(),
            y: event.y(),
            color: colors[(event.pointer_id() % colors.len() as i32) as usize].to_string(),
        });
        touches.update(|touches| {
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
        if state() == State::Revealing || state() == State::Resetting {
            return;
        }
        touches.update(|touches| {
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
        <Show when=move || touches().is_empty()>
            <InitialNotice />
        </Show>
        <Show when=move||state()==State::Selecting>
            <CountdownNotice />
        </Show>
        <svg on:pointerdown=handle_pointer_down on:pointerup=handle_pointer_up on:pointermove=handle_pointer_move style="background-color:#b38b6d">
            {state_view}
        </svg>
    }
}

#[component]
fn PreparingTouchZone(
    state: RwSignal<State>,
    touches: RwSignal<Vec<RwSignal<TouchPoint>>>,
    radius: RwSignal<i32>,
) -> impl IntoView {
    //timer to create a 1 second buffer when a finger is placed to when countdown starts
    create_effect(move |_| {
        let result = set_timeout_with_handle(
            move || {
                state.set(State::Selecting);
            },
            core::time::Duration::new(1, 0),
        );
        if touches().len() < 2 {
            result.unwrap().clear();
        }
    });

    view! {
        <For each=touches key=|touch| touch().id children=move |touch|{
            view!{<Touch touch_point=touch.read_only() radius=radius.read_only() />}
        }/>
    }
}

#[component]
fn SelectingTouchZone(
    state: RwSignal<State>,
    touches: RwSignal<Vec<RwSignal<TouchPoint>>>,
    radius: RwSignal<i32>,
) -> impl IntoView {
    create_effect(move |previous_touches: Option<Vec<RwSignal<TouchPoint>>>| {
        if let Some(previous_touches) = previous_touches {
            if previous_touches != touches() {
                state.set(State::Preparing)
            }
        }
        touches()
    });

    let result = set_timeout_with_handle(
        move || {
            state.set(State::Revealing);
            touches.update(|touches| {
                let mut hasher = DefaultHasher::new();
                touches.hash(&mut hasher);
                let random = hasher.finish() % touches.len() as u64;
                *touches = vec![touches[random as usize]];
            });
        },
        core::time::Duration::new(3, 0),
    );

    on_cleanup(move || {
        result.unwrap().clear();
    });

    view! {
        <For each=touches key=|touch| touch().id children=move |touch|{
            view!{<Touch touch_point=touch.read_only() radius=radius.read_only() />}
        }/>
    }
}

#[component]
fn RevealingTouches(
    // signal with the current state of the app
    state: RwSignal<State>,
    // read signal that contain the current winning touches
    touches: RwSignal<Vec<RwSignal<TouchPoint>>>,
    radius: RwSignal<i32>,
) -> impl IntoView {
    let interval_handle = set_interval_with_handle(
        move || {
            radius.update(|radius| {
                *radius += 10;
            });
        },
        Duration::new(0, 20000000),
    );
    set_timeout(
        move || {
            interval_handle.unwrap().clear();
            state.set(State::Resetting);
        },
        Duration::new(2, 0),
    );

    view! {
        <For each=touches key=|touch| touch().id children=move |touch|{
            view!{<Touch touch_point=touch.read_only() radius=radius.read_only()/>}
        }/>
    }
}
#[component]
fn ResettingTouches(
    // signal with the current state of the app
    state: RwSignal<State>,
    // read signal that contain the current winning touches
    touches: RwSignal<Vec<RwSignal<TouchPoint>>>,
    radius: RwSignal<i32>,
    initial_radius: i32,
) -> impl IntoView {
    let interval_handle = set_interval_with_handle(
        move || {
            radius.update(|radius| {
                *radius -= 10;
            });
        },
        Duration::new(0, 16000000),
    );
    set_timeout(
        move || {
            interval_handle.unwrap().clear();

            state.set(State::Preparing);
            radius.set(initial_radius);
            touches.set(vec![]);
        },
        Duration::new(2, 0),
    );

    view! {
        <For each=touches key=|touch| touch().id children=move |touch|{
            view!{<Touch touch_point=touch.read_only() radius=radius.read_only()/>}
        }/>
    }
}

#[component]
fn Touch(touch_point: ReadSignal<TouchPoint>, radius: ReadSignal<i32>) -> impl IntoView {
    let size = move || radius() * 2;

    view! {
        <svg x={move || touch_point().x-radius()} y={move || touch_point().y-radius()} height=size width=size >
            <circle style="mix-blend-mode:screen; " r=radius cx=radius cy=radius fill=move || touch_point().color />
        </svg>
    }
}
