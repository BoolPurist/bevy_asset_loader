# Bevy asset loader

[![crates.io](https://img.shields.io/crates/v/bevy_asset_loader.svg)](https://crates.io/crates/bevy_asset_loader)
[![docs](https://docs.rs/bevy_asset_loader/badge.svg)](https://docs.rs/bevy_asset_loader)
[![license](https://img.shields.io/crates/l/bevy_asset_loader)](https://github.com/NiklasEi/bevy_asset_loader/blob/main/LICENSE.md)
[![crates.io](https://img.shields.io/crates/d/bevy_asset_loader.svg)](https://crates.io/crates/bevy_asset_loader)

This [Bevy][bevy] plugin reduces boilerplate when loading game assets. The crate offers the `AssetCollection` trait and can automatically load structs that implement it. The trait can be derived.

*The `main` branch and all current releases support Bevy version 0.5. If you like living on the edge, take a look at the `bevy_main` branch, which tries to stay close to Bevy's development.*

## How to use

The `AssetLoader` is constructed with two states (see [the cheatbook on states][cheatbook-states]). During the first state it will load the assets and check up on the loading status in every frame. When the assets are done loading, the collections will be inserted as resources, and the plugin switches to the second state.

For structs with named fields that are either asset handles or implement default, `AssetCollection` can be derived. You can add as many `AssetCollection`s to the loader as you want by chaining `with_collection` calls. To finish the setup, call the `build` function with your `AppBuilder`.

Now you can start your game logic from the second configured state and use the asset collections as resources in your systems.

```rust no_run
use bevy::prelude::*;
use bevy_asset_loader::{AssetLoader, AssetCollection};

fn main() {
  let mut app = App::new();
  AssetLoader::new(GameState::AssetLoading)
          .continue_to_state(GameState::Next)
          .with_collection::<ImageAssets>()
          .with_collection::<AudioAssets>()
          .build(&mut app);
  app.add_state(GameState::AssetLoading)
          .add_plugins(DefaultPlugins)
          .add_system_set(SystemSet::on_enter(GameState::Next).with_system(use_my_assets))
          .run();
}

#[derive(AssetCollection)]
struct AudioAssets {
  #[asset(path = "walking.ogg")]
  walking: Handle<AudioSource>
}

#[derive(AssetCollection)]
struct ImageAssets {
  #[asset(path = "images/player.png")]
  player: Handle<Image>,
  #[asset(path = "images/tree.png")]
  tree: Handle<Image>,
}

fn use_my_assets(_image_assets: Res<ImageAssets>, _audio_assets: Res<AudioAssets>) {
  // do something using the asset handles from the resources
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
  AssetLoading,
  Next,
}
```

See [two_collections.rs](/bevy_asset_loader/examples/two_collections.rs) for a complete example.

### Dynamic assets

It is possible to decide an asset file path at run time. This is done via the resource `AssetKeys` which is basically a map of asset keys to their file paths. The `AssetLoader` initializes the resource and reads it during the loading state. You should define all asset keys and their paths in a previous state.

```rust
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
struct ImageAssets {
  #[asset(key = "player")]
  player: Handle<Image>,
}
```

Take a look at the [dynamic_asset](bevy_asset_loader/examples/dynamic_asset.rs) example to see how this can work in your code.

*There will likely be additions to this feature in the future. The goal is to allow defining all the other asset attributes dynamically, too (e.g., texture atlas options).*

### Loading a folder as asset

You can load all assets in a folder and keep them in an `AssetCollection` as a vector of untyped handles.
```rust
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
struct MyAssets {
    #[asset(folder = "images")]
    folder: Vec<HandleUntyped>,
}
```

### Loading standard materials

You can directly load standard materials if you enable the feature `render`. For a complete example please take a look at [standard_material.rs](/bevy_asset_loader/examples/standard_material.rs).
```rust
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
struct MyAssets {
    #[asset(standard_material)]
    #[asset(path = "images/player.png")]
    player: Handle<StandardMaterial>,
}
```

### Loading texture atlases

You can directly load texture atlases from sprite sheets if you enable the feature `render`. For a complete example please take a look at [atlas_from_grid.rs](/bevy_asset_loader/examples/atlas_from_grid.rs).
```rust
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
struct MyAssets {
    #[asset(texture_atlas(tile_size_x = 100., tile_size_y = 96., columns = 8, rows = 1, padding_x = 12., padding_y = 12.))]
    #[asset(path = "images/sprite_sheet.png")]
    sprite: Handle<TextureAtlas>,
}
```

### Initialize FromWorld resources

In situations where you would like to prepare other resources based on your loaded assets you can use `AssetLoader::init_resource` to initialize `FromWorld` resources. See [init_resource.rs](/bevy_asset_loader/examples/init_resource.rs) for an example that loads two images and then combines their pixel data into a third image.

`AssetLoader::init_resource` does the same as Bevy's `App::init_resource`, but at a different point in time. While Bevy inserts your resources at the very beginning, the AssetLoader will do so after having inserted your loaded asset collections. That means that you can use your asset collections in the `FromWorld` implementations.

## Usage without a loading state

Although the pattern of a loading state is quite nice, you might have reasons not to use it. In this case `bevy_asset_loader` can still be helpful. Deriving `AssetCollection` on a resource can significantly reduce the boilerplate for managing assets.

You can directly initialise asset collections on the bevy App or World. See [no_loading_state.rs](/bevy_asset_loader/examples/no_loading_state.rs) for a complete example.

```rust no_run
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetCollectionApp};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_collection::<MyAssets>()
        .run();
}

#[derive(AssetCollection)]
struct MyAssets {
    #[asset(texture_atlas(tile_size_x = 100., tile_size_y = 96., columns = 8, rows = 1, padding_x = 12., padding_y = 12.))]
    #[asset(path = "images/sprite_sheet.png")]
    sprite: Handle<TextureAtlas>,
}
```

## Compatible Bevy versions

The main branch is compatible with the latest Bevy release, while the branch `bevy_main` tracks the `main` branch of Bevy.

Compatibility of `bevy_asset_loader` versions:
| `bevy_asset_loader` | `bevy` |
| :--               | :--    |
| `0.8`             | `0.6`  |
| `0.1` - `0.7`     | `0.5`  |
| `main`            | `0.6`  |
| `bevy_main`       | `main` |

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

Assets in the examples might be distributed under different terms. See the [readme](bevy_asset_loader/examples/README.md#credits) in the `bevy_asset_loader/examples` directory.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[bevy]: https://bevyengine.org/
[cheatbook-states]: https://bevy-cheatbook.github.io/programming/states.html
