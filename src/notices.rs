use core::time::Duration;
use leptos::*;
use leptos_use::use_resize_observer;
#[component]
pub fn CountdownNotice() -> impl IntoView {
    let (countdown, set_countdown) = create_signal(3);
    let height = window().inner_height().unwrap().as_f64().unwrap();
    let width = window().inner_width().unwrap().as_f64().unwrap();

    let (x, set_x) = create_signal(width);
    let (y, set_y) = create_signal(height);

    let notice_element = create_node_ref::<leptos::svg::Text>();

    use_resize_observer(notice_element, move |entries, _observer| {
        let rect = entries[0].content_rect();
        set_x(width / 2.0 - rect.width() / 2.0);
        set_y(rect.height());
    });
    let interval = set_interval_with_handle(
        move || {
            set_countdown.update(|countdown| {
                *countdown -= 1;
            })
        },
        Duration::new(1, 0),
    );

    on_cleanup(|| {
        interval.unwrap().clear();
    });

    let notice_style = format!(
        "
            font-size:{}pt; 
            pointer-events:none; 
            opacity:30%; 
            fill: white;
        ",
        height * 0.5
    );

    view! {
            <text node_ref=notice_element x=x y=y style=&notice_style>{countdown}</text>
    }
}

#[component]
pub fn InitialNotice() -> impl IntoView {
    let height = window().inner_height().unwrap().as_f64().unwrap();
    let notice_style = format!(
        "
            position:absolute; 
            font-size:{}pt; 
            pointer-events:none; 
            height:100%; 
            width:100%; 
            opacity:30%; 
            display: flex;
            align-items: center;    
            justify-content: center;
            color: white;
            text-align:center;    
        ",
        height * 0.1
    );

    view! {
        <div style=&notice_style div>
            Place fingers on screen to begin
        </div>
    }
}
