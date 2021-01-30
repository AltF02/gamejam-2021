use bevy::prelude::*;

pub fn is_colliding_with_walls(window: &mut Window, transform: &mut Mut<Transform>) -> bool {
    let pos_x_abs = transform.translation.x.abs();
    let pos_y_bas = transform.translation.y.abs();

    let max_x = window.width() / 2.;
    let max_y = window.height() / 2.;

    if pos_x_abs >= max_x {
        if transform.translation.x > 0. {
            transform.translation.x -= 1.;
        } else {
            transform.translation.x += 1.;
        }

        return true;
    }

    if pos_y_bas >= max_y {
        if transform.translation.y > 0. {
            transform.translation.y -= 1.;
        } else {
            transform.translation.y += 1.;
        }

        return true;
    }

    false
}
