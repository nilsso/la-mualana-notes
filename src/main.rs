#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};
use serde::export::fmt::Display;
use serde::export::Formatter;

// NOTE:
// https://serde.rs/enum-representations.html
// https://glade.gnome.org/
// https://developer.gnome.org/gtk3/stable/ch03.html

#[derive(Deserialize, Serialize, Debug)]
enum Area {
    Unknown,
    Surface,
    GateOfGuidance,
    MausoleumOfTheGiants,
    EndlessCorridor,
    TwinLabyrinths,
}

#[derive(Deserialize, Serialize, Debug)]
struct AreaNotes {
    name: String,
    room: Vec<Room>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Room {
    coord: (usize, usize),
    name: String,
    feature: Option<Vec<Feature>>,
    note: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
enum DoorType {
    Normal,
    Eye,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
enum Feature {
    // Holy Grail Tablet
    Save {
        text: String,
    },
    Tablet {
        text: String,
        img: Option<String>,
    },
    Skeleton {
        text: String,
    },
    NPC {
        name: Option<String>,
        convos: Option<Vec<String>>,
        store: Option<Vec<String>>,
    },
    Puzzle {
        description: String,
        clue: String,
        solution: String,
    },
    Treasure {
        name: String,
        puzzle: Option<String>,
    },
    Door {
        door_type: DoorType,
        connects_to: (Area, (usize, usize)),
        openned_by: Option<String>,
    },
}

fn main() {
    // let an = AreaNotes {
    //     name: "Test".to_string(),
    //     room: vec![
    //         Room {
    //             coord: (0, 0),
    //             name: "".to_string(),
    //             feature: Some(vec! [
    //                 Feature::Door {
    //                     door_type: DoorType::Eye,
    //                     connects_to: (Area::Surface, (0, 0)),
    //                     openned_by: None
    //                 }
    //             ]),
    //             note: None,
    //         }
    //     ],
    // };
    //
    // println!("{}", toml::to_string(&an).unwrap());

    // let p = "area-notes/gate_of_guidance/notes.toml";
    let p = "area-notes/mausoleum_of_the_giants/notes.toml";

    let s = read_to_string(p).unwrap();

    let an: AreaNotes = toml::from_str(s.as_str()).unwrap();

    println!("{:#?}", an);
}