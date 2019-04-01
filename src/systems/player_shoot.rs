use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Entities, Join, System};
use amethyst::ecs::{Read, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;

use crate::components::{CurrentDirection, Player, PlayerProjectile, Speed};
use crate::states::SpriteSheet;

pub struct PlayerShootSystem;

impl<'s> System<'s> for PlayerShootSystem {
  type SystemData = (
    WriteStorage<'s, PlayerProjectile>,
    WriteStorage<'s, Player>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, Speed>,
    WriteStorage<'s, CurrentDirection>,
    WriteStorage<'s, SpriteRender>,
    Read<'s, SpriteSheet>,
    Entities<'s>,
    Read<'s, InputHandler<String, String>>,
    Read<'s, Time>,
  );

  fn run(
    &mut self,
    (
      mut player_projectiles,
      mut players,
      mut transforms,
      mut speeds,
      mut directions,
      mut sprite_renders,
      spritesheet,
      entities,
      input,
      time,
    ): Self::SystemData,
  ) {
    if let Some(true) = input.action_is_down("shoot") {
      let mut player_transforms_directions = Vec::<(Transform, CurrentDirection)>::new();

      for (mut player, transform, direction) in (&mut players, &transforms, &directions).join() {
        if player.time_since_shot >= 0.5 {
          player_transforms_directions.push((transform.clone(), direction.clone()));
          player.time_since_shot = 0.;
        } else {
          player.time_since_shot += time.delta_seconds();
          println!("Time since last shot: {}", player.time_since_shot);
        }
      }

      let sprite_render = {
        SpriteRender {
          sprite_sheet: spritesheet.sprite_sheet.clone().unwrap(),
          sprite_number: 1,
        }
      };

      for (transform, direction) in player_transforms_directions {
        entities
          .build_entity()
          .with(PlayerProjectile::new(), &mut player_projectiles)
          .with(transform, &mut transforms)
          .with(direction, &mut directions)
          .with(sprite_render.clone(), &mut sprite_renders)
          .with(Speed::default(), &mut speeds)
          .build();
      }
    }
  }
}