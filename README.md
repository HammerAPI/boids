# Boids

An implementation of the boids algorithms (Reyolds, 1986) in Rust using the Amethyst ECS game engine.

## Notes

**IMPORTANT**: The Amethyst project has undergone so significant revisions since I made this project. It may no longer work as intended. I've since [rewritten this in JavaScript](https://github.com/HammerAPI/BoidsJS) just for fun.


This project was simply for practice with ECS and the Amethyst game engine. I did not intend for it to be perfect or efficient.

This implementation is not exact. I did not follow Reynolds' algorithms directly. Because of this, the boids act a little different.

## Future plans

- Redesign the collision system so that it follows Reynolds' three rules more strictly. Now that I have more experience with Transforms, I feel more comfortable tackling this.
- Optimize the code. There are a few places in the collision system that could stand to be optimized. Calling functions twice unnecessarily, expensive calculations, etc.

## Acknowledgements

@Rapdorian, @radium-io
