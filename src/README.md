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