# Xanadu

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/yuma140902/xanadu/ci.yml?logo=github&label=CI)](https://github.com/yuma140902/Xanadu/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/xanadu)](https://crates.io/crates/xanadu)
[![docs.rs](https://img.shields.io/docsrs/xanadu?logo=docsdotrs)](https://docs.rs/xanadu/latest/xanadu/)

A toy ECS library; works on Windows, macOS, Linux and WebAssembly.

## Benchmark

```sh
cargo bench
```

### Result

![image](https://github.com/yuma140902/Xanadu/assets/23431077/8b9ebbc4-548c-4375-b312-125eea83a660)

| Number of entities | 10^2 (ns) | 10^3 (ns) | 10^4 (μs) | 10^5 (μs) |
| ------------------ | --------- | --------- | --------- | --------- |
| xanadu             |       182 |      1289 |     15.55 |     470.5 |
| bevy_ecs           |       130 |      1145 |     13.54 |     222.5 |
| game_objects_vec   |       156 |      2656 |     37.29 |    1015   |
| game_objects_hash  |       271 |      3379 |    166.1  |    1904   |

- OS: Windows 11 Home 23H2
- CPU: Intel Core i7-12700
- RAM: 32GB

code: [bench_ecs.rs](./benches/bench_ecs.rs)

## Usage

```rust
use xanadu::ecs::dyn_pool::{Mut, World};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

fn main() {
    let mut world = World::builder().register_component::<Position>().build();
    for i in 0..5 {
        let entity = world.new_entity();
        world.attach_component(
            entity,
            Position {
                x: i as f64,
                y: i as f64,
                z: i as f64,
            },
        );
    }

    world.execute::<'_, Position, _>(&print_system);
    world.execute::<'_, Mut<Position>, _>(&shuffle_system);
    world.execute::<'_, Mut<Position>, _>(&increment_system);
    world.execute::<'_, Mut<Position>, _>(&shuffle_system);
    println!("Shuffled and incremented");
    world.execute::<'_, Position, _>(&print_system);
}

fn print_system(pos: &Position) {
    println!("Pos: [{}, {}, {}]", pos.x, pos.y, pos.z);
}

fn shuffle_system(pos: &mut Position) {
    let tmp = pos.x;
    pos.x = pos.y;
    pos.y = pos.z;
    pos.z = tmp;
}

fn increment_system(pos: &mut Position) {
    pos.x += 1.0;
    pos.y += 2.0;
    pos.z += 3.0;
}
```

## Tests

```sh
cargo t
```

```sh
wasm-pack test --node
```

```sh
wasm-pack test --firefox --headless -- --features test_in_browser
```

