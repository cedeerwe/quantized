use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());
}

#[derive(Debug, Clone)]
struct FollowPath {
    path: Vec<Vec2>,
    current_origin: usize,
    total_part: f32,
    current_part: f32,
    cumulative_relative_lengths: Vec<f32>,
}

impl FollowPath {
    fn new(path: Vec<Vec2>) -> Self {
        let mut lengths = vec![];
        let mut cumulative_lengths = vec![0.];
        for (i, point) in path.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let length = (*point - path[i - 1]).length();
            lengths.push(length);
            cumulative_lengths.push(length + cumulative_lengths[i - 1]);
        }
        let total_length = cumulative_lengths.last().unwrap();
        Self {
            path,
            current_origin: 0,
            total_part: 0.,
            current_part: 0.,
            cumulative_relative_lengths: cumulative_lengths
                .iter()
                .map(|v| v / total_length)
                .collect(),
        }
    }

    fn position(&self) -> Vec2 {
        if self.total_part <= 0.0 {
            *self.path.first().unwrap()
        } else if self.total_part >= 1.0 {
            *self.path.last().unwrap()
        } else {
            self.path[self.current_origin]
                .lerp(self.path[self.current_origin + 1], self.current_part)
        }
    }

    fn move_part(&mut self, s: f32) {
        if self.path.len() < 2 {
            return;
        }
        self.total_part = (s + self.total_part).min(1.).max(0.);
        let mut current_origin = 0;
        for (i, val) in self.cumulative_relative_lengths.iter().enumerate().rev() {
            if *val <= self.total_part {
                current_origin = i;
                break;
            }
        }
        self.current_origin = current_origin;
        if self.current_origin == self.path.len() - 1 {
            return;
        }
        self.current_part = (self.total_part
            - self.cumulative_relative_lengths[self.current_origin])
            / (self.cumulative_relative_lengths[self.current_origin + 1]
                - self.cumulative_relative_lengths[self.current_origin]);
    }
}

struct EnemySpawner {
    follow_path: FollowPath,
    timer: Timer,
}

fn spawn_dots(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut spawner: ResMut<EnemySpawner>,
) {
    spawner.timer.tick(time.delta_seconds);

    if spawner.timer.finished {
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(1., 0.1, 0.1).into()),
                translation: Translation(spawner.follow_path.position().extend(1.0)),
                sprite: Sprite {
                    size: Vec2::new(3.0, 3.0),
                },
                ..Default::default()
            })
            .with(spawner.follow_path.clone());
        spawner.timer.reset();
    }
}

fn move_dots(time: Res<Time>, mut follow_path_query: Query<(&mut FollowPath, &mut Translation)>) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds);

    for (mut follow_path, mut translation) in &mut follow_path_query.iter() {
        follow_path.move_part(delta_seconds / 30.);
        translation.0 = follow_path.position().extend(1.0);
    }
}

fn remove_dots(mut commands: Commands, mut ball_query: Query<(Entity, &FollowPath)>) {
    for (entity, follow_path) in &mut ball_query.iter() {
        if follow_path.total_part >= 1. {
            commands.despawn(entity);
        }
    }
}

fn main() {
    let path = vec![
        Vec2::new(-200., -200.),
        Vec2::new(-200., -100.),
        Vec2::new(200., -100.),
        Vec2::new(200., 100.),
        Vec2::new(0., 200.),
        Vec2::new(-200., 100.),
        Vec2::new(0., 0.),
    ];
    let follow_path = FollowPath::new(path);
    App::build()
        .add_resource(WindowDescriptor {
            title: "TD".to_string(),
            width: 1000,
            height: 600,
            vsync: true,
            ..Default::default()
        })
        .add_resource(EnemySpawner {
            follow_path,
            timer: Timer::from_seconds(0.5),
        })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(spawn_dots.system())
        .add_system(move_dots.system())
        .add_system(remove_dots.system())
        .run();
}
