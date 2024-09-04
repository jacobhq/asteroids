use macroquad::prelude::*;

struct Player {
    position: Vec2,
    rotation: f32,
    velocity: Vec2
}

struct Asteroid {
    position: Vec2,
    rotation: f32,
    rotation_speed: f32,
    velocity: Vec2
}

struct Bullet {
    position: Vec2,
    velocity: Vec2,
    shot_at: f64,
    collided: bool,
}

enum CollisionTypes {
    Player(Player),
    Asteroid(Asteroid),
    Bullet(Bullet),
    Edge
}

struct Collision {
    objects: Vec<CollisionTypes>
}

fn wrap_around(v: &Vec2) -> Vec2 {
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > screen_width() {
        vr.x = 0.;
    }
    if vr.x < 0. {
        vr.x = screen_width()
    }
    if vr.y > screen_height() {
        vr.y = 0.;
    }
    if vr.y < 0. {
        vr.y = screen_height()
    }
    vr
}

#[macroquad::main("asteroids")]
async fn main() {
    let mut player = Player {
        position: Vec2::new(screen_width() / 2., screen_height() / 2.),
        rotation: 0.,
        velocity: Vec2::new(0., 0.)
    };

    let mut bullets = Vec::new();
    let mut last_shot = get_time();
    let mut asteroids = Vec::new();
    let mut gameover = false;

    let mut screen_center;

    loop {
        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }

}
