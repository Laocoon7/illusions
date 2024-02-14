use bevy::prelude::*;
use rand::{prelude::Rng, SeedableRng};
use rand_pcg::Pcg64;

use crate::lines::{data::Line, resources::LineState};

pub fn draw_line(
    mut a_images: ResMut<Assets<Image>>,
    time: Res<Time>,
    mut line_state: ResMut<LineState>,
    q_image: Query<&Handle<Image>>,
) {
    line_state.timer.tick(time.delta());
    if !line_state.timer.just_finished() {
        return;
    }

    let [r_a, g_a, b_a, _a_a] = line_state.color_a.as_rgba_u8();
    let [r_b, g_b, b_b, _a_b] = line_state.color_b.as_rgba_u8();

    let mut rng = Pcg64::from_entropy();

    for id in q_image.iter() {
        if let Some(image) = a_images.get_mut(id) {
            let start_x = rng.gen_range(0..image.width() as i32);
            let start_y = rng.gen_range(0..image.height() as i32);
            let end_x = rng.gen_range(0..image.width() as i32);
            let end_y = rng.gen_range(0..image.height() as i32);

            let line = Line::new(IVec2::new(start_x, start_y), IVec2::new(end_x, end_y));

            for value in line {
                let index = (value.y as usize * image.width() as usize + value.x as usize) * 4;
                if line_state.random_colors {
                    image.data[index] = rng.gen_range(0..255);
                    image.data[index + 1] = rng.gen_range(0..255);
                    image.data[index + 2] = rng.gen_range(0..255);
                } else {
                    image.data[index] = if image.data[index] == r_a { r_b } else { r_a };
                    image.data[index + 1] = if image.data[index + 1] == g_a { g_b } else { g_a };
                    image.data[index + 2] = if image.data[index + 2] == b_a { b_b } else { b_a };
                }
            }
        }
    }
}
