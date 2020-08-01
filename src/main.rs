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

impl Display for AreaNotes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[AREA \"{}\"]", self.name)?;
        for room in self.room.iter() {
            writeln!(f, "{}", room)?;
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Room {
    coord: (usize, usize),
    name: String,
    feature: Option<Vec<Feature>>,
    note: Option<String>,
}

impl Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x, y) = self.coord;
        writeln!(f, "[ROOM ({:2},{:2}) \"{}\"]", x, y, self.name)?;
        if let Some(features) = &self.feature {
            for feature in features {
                write!(f, "{}", feature)?;
            }
        }

        Ok(())
    }
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

impl Display for Feature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Feature::Save { text } => {
                writeln!(f, "[SAVE TABLET]")?;
                writeln!(f, "\"\"\"\n{}\n\"\"\"", text)?;
            }
            Feature::Tablet { text, img } => {
                writeln!(f, "[TABLET]")?;
                writeln!(f, "\"\"\"\n{}\n\"\"\"", text)?;
                if let Some(_img) = img {
                    // writeln!(f, "{}", img);
                }
            }
            Feature::Skeleton { text } => {
                writeln!(f, "[SKELETON]")?;
                writeln!(f, "\"\"\"\n{}\n\"\"\"", text)?;
            }
            Feature::NPC { name, convos, store } => {
                writeln!(f, "[NPC]")?;
                if let Some(name) = name {
                    writeln!(f, "name = {}", name)?;
                }
                if let Some(convos) = convos {
                    for convo in convos.iter() {
                        writeln!(f, "\"\"\"\n{}\n\"\"\"", convo)?;
                    }
                }
                if let Some(_store) = store {}
            }
            Feature::Puzzle { description, clue, solution } => {
                writeln!(f, "[PUZZLE]")?;
                writeln!(f, "DESCRIPTION: \"{}\"", description)?;
                writeln!(f, "CLUE: \"{}\"", clue)?;
                writeln!(f, "SOLUTION:")?;
                writeln!(f, "\"\"\"\n{}\n\"\"\"", solution)?;
            }
            Feature::Treasure { name, puzzle } => {
                writeln!(f, "[TREASURE]")?;
                writeln!(f, "{}", name)?;
                if let Some(_puzzle) = puzzle {}
            }
            Feature::Door { door_type, connects_to, openned_by } => {
                //
            }
        }

        Ok(())
    }
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

    println!("{}", an);
}