use bevy::prelude::*;

const BALL_SIZE: f32 = 25.0;
const PADDLE_WIDTH: f32 = 30.0;
const PADDLE_HEIGHT: f32 = 200.0;
const PADDLE_PADDING: f32 = 50.0;
const PADDLE_MOVEMENT_AMOUNT: f32 = 350.0;
const VELOCITY: f32 = 300.0;

#[derive(Component, Debug)]
struct Player {
    id: u8,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_paddles)
        .add_systems(Update, confine_paddles)
        .add_systems(Update, move_ball)
        .run();
}

fn setup(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.get_single().unwrap();
    let width = window.width();

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(BALL_SIZE)),
                ..default()
            },
            ..default()
        },
        Velocity {
            x: VELOCITY,
            y: VELOCITY,
        },
    ));
    //paddles
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: PADDLE_WIDTH,
                    y: PADDLE_HEIGHT,
                }),
                ..default()
            },
            transform: Transform::from_xyz(-width / 2.0 + PADDLE_PADDING, 0.0, 0.0),
            ..default()
        },
        Player { id: 0 },
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: PADDLE_WIDTH,
                    y: PADDLE_HEIGHT,
                }),
                ..default()
            },
            transform: Transform::from_xyz(width / 2.0 - PADDLE_PADDING, 0.0, 0.0),
            ..default()
        },
        Player { id: 1 },
    ));
}

fn move_paddles(
    mut paddle_query: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in paddle_query.iter_mut() {
        let upwards = match player.id {
            0 => KeyCode::W,
            1 => KeyCode::Up,
            _ => break,
        };
        let downwards = match player.id {
            0 => KeyCode::S,
            1 => KeyCode::Down,
            _ => break,
        };

        if input.pressed(upwards) {
            transform.translation.y += PADDLE_MOVEMENT_AMOUNT * time.delta_seconds();
        }
        if input.pressed(downwards) {
            transform.translation.y -= PADDLE_MOVEMENT_AMOUNT * time.delta_seconds();
        }
    }
}

fn confine_paddles(
    mut paddle_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();
    let height = window.height();
    let half_paddle = PADDLE_HEIGHT / 2.0;

    for mut transform in paddle_query.iter_mut() {
        if transform.translation.y + half_paddle >= height / 2.0 {
            transform.translation.y = height / 2.0 - half_paddle;
        }
        if transform.translation.y - half_paddle <= -height / 2.0 {
            transform.translation.y = -height / 2.0 + half_paddle;
        }
    }
}

fn move_ball(
    mut ball_query: Query<(&mut Transform, &mut Velocity)>,
    paddle_query: Query<(&Transform, &Player), Without<Velocity>>,
    window_query: Query<&Window>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();
    let half_height = window.height() / 2.0;

    for (mut ball_transform, mut velocity) in ball_query.iter_mut() {
        ball_transform.translation +=
            Vec3::new(velocity.x, velocity.y, 0.0) * Vec3::splat(time.delta_seconds());

        if ball_transform.translation.y + BALL_SIZE / 2.0 >= half_height
            || ball_transform.translation.y - BALL_SIZE / 2.0 <= -half_height
        {
            velocity.y *= -1.0;
        }
        for (paddle_transform, player) in paddle_query.iter() {
            match player.id {
                0 => {
                    if (ball_transform.translation.x - BALL_SIZE <= paddle_transform.translation.x)
                        && (paddle_transform.translation.y.abs() - PADDLE_HEIGHT / 2.0
                            <= ball_transform.translation.y)
                    {
                        velocity.x *= -1.0;
                    }
                }
                1 => {
                    if (ball_transform.translation.x + BALL_SIZE >= paddle_transform.translation.x)
                        && (paddle_transform.translation.y.abs() - PADDLE_HEIGHT / 2.0
                            <= ball_transform.translation.y)
                    {
                        velocity.x *= -1.0;
                    }
                }
                _ => break,
            };
        }
    }
}
