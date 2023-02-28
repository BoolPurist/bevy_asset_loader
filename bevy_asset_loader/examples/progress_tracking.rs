use bevy::app::AppExit;
use bevy::asset::LoadState;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::{ProgressCounter, ProgressPlugin, TrackedProgressSet};

/// This example shows how to track the loading progress of your collections using `iyes_progress`
///
/// Running it will print the current progress for every frame. The five assets from
/// the two collections will be loaded rather quickly (one/a few frames). The final task
/// completes after two seconds. At that point, `iyes_progress` will continue to the next state
/// and the app will terminate.
fn main() {
    App::new()
        .add_state::<MyStates>()
        .add_loading_state(LoadingState::new(MyStates::AssetLoading))
        .add_collection_to_loading_state::<_, TextureAssets>(MyStates::AssetLoading)
        .add_collection_to_loading_state::<_, AudioAssets>(MyStates::AssetLoading)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // track progress during `MyStates::AssetLoading` and continue to `MyStates::Next` when progress is completed
        .add_plugin(ProgressPlugin::new(MyStates::AssetLoading).continue_to(MyStates::Next))
        // gracefully quit the app when `MyStates::Next` is reached
        .add_system(expect.in_schedule(OnEnter(MyStates::Next)))
        .add_system(
            track_fake_long_task
                .before(print_progress)
                .in_set(TrackedProgressSet)
                .run_if(in_state(MyStates::AssetLoading)),
        )
        .add_system(print_progress)
        .run();
}

// Time in seconds to complete a custom long-running task.
// If assets are loaded earlier, the current state will not
// be changed until the 'fake long task' is completed (thanks to 'iyes_progress')
const DURATION_LONG_TASK_IN_SECS: f64 = 2.0;

#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(path = "audio/background.ogg")]
    background: Handle<AudioSource>,
    #[asset(path = "audio/plop.ogg")]
    plop: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
struct TextureAssets {
    #[asset(path = "images/player.png")]
    player: Handle<Image>,
    #[asset(path = "images/tree.png")]
    tree: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 96., tile_size_y = 99., columns = 8, rows = 1))]
    #[asset(path = "images/female_adventurer_sheet.png")]
    female_adventurer: Handle<TextureAtlas>,
}

fn track_fake_long_task(time: Res<Time>, progress: Res<ProgressCounter>) {
    if time.elapsed_seconds_f64() > DURATION_LONG_TASK_IN_SECS {
        info!("Long task is completed");
        progress.manually_track(true.into());
    } else {
        progress.manually_track(false.into());
    }
}

fn expect(
    audio_assets: Res<AudioAssets>,
    texture_assets: Res<TextureAssets>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut quit: EventWriter<AppExit>,
) {
    assert_eq!(
        asset_server.get_load_state(audio_assets.background.clone()),
        LoadState::Loaded
    );
    assert_eq!(
        asset_server.get_load_state(audio_assets.plop.clone()),
        LoadState::Loaded
    );
    let atlas = texture_atlases
        .get(&texture_assets.female_adventurer)
        .expect("Texture atlas should be added to its assets resource.");
    assert_eq!(
        asset_server.get_load_state(atlas.texture.clone()),
        LoadState::Loaded
    );
    assert_eq!(
        asset_server.get_load_state(texture_assets.player.clone()),
        LoadState::Loaded
    );
    assert_eq!(
        asset_server.get_load_state(texture_assets.tree.clone()),
        LoadState::Loaded
    );
    info!("Everything looks good!");
    info!("Quitting the application...");
    quit.send(AppExit);
}

fn print_progress(
    progress: Option<Res<ProgressCounter>>,
    diagnostics: Res<Diagnostics>,
    mut last_done: Local<u32>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            *last_done = progress.done;
            info!(
                "[Frame {}] Changed progress: {:?}",
                diagnostics
                    .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                    .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                    .unwrap_or(0.),
                progress
            );
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}
