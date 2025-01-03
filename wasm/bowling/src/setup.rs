//! Scene setup methods

use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    Ccd, Collider, ColliderMassProperties, Friction, GravityScale, Restitution, RigidBody, Velocity,
};

pub mod ball;
pub mod pin;

pub use ball::Ball;
pub use pin::Pin;

/// Lane length
const LANE_LENGTH: f32 = 30.0;
/// Lane width
pub const LANE_WIDTH: f32 = 3.0;

/// Number of pins in a standard arrangement
const PIN_COUNT: usize = 10;

/// Distance from origin to the first pin
pub const PIN_START_Z: f32 = 10.0;

/// Where the ball starts
pub const BALL_START_Z: f32 = -5.0;

/// How fast the ball moves once “released”
pub const BALL_SPEED: f32 = 10.0;

/// Pin radius
const PIN_RADIUS: f32 = 0.15;
/// Pin height
const PIN_HEIGHT: f32 = 0.65;

/// Scorecard identifying Component
#[derive(Component)]
pub struct Scorecard;

/// Scorecard bg identifying Component
#[derive(Component)]
pub struct ScorecardBg;

/// Component for anything we want hidden when the Game is over
#[derive(Component)]
pub struct Hideable;

/// Component for final score label
#[derive(Component)]
pub struct FinalScore;

/// Spawns the lane, the ball, and pins
pub fn setup(
    mut commands: Commands<'_, '_>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    // Spawn Lane
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(LANE_WIDTH, 0.1, LANE_LENGTH))),
        MeshMaterial3d(materials.add(Color::hsl(33.0, 0.20, 0.76))),
        Transform::from_xyz(0.0, -0.05, LANE_LENGTH * 0.5 - 10.0),
        Name::new("Lane"),
        Collider::cuboid(LANE_WIDTH * 0.5, 0.05, LANE_LENGTH * 0.5),
        Restitution::coefficient(0.01),
        RigidBody::Fixed,
        Friction::coefficient(0.04),
        Visibility::Visible,
        Hideable,
    ));

    // Spawn pins
    let rows = how_many_rows(PIN_COUNT);
    for row in 1..=rows {
        let z_pos = PIN_START_Z + (row as f32);

        let start_pos = 0.0 - ((row - 1) as f32 / 2.0) * PIN_RADIUS * 4.0;

        for pin in 0..row {
            let x_pos = start_pos + ((pin as f32) * PIN_RADIUS * 4.0);

            let point = Transform::from_xyz(x_pos, PIN_HEIGHT * 0.5 + 0.05, z_pos);

            commands.spawn((
                Mesh3d(meshes.add(Cylinder::new(PIN_RADIUS, PIN_HEIGHT))),
                MeshMaterial3d(materials.add(Color::hsl(33.0, 0.0, 0.93))),
                point.clone(),
                Pin::new(point),
                Name::new(format!("Pin {pin} in Row {row}")),
                Collider::cylinder(PIN_HEIGHT * 0.5, PIN_RADIUS),
                RigidBody::Dynamic,
                Restitution::coefficient(0.8),
                Friction::coefficient(0.6),
                GravityScale(0.9),
                ColliderMassProperties::Density(0.8),
                Velocity::linear(Vec3::ZERO),
                Ccd::enabled(),
                Visibility::Visible,
                Hideable,
            ));
        }
    }

    // Spawn Ball
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(Color::hsl(33.0, 0.90, 0.61))),
        Transform::from_xyz(0.0, 0.3, BALL_START_Z).looking_at(Vec3::ZERO, Vec3::Y),
        Ball::default(),
        Name::new("Ball"),
        RigidBody::KinematicPositionBased,
        Collider::ball(0.3),
        Restitution::coefficient(0.4),
        GravityScale(1.0),
        Friction::coefficient(0.6),
        Velocity::linear(Vec3::ZERO),
        ColliderMassProperties::Density(1.2),
        Ccd::enabled(),
        Visibility::Visible,
        Hideable,
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spawn UI Camera
    commands.spawn(Camera2d::default());
    commands.spawn((Text::new(":D"), TextColor::WHITE, Scorecard));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::hsl(44.0, 0.23, 0.42)),
            Visibility::Hidden,
            ScorecardBg,
        ))
        .with_children(|parent| {
            parent.spawn((Text::new(""), Visibility::Inherited, FinalScore));
        });

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 3.0, -13.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Calculates how many rows a bowling lane should have
pub fn how_many_rows(pins: usize) -> usize {
    let mut count = 0;
    let mut pins = pins;
    while pins > 0 {
        count += 1;
        if count > pins {
            pins = 0
        } else {
            pins -= count
        }
    }
    count
}
