use crate::models::player::Player;
use bevy::prelude::*;

pub fn is_colliding_with_walls(mut window: &mut Window, mut transform: &Mut<Transform>) -> bool {
    let pos_x_abs = transform.translation.x.abs();
    let pos_y_bas = transform.translation.y.abs();

    let max_x = window.width() / 2.;
    let max_y = window.height() / 2.;

    if pos_x_abs >= max_x {
        return true;
    }

    if pos_y_bas >= max_y {
        return true;
    }

    false
}
