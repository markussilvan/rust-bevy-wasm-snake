use bevy::prelude::*;

use crate::common::ScreenPosition;

#[derive(Component, Default)]
pub(crate) struct Bomb {
    pub timer: Timer,
}

impl Bomb {
    pub fn default() -> Bomb {
        let seconds = 5.0;
        Bomb {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct Particle {
    position: ScreenPosition,
    velocity: Vec2,
    //TODO: direction: AngleDirection,
    size: f32,
    color: Color,
}

impl Particle {
    pub fn new(position: ScreenPosition, velocity: Vec2, size: f32, color: Color) -> Self {
        Particle {
            position,
            velocity,
            size,
            color,
        }
    }
}

#[derive(Resource)]
pub(crate) struct ParticleSystem {
    particles: Vec<Entity>
}

impl ParticleSystem {
    pub fn new() -> Self {
        ParticleSystem {
            particles: Vec::new(),
        }
    }

    pub fn create_explosion(&mut self, commands: &mut Commands, position: ScreenPosition) {
        //TODO: create a nice particle pattern (that looks more "round")
        let velocities = [
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 5.0),
            Vec2::new(0.0, -5.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(5.0, -5.0),
            Vec2::new(-5.0, 0.0),
            Vec2::new(-5.0, 5.0),
            Vec2::new(-5.0, -5.0),
        ];
        let mut p = position.clone();
        p.z = 400.0; //TODO
        let size = 40.0;
        for vel in velocities {
            self.spawn_particle(commands, p, size, vel);
        }
    }

    pub fn update(&mut self,
                  mut commands: Commands,
                  mut query: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>) {
        for (entity, mut particle, mut transform, mut sprite) in query.iter_mut() {
            if transform.scale.x < 1.0 {
                commands.entity(entity).despawn();
                let index = self.particles.iter().position(|x| *x == entity).unwrap();
                self.particles.remove(index);
                continue;
            }

            particle.size -= 5.0;
            transform.scale = Vec3::new(particle.size, particle.size, 1.0);
            transform.translation.x += particle.velocity.x;
            transform.translation.y += particle.velocity.y;

            let a = sprite.color.a();
            sprite.color.set_a(0.9 * a);
            let b = sprite.color.b();
            sprite.color.set_b(1.2 * b);
        }
    }

    pub fn despawn_particles(&mut self, mut commands: Commands) {
        for entity in self.particles.iter() {
            commands.entity(*entity).despawn();
        }
        self.particles.clear();
    }

    fn spawn_particle(
        &mut self,
        commands: &mut Commands,
        position: ScreenPosition,
        size: f32,
        velocity: Vec2,
    ) {
        debug!("Spawning new particle to position {}", position);
        let particle = Particle::new(position, velocity, size, Color::GOLD);
        let entity = commands.spawn(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GOLD,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(size, size, 1.0),
                    translation: Vec3::new(position.x, position.y, position.z),
                    ..default()
                },
                ..default()
            })
            .insert(particle)
            .id();
        self.particles.push(entity);
    }
}
