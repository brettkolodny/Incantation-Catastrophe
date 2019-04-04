use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

use crate::components::tags::GameplayItem;

use crate::components::{CurrentDirection, Speed};

#[derive(Default)]
pub struct Player {
  pub width: f32,
  pub height: f32,
  pub time_since_shot: f32,
  pub cooldown: f32,
}

impl Player {
  pub fn new() -> Player {
    Player {
      width: 1.,
      height: 200.,
      time_since_shot: 3.,
      cooldown: 0.25,
    }
  }

  pub fn initialize(_world: &mut World, _sprite_sheet_handle: SpriteSheetHandle) {
    let mut local_transform = Transform::default();
    local_transform.set_xyz(GAMEPLAY_AREA_WIDTH / 2., GAMEPLAY_AREA_HEIGHT / 2., 0.);
    local_transform.set_scale(1., 1., 1.);

    let sprite_render = {
      SpriteRender {
        sprite_sheet: _sprite_sheet_handle,
        sprite_number: 2,
      }
    };

    _world
      .create_entity()
      .with(sprite_render)
      .with(local_transform)
      .with(Player::new())
      .with(Speed::new(5.))
      .with(CurrentDirection::default())
      .with(GameplayItem)
      .build();
  }
}

impl Component for Player {
  type Storage = DenseVecStorage<Self>;
}
