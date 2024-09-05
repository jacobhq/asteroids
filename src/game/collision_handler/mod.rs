use crate::game::asteroid::Asteroid;
use crate::game::bullet::Bullet;
use crate::game::player::Player;

enum CollisionType {
    PlayerAsteroid(Player, Asteroid),
    BulletAsteroid(Bullet, Asteroid)
}

struct Collision {
    collision_type: CollisionType,
    fatal: bool
}

impl Collision {
    pub fn collide(collision_type: CollisionType) {
        match collision_type { 
            CollisionType::PlayerAsteroid(player, asteroid) => { 
                player.die()
            },
            CollisionType::BulletAsteroid(bullet, asteroid) => { unimplemented!() }
        }
    }
}