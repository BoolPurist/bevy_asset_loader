# Examples

These examples are simple Bevy Apps illustrating the capabilities of `bevy_asset_loader`. Run the examples with `cargo run --example <example>`.

| Example                                        | Description                                                            |
| ---------------------------------------------- | ---------------------------------------------------------------------- |
| [`two_collections.rs`](two_collections.rs)     | Load multiple asset collections                                        |
| [`dynamic_asset.rs`](dynamic_asset.rs)         | Load an image asset from a path resolved at run time                   |
| [`dynamic_asset_ron.rs`](dynamic_asset_ron.rs) | Load dynamic assets from a `.ron` file                                 |
| [`atlas_from_grid.rs`](atlas_from_grid.rs)     | Loading a texture atlas from a sprite sheet                            |
| [`standard_material.rs`](standard_material.rs) | Loading a standard material from a png file                            |
| [`init_resource.rs`](init_resource.rs)         | Inserting a `FromWorld` resource when all asset collections are loaded |
| [`no_loading_state.rs`](no_loading_state.rs)   | How to use asset collections without a loading state                   |

## Examples for stageless

The following examples use `iyes_loopless`, which implements ideas from Bevy's [Stageless RFC](https://github.com/bevyengine/rfcs/pull/45). All examples require the `stageless` feature.
Note that progress tracking needs `progress_tracking_stageless` feature together with `progress_tracking`.

| Example                                                    | Description                                      |
| ---------------------------------------------------------- | ------------------------------------------------ |
| [`stageless.rs](stageless.rs)                              | Basic example                                    |
| [`stageless_dynamic_asset.rs`](stageless_dynamic_asset.rs) | Load an image asset from a path resolved runtime |
| [`stageless_progress.rs`](stageless_progress.rs)           | Stageless with progress tracking                 |
| [`stageless_dynamic_ron.rs](stageless_dynamic_ron.rs)      | Stageless with ron loading                       |

## Credits

The examples include third party assets:

Background audio: [CC BY 3.0](https://creativecommons.org/licenses/by/3.0/) by [Jay_You](https://freesound.org/people/Jay_You/sounds/460432/)

Toon character sheets [CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/) by [Kenny](https://kenney.nl/assets/toon-characters-1)
