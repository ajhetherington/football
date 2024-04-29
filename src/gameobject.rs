use crate::pitch::Pitch;
use crate::position::Position;
use crate::visibleplayer::VisiblePlayer;
use std::f32::consts::E;
use macroquad::miniquad::conf::Platform;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct GameObject {
    pub pos: Position,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub radius: f32,
    mass: f32,
    friction: f32,
}

const WALL_FRICTION: f32 = 0.99;
const ELASTICITY: f32 = 0.9;
const PLAYER_ELASTICITY: f32 = 0.7;

impl GameObject {
    pub fn new(x: f32, y: f32, radius: f32, mass: f32, friction: f32) -> Self {
        GameObject {
            pos: Position {
                x,
                y,
                prev_x: x,
                prev_y: y,
            },
            x_velocity: 0.0,
            y_velocity: 0.0,
            radius,
            mass,
            friction,
        }
    }

    pub fn apply_force(&mut self, force_x: f32, force_y: f32, dt: f32) {
        let accel_x = force_x / self.mass;
        let accel_y = force_y / self.mass;
        self.x_velocity += dt * accel_x;
        self.y_velocity += dt * accel_y;
    }
    pub fn apply_friction(&mut self, dt: f32) {
        // v(t+dt)=v(t)× exp(−friction×dt)
        let decay_factor = E.powf(-self.friction * dt);
        self.x_velocity *= decay_factor;
        self.y_velocity *= decay_factor;
        if self.x_velocity.abs() < 0.001 {
            self.x_velocity = 0.0
        }
        if self.y_velocity.abs() < 0.001 {
            self.y_velocity = 0.0
        }
    }

    fn check_wall_collisions(
        &mut self,
        x_position: f32,
        y_position: f32,
        pitch: &Pitch,
    ) -> (f32, f32) {
        let mut updated_x_position = x_position;
        let mut updated_y_position = y_position;
        // checking
        if updated_x_position <= (pitch.x as f32) + self.radius {
            updated_x_position = (pitch.x as f32) + self.radius;
            self.x_velocity = -1.0 * ELASTICITY * WALL_FRICTION * self.x_velocity;
            self.y_velocity = WALL_FRICTION * self.y_velocity;
        } else if updated_x_position >= ((pitch.x + pitch.width) as f32) - self.radius {
            updated_x_position = ((pitch.x + pitch.width) as f32) - self.radius;
            self.x_velocity = -1.0 * ELASTICITY * WALL_FRICTION * self.x_velocity;
            self.y_velocity = WALL_FRICTION * self.y_velocity;
        }

        if updated_y_position <= (pitch.y as f32) + self.radius {
            updated_y_position = (pitch.y as f32) + self.radius;
            self.y_velocity = -1.0 * ELASTICITY * WALL_FRICTION * self.y_velocity;
            self.x_velocity = WALL_FRICTION * self.x_velocity;
        } else if updated_y_position >= ((pitch.y + pitch.height) as f32) - self.radius {
            updated_y_position = ((pitch.y + pitch.height) as f32) - self.radius;
            self.y_velocity = -1.0 * ELASTICITY * WALL_FRICTION * self.y_velocity;
            self.x_velocity = WALL_FRICTION * self.x_velocity;
        }

        (updated_x_position, updated_y_position)
    }

    // How best to go about collision checking? brute force has squared scaling with 
    // the number of objects, could sort by x position, then narrow down to 'candidate' 
    // collisions, do i have to do this in all dimensions? surely just checking
    // one dimension is enough? but doing both would narrow down more.
    

    pub fn update_position(&mut self, pitch: &Pitch, dt: f32) {
        let mut updated_x_position = self.pos.x + (self.x_velocity * dt);
        let mut updated_y_position = self.pos.y + (self.y_velocity * dt);

        (updated_x_position, updated_y_position) = self.check_wall_collisions(updated_x_position, updated_y_position, pitch);

        self.pos.prev_x = self.pos.x;
        self.pos.prev_y = self.pos.y;
        self.pos.x = updated_x_position;
        self.pos.y = updated_y_position;
    }
}


/// objects comprises all players in the pitch
pub fn arange_checks(objects: &mut Vec<Rc<RefCell<VisiblePlayer>>>, radius: f32) {
    // println!("{:?}",objects);
    objects.sort_by(|a, b|  {
        println!("{:?}", a);
        println!("{:?}", b);
        match a.borrow().object.pos.x.partial_cmp(&b.borrow().object.pos.x) {
            Some(val) => val,
            None => panic!("{:?} - {:?}", a.borrow().object.pos.x, b.borrow().object.pos.x)

        }
    }
    );
    let largest_index = objects.len() - 1;

    let mut combo_s: Vec<(usize, usize)> = vec![];
    for (index, obj) in objects.iter().enumerate() {
        let mut i = index + 1;
        loop {
            if i > largest_index {
                break
            }
            if (obj.borrow().object.pos.x - objects[i].borrow().object.pos.x).abs() < radius {
                println!("this {:?}",(obj.borrow().object.pos.x - objects[i].borrow().object.pos.x).abs());
                combo_s.push((index, i));
                i += 1;
                continue
            }
            break
        }
    }

    for (index_a, index_b) in combo_s {
        assert_ne!(index_a, index_b, "Indices must be different.");
        let p1 = &mut objects[index_a].borrow_mut().object;
        let p2 = &mut objects[index_b].borrow_mut().object;
        // check_player_collisions(p1, p2);
        inelastic_check_player_collisions(p1, p2);
    }

}



