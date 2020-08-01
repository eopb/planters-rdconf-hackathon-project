use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::px; // almost always want seed-style px instead of seed px
use seed_style::{pc, *};

use crate::raf_loop;
use crate::sound_scheduler::*;

#[topo::nested] // Needed for Seed Hooks
pub fn view(model: &Model) -> Node<Msg> {
    raf_loop::raf_loop_atom().get();

    div![
        s().flex("0 1 100%"),
        play_controls(model),
        schedule_controls(),
        current_schedule(model),
    ]
}

#[topo::nested]
pub fn play_controls(model: &Model) -> Node<Msg> {
    div![
        button![
            s().bg_color("green")
                .radius(px(4))
                .color("white")
                .px(px(18))
                .py(px(8))
                .m(px(4))
                .cursor_pointer(),
            "Start Event Loop (aka Play)",
            raf_loop::raf_loop_atom().on_click(|raf| raf.start()),
        ],
        button![
            s().bg_color("red")
                .radius(px(4))
                .color("white")
                .px(px(18))
                .py(px(8))
                .m(px(4))
                .cursor_pointer(),
            "Stop Event Loop (aka Stop)",
            raf_loop::raf_loop_atom().on_click(|raf| raf.stop()),
        ],
        p![
            "Current Time:",
            format!("{:.2}", model.secs_elapsed()),
            ", Current Time step:",
            model.current_time_step
        ]
    ]
}

#[topo::nested]
pub fn schedule_controls() -> Node<Msg> {
    // TODO fixme
    panic!()
    // let new_scheduled_sound_name = use_state(|| "".to_string());
    // let new_scheduled_sound_time = use_state(|| 0.0 as f64);
    // div![
    //     s().style_children(&["label", "input"]).display_block(),
    //     s().style_child("input")
    //         .b_style_solid()
    //         .b_width(px(1))
    //         .b_color("gray")
    //         .radius(px(3))
    //         .px(px(8))
    //         .py(px(4)),
    //     s().style_child("label").px(px(8)).py(px(4)).my(px(8)),
    //     label!["Add Sound at time:"],
    //     input![new_scheduled_sound_time.bind(At::Value),],
    //     label!["With Name"],
    //     input![
    //         attrs!(At::Value => new_scheduled_sound_name.get()),
    //         new_scheduled_sound_name.on_input(|n, inp| *n = inp),
    //     ],
    //     button![
    //         s().bg_color("blue")
    //             .radius(px(4))
    //             .color("white")
    //             .px(px(18))
    //             .py(px(8))
    //             .m(px(4))
    //             .cursor_pointer(),
    //         "Add Sound",
    //         mouse_ev(Ev::Click, move |_| {
    //             let sound_msg = SoundSchedulerMsg::AddSound(
    //                 new_scheduled_sound_time.get(),
    //                 new_scheduled_sound_name.get(),
    //             );

    //             new_scheduled_sound_time.set(0.0);
    //             new_scheduled_sound_name.set("".to_string());

    //             Msg::Scheduler(sound_msg)
    //         })
    //     ],
    //     pre![
    //         "Time: ",
    //         new_scheduled_sound_time.get(),
    //         "secs",
    //         " , Sound name: ",
    //         new_scheduled_sound_name.get()
    //     ],
    // ]
}

pub fn current_schedule(model: &Model) -> Node<Msg> {
    panic!();
    div![
        h2!["Scheduled Sounds"],
        // TODO Fixme
        // ul![model.sound_scheduler.schedule.iter().map(|(ts, sound)| li![
        //     {
        //         if sound.played {
        //             s().color("green").font_weight_v900()
        //         } else {
        //             s().font_weight_v300()
        //         }
        //     },
        //     "At secs: ",
        //     (*ts as f64) / 60.0,
        //     ",
        //             At time step: ",
        //     ts,
        //     ",
        //             Sound name: ",
        //     &sound.data,
        // ])]
    ]
}
