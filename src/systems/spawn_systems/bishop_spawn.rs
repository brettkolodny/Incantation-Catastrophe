use amethyst::core::{
    nalgebra::{Unit, Vector3},
    Time, Transform,
};
use amethyst::ecs::{Entities, Read, System, WriteStorage};
use amethyst::renderer::SpriteRender;
use rand::Rng;

use crate::components::{Bishop, Enemy, Health, Size};
use crate::resources::SpriteSheet;
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

pub struct BishopSpawnSystem {
    pub spawn_timer: f32,
    pub time_since_spawn: f32,
}

impl<'s> System<'s> for BishopSpawnSystem {
    type SystemData = (
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Bishop>,
        WriteStorage<'s, Size>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Health>,
        Read<'s, SpriteSheet>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            mut enemies,
            mut bishops,
            mut sizes,
            mut sprite_renders,
            mut transforms,
            mut healths,
            spritesheet,
            time,
            entities,
        ): Self::SystemData,
    ) {
        if self.time_since_spawn >= self.spawn_timer {
            let radius = (GAMEPLAY_AREA_HEIGHT) / 2.;
            let angle = rand::thread_rng().gen_range(0, 360) as f32;

            let circle_vector = {
                let x = radius * angle.sin() + GAMEPLAY_AREA_WIDTH / 2.;
                let y = radius * angle.cos() + GAMEPLAY_AREA_HEIGHT / 2.;
                let z = 0.;

                let circle_vector = Vector3::new(x, y, z);
                circle_vector
            };

            let mut bishop_transform = Transform::default();
            bishop_transform.set_scale(2., 2., 1.);
            bishop_transform.move_global(circle_vector);

            let center_vector =
                Vector3::new(GAMEPLAY_AREA_WIDTH / 2., GAMEPLAY_AREA_HEIGHT / 2., 0.);

            let spawn_vector = center_vector - circle_vector;

            let distance = rand::thread_rng().gen_range(0, GAMEPLAY_AREA_HEIGHT as i32) as f32;

            bishop_transform.move_along_global(Unit::new_normalize(spawn_vector), distance);

            let sprite_render = {
                SpriteRender {
                    sprite_sheet: spritesheet.sprite_sheet.clone().unwrap(),
                    sprite_number: 1,
                }
            };

            entities
                .build_entity()
                .with(
                    Bishop {
                        time_since_move: 0.,
                        time_since_shot: 2.,
                        shot_cooldown: 4.,
                    },
                    &mut bishops,
                )
                .with(Enemy, &mut enemies)
                .with(Size::new(32., 32.), &mut sizes)
                .with(bishop_transform, &mut transforms)
                .with(sprite_render, &mut sprite_renders)
                .with(Health::bishop(), &mut healths)
                .build();

            self.time_since_spawn = 0.;
        } else {
            self.time_since_spawn += time.delta_seconds();
        }
    }
}