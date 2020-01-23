RustTask
===
Task for job interview.

## Requirements
`rust >= 1.36`

## Run
`cargo run --release`

## How it works
Program saves expected values to file `output.csv`, with data from files `input.csv`, `hotels.json`, `room_names.csv`. \
Program loads only key data from files `hotels.json` and `room_names.csv` in to `BTreeMap` with the value pointing to start of line in file. \
File names of all files are hardcoded in src file: `src/consts.rs`.