/// didn't quite work, couldn't work out what was going wrong, Don't use this, instead use `inelastic_check_player_collisions`
pub fn old_check_player_collisions(
    object_a: &mut GameObject,
    object_b: &mut GameObject,
) {
    let sequared_term = (object_a.pos.x - object_b.pos.x).powf(2.0) + (object_a.pos.y - object_b.pos.y).powf(2.0);
    if sequared_term >= (object_a.radius + object_b.radius).powf(2.0) {
        return
    }

    println!("past early return {:?} {:?}", object_a, object_b);
    let distance = sequared_term.sqrt();
    let dir_x = (object_b.pos.x - object_a.pos.x) / distance;
    let dir_y = (object_b.pos.y - object_a.pos.y) / distance;
    let half_overlap = (object_a.radius + object_b.radius - distance) / 2.0;

    // update positions by moving by half the marginal distance
    object_a.pos.x = object_a.pos.x - (dir_x * half_overlap);
    object_a.pos.y = object_a.pos.y - (dir_y * half_overlap);
    object_b.pos.x = object_b.pos.x + (dir_x * half_overlap);
    object_b.pos.y = object_b.pos.y + (dir_y * half_overlap);

    let rel_velocity_x = object_b.x_velocity - object_a.x_velocity;
    let rel_velocity_y = object_b.y_velocity - object_a.y_velocity;
    let impact_velocity = (rel_velocity_x * dir_x) + (rel_velocity_y * dir_y);

    // change in velocity due to impact
    let total_mass = object_a.mass + object_b.mass;
    let j = (2.0 * impact_velocity) / total_mass;
    object_a.x_velocity = object_a.x_velocity + (j * dir_x * (object_b.mass / total_mass));
    object_a.y_velocity = object_a.y_velocity + (j * dir_y * (object_b.mass / total_mass));
    object_b.x_velocity = object_b.x_velocity + (j * dir_x * (object_a.mass / total_mass));
    object_b.y_velocity = object_b.y_velocity + (j * dir_y * (object_a.mass / total_mass));
    println!("doing something");

}



/// Inelastic collisions between players, see `PLAYER_ELASTICITY`
pub fn inelastic_check_player_collisions(
    object_a: &mut GameObject,
    object_b: &mut GameObject,
) {
    let squared = (object_a.pos.x - object_b.pos.x).powf(2.0) + (object_a.pos.y - object_b.pos.y).powf(2.0);
    if squared >= (object_a.radius + object_b.radius).powf(2.0) {
        return
    }

    let total_mass = object_a.mass + object_b.mass;
    let a_x_vel = (((object_a.mass - PLAYER_ELASTICITY * object_b.mass) * object_a.x_velocity) + (1.0 + PLAYER_ELASTICITY)*object_b.mass*object_b.x_velocity) / (total_mass);
    let a_y_vel = (((object_a.mass - PLAYER_ELASTICITY * object_b.mass) * object_a.y_velocity) + (1.0 + PLAYER_ELASTICITY)*object_b.mass*object_b.y_velocity) / (total_mass);
    let b_x_vel = (((object_b.mass - PLAYER_ELASTICITY * object_a.mass) * object_b.x_velocity) + (1.0 + PLAYER_ELASTICITY)*object_a.mass*object_a.x_velocity) / (total_mass);
    let b_y_vel = (((object_b.mass - PLAYER_ELASTICITY * object_a.mass) * object_b.y_velocity) + (1.0 + PLAYER_ELASTICITY)*object_a.mass*object_a.y_velocity) / (total_mass);

    object_a.x_velocity = a_x_vel;
    object_a.y_velocity = a_y_vel;
    object_b.x_velocity = b_x_vel;
    object_b.y_velocity = b_y_vel;

    let distance = squared.sqrt();
    let dir_x = (object_b.pos.x - object_a.pos.x) / distance;
    let dir_y = (object_b.pos.y - object_a.pos.y) / distance;
    let half_overlap = (object_a.radius + object_b.radius - distance) / 2.0;

    // update positions by moving by half the marginal distance
    object_a.pos.x = object_a.pos.x - (dir_x * half_overlap);
    object_a.pos.y = object_a.pos.y - (dir_y * half_overlap);
    object_b.pos.x = object_b.pos.x + (dir_x * half_overlap);
    object_b.pos.y = object_b.pos.y + (dir_y * half_overlap);
}
