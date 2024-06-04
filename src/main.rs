use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
enum Version {
    V1(V1),
    // V2(V2), // descomenta esse
    // V3(V3), // descomenta esse
}

#[derive(Serialize, Deserialize, Debug)]
struct V1 {
    field1: String,
    field2: i32,
}

// V1, removido o field2
#[derive(Serialize, Deserialize, Debug)]
struct V2 {
    field1: String,
}

// V1, adicionado o field3
#[derive(Serialize, Deserialize, Debug)]
struct V3 {
    field1: String,
    field2: i32,
    field3: Option<i32>,
}

fn main() {
    escreve(); // comenta esse
    carrega();
}

fn escreve() {
    let foo_to_write = Version::V1(V1 {
        field1: String::from("1111"),
        field2: 2222,
    });
    let text = bincode::serialize(&foo_to_write).unwrap();
    fs::write("out", text).unwrap();
}

fn carrega() {
    let text = fs::read("out").unwrap();
    let foo_loaded: Version = bincode::deserialize(&text).unwrap();
    dbg!(foo_loaded);
}
