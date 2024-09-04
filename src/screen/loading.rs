use bevy::prelude::*;

use super::Screen;
use crate::{
    game::assets::{HandleMap, LevelKey, PatternKey, TextureKey},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
    app.add_systems(
        Update,
        continue_to_title.run_if(in_state(Screen::Loading).and_then(all_assets_loaded)),
    );
}

fn enter_loading(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });
}

fn all_assets_loaded(
    asset_server: Res<AssetServer>,
    texture_handles: Res<HandleMap<TextureKey>>,
    level_handles: Res<HandleMap<LevelKey>>,
    // sprite_handles: Res<HandleMap<SpriteKey>>,
    pattern_handles: Res<HandleMap<PatternKey>>,
) -> bool {
    texture_handles.all_loaded(&asset_server)
        && level_handles.all_loaded(&asset_server)
        // && sprite_handles.all_loaded(&asset_server)
        && pattern_handles.all_loaded(&asset_server)
}

fn continue_to_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Playing);
}
