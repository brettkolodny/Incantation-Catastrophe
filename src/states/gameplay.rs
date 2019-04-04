use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection, SpriteRender, SpriteSheetHandle};

use crate::components::{Background, GameplayItem, Player};
use crate::resources::SpriteSheet;
use crate::utility::{load_sprite_sheet, GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

pub struct GameplayState;

impl SimpleState for GameplayState {
  fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    let world = _data.world;

    let spritesheet_handle = Some(load_sprite_sheet(
      world,
      "textures/incantation_catastrophe.png",
      "textures/incantation_catastrophe.ron",
    ));

    world.write_resource::<SpriteSheet>().sprite_sheet = Some(spritesheet_handle.unwrap());

    let spritesheet = world
      .read_resource::<SpriteSheet>()
      .sprite_sheet
      .clone()
      .unwrap();

    initialize_arena(world, spritesheet.clone());
    Player::initialize(world, spritesheet.clone());
    initialize_camera(world);
  }

  fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    Trans::None
  }
}

fn initialize_camera(_world: &mut World) {
  let mut transform = Transform::default();
  transform.set_z(1.0);
  _world
    .create_entity()
    .with(Camera::from(Projection::orthographic(
      0.0,
      GAMEPLAY_AREA_WIDTH,
      0.0,
      GAMEPLAY_AREA_HEIGHT,
    )))
    .with(transform)
    .build();
}

pub fn initialize_arena(_world: &mut World, _sprite_sheet_handle: SpriteSheetHandle) {
  let mut local_transform = Transform::default();
  local_transform.set_xyz(GAMEPLAY_AREA_WIDTH / 2., GAMEPLAY_AREA_HEIGHT / 2., 0.);
  local_transform.set_scale(10., 10., 1.);

  let sprite_render = {
    SpriteRender {
      sprite_sheet: _sprite_sheet_handle,
      sprite_number: 0,
    }
  };

  _world
    .create_entity()
    .with(sprite_render)
    .with(local_transform)
    .with(GameplayItem)
    .with(Background)
    .build();
}
