#![allow(dead_code, unused_imports)]

use bevy::app::AppExit;
use bevy::asset::AssetPlugin;
use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

#[cfg(all(
    not(feature = "2d"),
    not(feature = "3d"),
    not(feature = "progress_tracking")
))]
#[test]
fn multiple_asset_collections() {
    App::new()
        .add_state::<MyStates>()
        .add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            AudioPlugin::default(),
        ))
        .add_loading_state(LoadingState::new(MyStates::Load).continue_to_state(MyStates::Next))
        .add_collection_to_loading_state::<_, PlopAudio>(MyStates::Load)
        .add_collection_to_loading_state::<_, BackgroundAudio>(MyStates::Load)
        .add_systems(Update, timeout.run_if(in_state(MyStates::Load)))
        .add_systems(OnEnter(MyStates::Next), expect)
        .run();
}

fn timeout(time: Res<Time>) {
    if time.elapsed_seconds_f64() > 60. {
        panic!("The asset loader did not change the state in 60 seconds");
    }
}

fn expect(
    collection: Option<Res<PlopAudio>>,
    other_collection: Option<Res<BackgroundAudio>>,
    mut exit: EventWriter<AppExit>,
) {
    if collection.is_none() || other_collection.is_none() {
        panic!("At least one asset collection was not inserted");
    } else {
        exit.send(AppExit);
    }
}

#[derive(AssetCollection, Resource)]
struct PlopAudio {
    #[asset(path = "audio/plop.ogg")]
    plop: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
struct BackgroundAudio {
    #[asset(path = "audio/background.ogg")]
    background: Handle<AudioSource>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    Load,
    Next,
}
