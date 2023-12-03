# Advent of Code 2023
For each day and task a solution is placed in path `src/bin/day_{d}_{t}.rs` where:
- `{d}` - day number
- `{t}` - task number

Each of the solutions can be compiled into binary that reads input data from `STDIN` and outputs result to `STDOUT`.

I don't plan to publish built libraries

To run binary (using cargo) for a day `{D}` and task 
`{T}` use: 
```sh
cargo run --bin day{D}_{T}
```
e.g. for day 1 and task 2:
```sh
cargo run --bin day1_2
```
Binares can be built using
```sh
cargo build
```
For more info about run/build options see cargo documentation. 