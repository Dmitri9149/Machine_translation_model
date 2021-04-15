// initial commit is based on 
// https://stackoverflow.com/questions/62771576/how-do-i-save-structured-data-to-file
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f6bd6ea542877293e8a85cd25419a77c

extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write, Result};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

fn load_world<P: AsRef<Path>>(path: P) -> World {
    if let Ok(mut file) = File::open(path) {
        let mut buf = vec![];
        if file.read_to_end(&mut buf).is_ok() {
            if let Ok(world) = serde_json::from_slice(&buf[..]) {
                return world;
            }
        }
    }
    
//There was no file, or the file failed to load, create a new World.
    World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }])
}

fn save_world<P: AsRef<Path>>(path: P, world: World) -> Result<()> {
    let mut f = File::create(path)?;
    let buf = serde_json::to_vec(&world)?;
    f.write_all(&buf[..])?;
    Ok(())
}

fn main() {
    //Let's do it twice to see if the changes save
    for _ in 0..2 {
        let mut world = load_world("data/stages/cached_world");
        
        println!("Loaded: {:?}", world);
        
        //Move all entities
        for mut i in world.0.iter_mut() {
            i.x += 10.0;
            i.y += 5.0;
        }
    
        let _ = save_world("data/stages/cached_world", world);
    }
}
