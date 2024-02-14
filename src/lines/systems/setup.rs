use bevy::{
    prelude::*,
    render::{
        camera::ScalingMode,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    sprite::Anchor,
};

use crate::lines::resources::LineState;

const IMAGE_WIDTH: u32 = 100;
const IMAGE_HEIGHT: u32 = 100;

pub fn setup(mut commands: Commands, mut a_images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 1.0,
                height: 1.0,
            },
            ..Default::default()
        },
        transform: Transform::from_xyz(0.5, 0.5, 256.0),
        ..Default::default()
    });

    let line_state = LineState::default();

    let color = line_state.color_a;
    let pixel = color.as_rgba_u8();
    let image = Image::new_fill(
        Extent3d {
            width: IMAGE_WIDTH,
            height: IMAGE_HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &pixel,
        TextureFormat::Rgba8UnormSrgb,
    );

    let texture = a_images.add(image);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::ONE),
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        // transform: Transform::from_translation(Vec2::ZERO.extend(1.0)),
        texture,
        ..Default::default()
    });

    commands.insert_resource(line_state);
}
