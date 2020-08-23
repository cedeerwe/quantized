use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());
}

struct SpawnTimer(Timer);

struct Ball {
    pub velocity: Vec3,
    pub sine_bias_term: f32,
}

fn spawn_dots(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    window: Res<WindowDescriptor>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(1., 1., 1.).into()),
                translation: Translation(Vec3::new(
                    rand::thread_rng()
                        .gen_range(-(window.width as f32) * 0.5, (window.width as f32) * 0.5)
                        as f32,
                    (window.height as f32) * 0.5,
                    1.0,
                )),
                sprite: Sprite {
                    size: Vec2::new(3.0, 3.0),
                },
                ..Default::default()
            })
            .with(Ball {
                velocity: 60.0 * Vec3::new(0.0, -1.0, 0.0).normalize(),
                sine_bias_term: rand::thread_rng().gen_range(0., PI),
            });
        timer.0.reset();
    }
}

fn move_dots(time: Res<Time>, mut ball_query: Query<(&mut Ball, &mut Translation)>) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds);

    for (mut ball, mut translation) in &mut ball_query.iter() {
        translation.0 += ball.velocity * delta_seconds;
        let bias = ball.sine_bias_term;
        ball.velocity += Vec3::new(
            0.2 * (time.seconds_since_startup as f32 + bias).cos(),
            0.,
            0.,
        )
    }
}

fn remove_dots(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    mut ball_query: Query<(Entity, &Translation, &Ball)>,
) {
    for (entity, translation, _) in &mut ball_query.iter() {
        if translation.0.y() < -(window.height as f32 * 0.5) {
            commands.despawn(entity);
        }
    }
}

fn main() {
    App::build()
        .add_resource(SpawnTimer(Timer::from_seconds(0.1)))
        .add_resource(WindowDescriptor {
            title: "TD".to_string(),
            width: 1000,
            height: 600,
            vsync: true,
            ..Default::default()
        })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(spawn_dots.system())
        .add_system(move_dots.system())
        .add_system(remove_dots.system())
        .run();
}
