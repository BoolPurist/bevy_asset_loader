#![allow(dead_code)]

use bevy::app::AppExit;
use bevy::asset::AssetPlugin;
use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use iyes_loopless::prelude::*;

#[cfg_attr(
    all(
        not(feature = "2d"),
        not(feature = "3d"),
        not(feature = "progress_tracking"),
        feature = "stageless"
    ),
    test
)]
fn stageless() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin::default())
        .add_plugin(AudioPlugin::default())
        .add_loopless_state(MyStates::Load);
    AssetLoader::new(MyStates::Load)
        .continue_to_state(MyStates::Next)
        .with_collection::<MyAssets>()
        .init_resource::<PostProcessed>()
        .build(&mut app);

    app.add_system(timeout.run_in_state(MyStates::Load))
        .add_enter_system(MyStates::Next, expect)
        .run();
}

fn timeout(time: Res<Time>) {
    if time.seconds_since_startup() > 10. {
        panic!("The asset loader did not change the state in 10 seconds");
    }
}

fn expect(collection: Option<Res<PostProcessed>>, mut exit: EventWriter<AppExit>) {
    if collection.is_none() {
        panic!("Post processed collection was not inserted");
    } else {
        exit.send(AppExit);
    }
}

#[allow(dead_code)]
#[derive(AssetCollection)]
struct MyAssets {
    #[asset(path = "audio/background.ogg")]
    background: Handle<AudioSource>,
}

#[allow(dead_code)]
// this struct could e.g. contain TextureAtlas handles or anything else
// created from previously loaded assets
struct PostProcessed {
    background: Handle<AudioSource>,
    // use other resources/add fields
    fuu: String,
}

impl FromWorld for PostProcessed {
    fn from_world(world: &mut World) -> Self {
        let assets = world
            .get_resource::<MyAssets>()
            .expect("MyAssets not loaded");
        PostProcessed {
            background: assets.background.clone(),
            fuu: "bar".to_owned(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MyStates {
    Load,
    Next,
}
