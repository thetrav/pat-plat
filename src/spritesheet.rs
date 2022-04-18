use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct SpriteSheetPlugin;

pub trait AtlasBox {
    fn atlas(&self) -> Handle<TextureAtlas>;
}

pub struct BackgroundAtlas(Handle<TextureAtlas>);
impl AtlasBox for BackgroundAtlas {
    fn atlas(&self) -> Handle<TextureAtlas> {
        self.0.clone()
    }
}

pub struct CharacterAtlas(Handle<TextureAtlas>);
impl AtlasBox for CharacterAtlas {
    fn atlas(&self) -> Handle<TextureAtlas> {
        self.0.clone()
    }
}

pub struct TileAtlas(Handle<TextureAtlas>);
impl AtlasBox for TileAtlas {
    fn atlas(&self) -> Handle<TextureAtlas> {
        self.0.clone()
    }
}

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app:&mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            load_sprite_sheets);
    }
}

pub(crate) fn spawn_sprite<T>(
    commands: &mut Commands,
    atlas_box: &T,
    index: usize,
    translation: Vec3
) -> Entity where T: AtlasBox {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    let texture_atlas = atlas_box.atlas();
    return commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas,
        sprite,
        transform: Transform{
            translation,
            ..Default::default()
        },
        ..Default::default()
    }).id();
}

fn load_sprite_sheets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>
) {
    let image:Handle<Image> = assets.load("tiles.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(18.),
        20,
        9,
        Vec2::splat(2.0)
    );
    let atlas_handle = texture_atlasses.add(atlas);
    commands.insert_resource(TileAtlas(atlas_handle));

    let image:Handle<Image> = assets.load("background.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(23.),
        6,
        2,
        Vec2::splat(1.)
    );
    let atlas_handle = texture_atlasses.add(atlas);
    commands.insert_resource(BackgroundAtlas(atlas_handle));

    let image:Handle<Image> = assets.load("characters.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(24.),
        9,
        3,
        Vec2::splat(1.)
    );
    let atlas_handle = texture_atlasses.add(atlas);
    commands.insert_resource(CharacterAtlas(atlas_handle));
}