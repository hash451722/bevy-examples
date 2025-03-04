use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(50.))),
        MeshMaterial2d(materials.add(Color::WHITE)),
    ));
}

const PLAYER_SPEED: f32 = 300.;
fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player.translation += move_delta.extend(0.);  // Vec2 -> Vec3
}