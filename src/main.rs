use bevy::prelude::*;
use bevy::tasks::{Scope, TaskPool};

use itertools::Itertools;

#[derive(Component)]
pub struct X(u8);

#[derive(Component)]
pub struct Y(u8);

fn main() {
        App::new()
                .insert_resource(TaskPool::new())
                .add_plugins(DefaultPlugins)
                .run();
}

fn system(mut query: Query<(&mut X, &mut Y)>, tp: Res<TaskPool>) {
        let chunks = query.iter_mut().chunks(u16::MAX as usize);

        let (x_sum, y_sum) = tp.scope(|scope: &mut Scope<(u64, u64)>| {
                for c in chunks.into_iter() {
                        tp.spawn_local(async {
                                let (mut x_sum, mut y_sum) = (0u64,0u64);

                                for (x, y) in c.into_iter() {
                                        x_sum += x.0 as u64;
                                        y_sum += y.0 as u64;
                                }

                                (x_sum, y_sum)
                        });
                }
        }).into_iter().fold((0,0), |(x_sum, y_sum), (x, y)| (x_sum + x, y_sum + y));
}
