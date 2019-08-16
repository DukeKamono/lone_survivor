use crate::player::Bullet;
use ggez::*;
use ggez::graphics::Mesh;
use rand::prelude::*;

pub struct Enemy {
    hp: u32,
    x: f32,
    y: f32,
    pub hitbox: graphics::Rect,
}

impl Enemy {
    pub fn new() -> Enemy {
        let mut rng = thread_rng();
        let side = rng.gen_range(0, 4) as u32;
        let mut xpos = 0.0;
        let mut ypos = 0.0;

        match side {
            s if s == 0 => { ypos = 0.0; xpos = rng.gen_range(0, 800) as f32; },
            s if s == 1 => { ypos = 600.0; xpos = rng.gen_range(0, 800) as f32; },
            s if s == 2 => { ypos = rng.gen_range(0, 600) as f32; xpos = 0.0; },
            s if s == 3 => { ypos = rng.gen_range(0, 600) as f32; xpos = 600.0; },
            _ => { ypos = ypos; xpos = xpos},
        };

        Enemy {
            hp: rng.gen_range(1, 3) as u32,
            x: xpos,
            y: ypos,
            hitbox: graphics::Rect::new(xpos, ypos, 15.0, 15.0),
        }
    }
    
    pub fn update(&mut self, _ctx: &mut Context, xpos: f32, ypos: f32) {
        if self.x >= xpos + 25.0 {
            self.x -= 10.0;
        }
        if self.x <= xpos + 25.0 {
            self.x += 10.0;
        }

        if self.y >= ypos + 25.0 {
            self.y -= 10.0;
        }
        if self.y <= ypos + 25.0 {
            self.y += 10.0;
        }

        self.hitbox.x = self.x;
        self.hitbox.y = self.y;
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mesh: Mesh = graphics::MeshBuilder::new()
        .rectangle(graphics::DrawMode::fill(), self.hitbox, graphics::Color::from_rgb(255u8, 0u8, 0u8))
        .build(ctx).expect("Failed to draw Enemy");

        let my_dest = nalgebra::Point2::new(0.0, 0.0);
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(my_dest)).expect("");
    }

    pub fn update_hp(&mut self, bullets: &Vec<Bullet>) {
        bullets.iter().for_each(|b| {
            if b.hitbox.overlaps(&self.hitbox) && self.hp != 0 {
                self.hp -= b.dmg;
            }
        });
    }

    pub fn isalive(&self) -> bool {
        self.hp > 0
    }
}