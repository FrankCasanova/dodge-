use godot::classes::{AnimatedSprite2D, IRigidBody2D, RigidBody2D};
use godot::prelude::*;


#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {

    pub min_speed: real,
    pub max_speed: real,

    base: Base<RigidBody2D>
}

#[godot_api]
impl IRigidBody2D for Mob {
    /// This function initializes a new Mob instance.
    ///
    /// It takes a `Base<RigidBody2D>` as an argument, which represents the base node for the mob.
    ///
    /// It returns a new instance of `Mob` with the following properties:
    /// - `min_speed`: The minimum speed of the mob, in units of the physics engine.
    /// - `max_speed`: The maximum speed of the mob, in units of the physics engine.
    /// - `base`: The base node of the mob, which is a `RigidBody2D`.
    ///
    /// The default values of `min_speed` and `max_speed` are 300.0 and 600.0 respectively.
    /// These values are arbitrary and may be changed in the future.
    fn init(base: Base<RigidBody2D>) -> Self {
        Mob {
            min_speed: 300.0,
            max_speed: 600.0,
            base
        }
    }

    fn ready(&mut self) {
        // Get a reference to the AnimatedSprite2D node which is a child of the mob node.
        // This is done by calling the `get_node_as` method on the base node (the mob node)
        // and specifying the path to the AnimatedSprite2D node.
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        // Play the animation on the AnimatedSprite2D node.
        // This will start the animation playing.
        sprite.play();

        // Commented out code that attempts to play a random animation.
        // This code is not currently working as intended.
        // let anim_names = sprite
        //     .get_sprite_frames().unwrap()
        //     .get_animation_names()
        //     .to_vec();

        // let mut rng = rand::rng(); //adaptation to 0.9 rand

        // let anim_name = anim_names.choose(&mut rng).unwrap();

        // Set the animation to 'fly'. This is a placeholder until the random
        // animation code is fixed.
        sprite.set_animation("fly");
        
    }
}

