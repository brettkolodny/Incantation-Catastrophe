use amethyst::core::{nalgebra, timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{CurrentDirection, Rook, Speed};
use crate::resources::PlayerResource;

pub struct RookMoveSystem;

impl<'s> System<'s> for RookMoveSystem {
    type SystemData = (
        Read<'s, PlayerResource>,
        ReadStorage<'s, Rook>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, CurrentDirection>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (player, rooks, speeds, mut directions, mut transforms, time): Self::SystemData,
    ) {
        if let Some(player) = player.player {
            let player_transform = transforms.get(player).unwrap().clone();
            for (rook_transform, rook_speed, direction, _) in
                (&mut transforms, &speeds, &mut directions, &rooks).join()
            {
                let player_vector = player_transform.translation();
                let rook_vector = rook_transform.translation();

                let new_vector = player_vector - rook_vector;
                let new_vector = nalgebra::base::Matrix::normalize(&new_vector);
                let new_vector = nalgebra::Unit::new_unchecked(new_vector);

                if new_vector.x.abs() > new_vector.y.abs() {
                    if new_vector.x < 0. {
                        direction.turn_right();
                        rook_transform.set_rotation_euler(0., 0., 4.71239);
                    } else {
                        direction.turn_left();
                        rook_transform.set_rotation_euler(0., 0., 1.5708);
                    }
                } else {
                    if new_vector.y < 0. {
                        direction.turn_up();
                        rook_transform.set_rotation_euler(0., 0., 0.);
                    } else {
                        direction.turn_down();
                        rook_transform.set_rotation_euler(0., 0., 3.14159);
                    }
                }

                rook_transform
                    .move_along_global(new_vector, time.delta_seconds() * rook_speed.speed);
            }
        }
    }
}