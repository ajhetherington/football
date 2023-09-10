## Notes

The game loop is ideal at about 60 fps. Updating all the physics every frame is a bit much. Instead we can set a tick rate for the physics, & then whenever the fps count goes past a particular modulo of tick, then we will calculate physics.

This enforces some stipulations:
1. Keep drawing methods / functions separate from physics
2. We might want to have the players / ball move still on the frames we're not doing physics, in that case we just apply their velocity, but not the friction or other physics like that. Otherwise we'll be basically keeping the game at a lower frame rate?


So `Visible` elements should have separate methods for: 
* **physics** / executing actions for players (todo)
* **applying velocity** to get a new position for the new frame (
    perhaps only do a modulo fraction of the update? ie if rendering is 60fps, & game loop is 20fps, then we can do updated positions, but applying updates means we'll have to do only 1/3 of the update at a time
)
* **drawing** to the screen


## Bit of a plan

I have made the ball struct as follows
```rust
pub struct Ball {
    pub pos: Position,
    pub x_velocity: f32,
    pub y_velocity: f32,
    radius: f32,
    mass: f32,
    friction: f32,
}
// with Position defined as
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub prev_x: f32,
    pub prev_y: f32
}
```

But really the state of a ball is very similar to that of a player, really the only thing that i might not want on a player is friction. Mass can probably be equal between players for now, but being able to have different mass / weight per player may be helpful in future (though more mass is surely a hinderance for now).

The way the ball moves is by simulating each keypress as a force applied to the object in that instance:
```rust
impl Ball{
    pub fn kick(&mut self, force_x: f32, force_y: f32, dt: f32) {
        let accel_x = force_x / self.mass;
        let accel_y = force_y / self.mass;
        self.x_velocity += dt * accel_x;
        self.y_velocity += dt * accel_y;
        println!("{:?}", self);
    }
}
```
& later i apply friction, & then update the position. Basically, the idea is for Ball and Players to each be composed structs from a common `GameObject`, which has `Position`, `x_velocity`, `y_velocity`, `mass`, `friction`. Then the methods for each struct are different, probably with not too many methods for ball (probs just render, it cannot do anything itself afterall), but Players will have methods that can kick the ball (apply a force to the ball, magnitude range relative to some of the player's stats) in a direction (with random error based on the player's passing/shooting ability). They also need methods to move, & their movement can be the same as a ball's movement, just apply a force to itself in a given direction. Players shouldn't have any friction with the ground, but instead have much more mass than the ball, which will enable agents to stop much faster relative to the ball by applying force in the opposite direction of travel.


### Passing & Shooting? 
One quick thought, there's no real difference between passing and shooting, they're just different actions that the agent takes, the physics of the two are completely the same, it's just where the agent chooses to apply the force that's different. We could do away with passing & shooting stats, & instead just have a single value for the player's Technique, & have the magnitude of the player's kick proportional to some physical 'strength'.

### Also todo:
When the player kicks the ball, should the ball also get more momentum from 'hitting' the foot? akin to the ball hitting the sides of the pitch?
