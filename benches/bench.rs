use criterion::{black_box, criterion_group, criterion_main, Criterion};
use json_value_search::Search;
use serde_json::*;

static BENCH_DATA: &'static str = r#"{
    "squadName": "Super hero squad",
    "homeTown": "Metro City",
    "formed": 2016,
    "secretBase": "Super tower",
    "position": { 
        "x": 412365161,
        "y": 1165165146
    },
    "active": true,
    "members": [
        {
            "name": "Molecule Man",
            "age": 29,
            "secretIdentity": "Dan Jukes",
            "powers": [
                "Radiation resistance",
                "Turning tiny",
                "Radiation blast"
            ]
        },
        {
            "name": "Madame Uppercut",
            "age": 39,
            "secretIdentity": "Jane Wilson",
            "powers": [
                "Million tonne punch",
                "Damage resistance",
                "Superhuman reflexes"
            ]
        },
        {
            "name": "Eternal Flame",
            "age": 1000000,
            "secretIdentity": "Unknown",
            "powers": [
                "Immortality",
                "Heat Immunity",
                "Inferno",
                "Teleportation",
                "Interdimensional travel"
            ]
        }
    ]
}"#;

fn search(c: &mut Criterion) {
    let value: Value = serde_json::from_str(BENCH_DATA).unwrap();
    c.bench_function("Bench json_value.search(/field/other_field).", |b| {
        b.iter(|| black_box(value.clone().search("/position/x")))
    });
    let value: Value = serde_json::from_str(BENCH_DATA).unwrap();
    c.bench_function("Bench json_value.search(/field/*/other_field).", |b| {
        b.iter(|| black_box(value.clone().search("/members/*/name")))
    });
    let value: Value = serde_json::from_str(BENCH_DATA).unwrap();
    c.bench_function("Bench json_value.search(/field/1/other_field).", |b| {
        b.iter(|| black_box(value.clone().search("/members/1/name")))
    });
    let value: Value = serde_json::from_str(BENCH_DATA).unwrap();
    c.bench_function("Bench json_value.search(/field/1).", |b| {
        b.iter(|| black_box(value.clone().search("/members/1")))
    });
    let value: Value = serde_json::from_str(BENCH_DATA).unwrap();
    c.bench_function("Bench json_value.search(/field/*/regex).", |b| {
        b.iter(|| black_box(value.clone().search("/members/*/secret.+")))
    });
}

criterion_group!(benches, search);
criterion_main!(benches);
