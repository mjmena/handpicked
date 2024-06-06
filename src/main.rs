use crate::leptos_dom::helpers::TimeoutHandle;
use core::fmt;
use ev::PointerEvent;
use leptos::*;
use log::info;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[derive(Clone, PartialEq)]
enum State {
    Preparing,
    Selecting,
    Revealing,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Preparing => write!(f, "Preparing"),
            State::Selecting => write!(f, "Selecting"),
            State::Revealing => write!(f, "Revealing"),
        }
    }
}

#[component]
fn App() -> impl IntoView {
    let default_radius = 100.0;
    let (state, set_state) = create_signal(State::Preparing);
    let (touches, set_touches) =
        create_signal(Vec::<(ReadSignal<TouchPoint>, WriteSignal<TouchPoint>)>::new());
    let (timer, set_timer) = create_signal::<Option<TimeoutHandle>>(None);

    create_effect(move |_| {
        //clear timer from previous group of touches
        if let Some(timer) = timer() {
            timer.clear();
        };

        if state() == State::Revealing {
            return;
        }

        if touches().len() > 0 {
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
                        info!("Selected: {}", selected_touch.0());
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

    let (radius, set_radius) = create_signal(default_radius);
    let (growth_value, set_growth_value) = create_signal(1.0);

    set_interval(
        move || {
            if radius() > default_radius {
                set_growth_value(-1.0)
            };
            if radius() < default_radius / 2.0 {
                set_growth_value(1.0)
            };
            set_radius.update(|radius| *radius += growth_value())
        },
        core::time::Duration::new(0, 20000000),
    );

    let handle_pointer_down = move |event: PointerEvent| {
        if state() == State::Revealing {
            return;
        }
        let signal = create_signal(TouchPoint {
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
        if state() == State::Revealing {
            return;
        }
        if let Some(pos) = touches()
            .iter()
            .position(|(touch, _)| touch().id == event.pointer_id())
        {
            let (_, set_touch) = touches()[pos];
            set_touch.set(TouchPoint {
                id: event.pointer_id(),
                x: event.x(),
                y: event.y(),
                color: "#ffbe0b".to_string(),
            })
        };
    };

    let handle_pointer_up = move |event: PointerEvent| {
        if state() == State::Revealing {
            return;
        }
        set_touches.update(|touches| {
            if let Some(pos) = touches
                .iter()
                .position(|(touch, _)| touch().id == event.pointer_id())
            {
                let (touch, set_touch) = touches.remove(pos);
                touch.dispose();
                set_touch.dispose();
            };
        });
    };

    view! {
        <div>{move || state().to_string() }</div>
        <svg class="h-screen w-screen" on:pointerdown=handle_pointer_down on:pointerup=handle_pointer_up on:pointermove=handle_pointer_move >
            <For each=touches key=|(touch,_)| touch().id children=move |(touch,_)|{
                view!{<Touch touch_point={touch} radius={radius}/>}
            }/>
        </svg>
    }
}
#[component]
fn Touch(touch_point: ReadSignal<TouchPoint>, radius: ReadSignal<f64>) -> impl IntoView {
    let (position, set_position) = create_signal((touch_point().x as f64, touch_point().y as f64));
    let (_, set_velocity) = create_signal((0.0, 0.0));

    let interval = set_interval_with_handle(
        move || {
            set_velocity.update(|(velocity_x, velocity_y)| {
                *velocity_x += (touch_point().x as f64 - position().0) * 0.01;
                *velocity_x *= 0.95;
                *velocity_y += (touch_point().y as f64 - position().1) * 0.01;
                *velocity_y *= 0.95;

                set_position.update(|(x, y)| {
                    *x += *velocity_x;
                    *y += *velocity_y;
                })
            });
        },
        core::time::Duration::new(0, 10000000),
    );

    on_cleanup(|| {
        interval.unwrap().clear();
    });

    let size = move || radius() * 2.0;

    view! {
        <svg x={move || position().0-radius()} y={move || position().1-radius()} height={size()} width={size()}>
          <circle r=radius() cx=radius() cy=radius() fill=move || touch_point().color
          />
        </svg>
    }
}
#[derive(Clone, Debug)]
struct TouchPoint {
    id: i32,
    x: i32,
    y: i32,
    color: String,
}

impl fmt::Display for TouchPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({},{})", self.id, self.x, self.y)
    }
}
