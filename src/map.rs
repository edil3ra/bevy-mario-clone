use bevy::{prelude::*};

use crate::{Game, level, config};

#[derive(Component)]
struct Index(usize, usize);


#[derive(Debug, Default, Resource)]
pub struct Map {
    pub tiles: Vec<Entity>,
    pub total_row: usize,
    pub total_column: usize,
    pub width: usize,
    pub height: usize,
}

impl Map {
    fn get_index_from_x_y(&self, x: f32, y: f32) -> &Entity {
        let row = (y / config::TILE_SIZE) as usize * self.total_column ;
        let col = (x / config::TILE_SIZE) as usize;
        let index = row + col;
        &self.tiles[index]
    }
}

pub struct MapPlugin {}
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::default())
            .add_startup_system_set_to_stage(
            StartupStage::Startup,
            SystemSet::new()
                .with_system(build_map)
        );
    }
}



fn build_map(mut commands: Commands, game_resource: Res<Game>, mut map_resource: ResMut<Map>) {
    let current_level = level::LevelFile::new(config::LEVELS[game_resource.level.current]);
    map_resource.total_row = current_level.dims.1;
    map_resource.total_column = current_level.dims.0;
    map_resource.height = map_resource.total_row * config::TILE_SIZE as usize;
    map_resource.width = map_resource.total_column * config::TILE_SIZE as usize;

    commands
        .spawn((
            TransformBundle::default(),
            VisibilityBundle::default(),
            Name::new("map"),
        ))
        .add_children(|parent| {
            for y in 0..current_level.dims.1 {
                for x in 0..current_level.dims.0 {
                    let pos = (x, y);
                    let tile = current_level.get(pos);
                    let index_map = game_resource.map_to_index.get(&tile).unwrap();
                    let entity = parent
                        .spawn(SpriteSheetBundle {
                            texture_atlas: game_resource.assets.tiles.clone(),
                            transform: Transform::from_xyz((x * 16) as f32, (y * 16) as f32, 0.0),
                            sprite: TextureAtlasSprite::new(*index_map),
                            ..default()
                        })
                        .insert(Name::new(tile.to_string()))
                        .insert(Index(pos.0, pos.1))
                        .id();
                    map_resource.tiles.push(entity);
                }
            }
        });
    dbg!(&map_resource);
}


