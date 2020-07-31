use crate::{Msg, Model};
use seed::{*,prelude::*};

pub fn sidebar(_model: &Model)->Node<Msg>{
    div![
        h1![
            "Sidebar"
        ],
        ul![
            li![a![attrs!{At::Href=>"/home"}, "/home"]],
            li![a![attrs!{At::Href=>"/app"}, "/app"]],
        ]
    ]
}