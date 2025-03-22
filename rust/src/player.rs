
use godot::classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, PhysicsBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    speed: real,
    screen_size: Vector2,

    base: Base<Area2D>
}

#[godot_api]
impl IArea2D for Player {
    // This function initializes a new Player instance.
    // It takes a Base<Area2D> as an argument, which represents the base node for the player.
    fn init(base: Base<Area2D>) -> Self {
        Player {
            // Sets the initial speed of the player to 400.0 units.
            speed: 400.0,
            // Initializes the screen size to a Vector2 with both dimensions set to 0.0.
            // This will likely be updated later with the actual screen size.
            screen_size: Vector2::new(0.0, 0.0),
            // Assigns the provided base node to the player's base field.
            base
        }
    }

    fn ready(&mut self) {
        // Set the player's screen size to the size of the viewport rectangle.
        // This is useful for ensuring the player remains within the screen boundaries.
        self.screen_size = self.base().get_viewport_rect().size;

        // Hide the player's base node initially.
        // This could be used to keep the player hidden until certain conditions are met.
        self.base_mut().hide();
    }

    fn physics_process(&mut self, delta: f64) {
        // Process the player's physics logic (movement, animation, etc.) here.
        // This function is called every frame.

        // Get the player's animated sprite node.
        let mut animated_sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        // Get the game's input singleton.
        let input = Input::singleton();

        // Create a variable to store the player's velocity.
        let mut velocity = Vector2::new(0.0, 0.0);

        // Check which direction buttons are being pressed and update velocity accordingly.
        if input.is_action_pressed("move_left") {
            // If the left arrow key is being pressed, set the x component of velocity to -1.0.
            velocity.x -= 1.0;
        }
        if input.is_action_pressed("move_right") {
            // If the right arrow key is being pressed, set the x component of velocity to 1.0.
            velocity.x += 1.0;
        }
        if input.is_action_pressed("move_up") {
            // If the up arrow key is being pressed, set the y component of velocity to -1.0.
            velocity.y -= 1.0;
        }
        if input.is_action_pressed("move_down") {
            // If the down arrow key is being pressed, set the y component of velocity to 1.0.
            velocity.y += 1.0;
        }

        // If the player is moving in some direction (i.e. velocity is not the origin), then:
        if velocity.length() > 0.0 {
            // Normalize the velocity vector (i.e. set its magnitude to 1.0) and multiply it by the player's speed.
            velocity = velocity.normalized() * self.speed;

            // Determine the correct animation to play based on the direction of the velocity.
            let animation = match (velocity.x, velocity.y) {
                // If the player is moving right, play the "run_right" animation.
                (x, y) if x.abs() > y.abs() && x > 0.0 => "run_right",
                // If the player is moving left, play the "run_left" animation.
                (x, y) if x.abs() > y.abs() && x < 0.0 => "run_left",
                // If the player is moving up, play the "run_up" animation.
                (x, y) if y.abs() >= x.abs() && y > 0.0 => "run_up",
                // If the player is moving down, play the "run_down" animation.
                (x, y) if y.abs() >= x.abs() && y < 0.0 => "run_down",
                _ => "run_down", // Default to "run_down" if none of the above conditions are true.
            };

            // Play the chosen animation.
            animated_sprite.play_ex().name(animation).done();
        } else {
            // If the player is not moving, play the corresponding idle animation.
            let current_animation = animated_sprite.get_animation();
            
            // Convert the run animation to the corresponding idle animation.
            let idle_animation = match current_animation {
                // If the player is currently playing the "run_up" animation, play the "idle_up" animation instead.
                anim if anim.contains("run_up") => "idle_up",
                // If the player is currently playing the "run_down" animation, play the "idle_down" animation instead.
                anim if anim.contains("run_down") => "idle_down",
                // If the player is currently playing the "run_left" animation, play the "idle_left" animation instead.
                anim if anim.contains("run_left") => "idle_left",
                // If the player is currently playing the "run_right" animation, play the "idle_right" animation instead.
                anim if anim.contains("run_right") => "idle_right",
                // Default to "idle_down" if none of the above conditions are true.
                _ => "idle_down",
            };

            // Play the chosen idle animation.
            animated_sprite.play_ex().name(idle_animation).done();
        }

        // Update the player's position by adding the velocity * delta to the current position.
        // Clamp the position to the edge of the screen to prevent the player from moving off the edge.
        let change = velocity * real::from_f64(delta);
        let position = self.base().get_global_position() + change;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );
        self.base_mut().set_global_position(position);
    }
}


#[godot_api]
impl Player {
    #[signal]
    pub fn hit();

    #[func]
    fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
        // This function is a callback that is called whenever a PhysicsBody2D enters the player's area.
        // The body parameter is the PhysicsBody2D that entered the player's area.
        // The _ prefix on the parameter is a naming convention to indicate that the parameter is not used in the function body.

        // Hide the player's node so that it is no longer visible.
        // This is done to prevent the player from continuing to move after they have been hit.
        self.base_mut().hide();

        // Emit a signal that the player has been hit.
        // This signal is used by the Game node to track the player's score and to trigger the game over screen.
        self.signals().hit().emit();

        // Get a reference to the player's CollisionShape2D node.
        // The CollisionShape2D node is used to detect when other nodes enter the player's area.
        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        // Disable the player's CollisionShape2D node.
        // This is done to prevent the player from continuing to detect other nodes after they have been hit.
        collision_shape.set_deferred("disable", &true.to_variant());
    }

    #[func]
    pub fn start(&mut self, position: Vector2) {
        // Set the player's position to the given position.
        // This is done to position the player at the starting location.
        self.base_mut().set_global_position(position);

        // Show the player's node.
        // This is done to make the player visible again after they have been hit and their node has been hidden.
        self.base_mut().show();

        // Get a reference to the player's CollisionShape2D node.
        // The CollisionShape2D node is used to detect when other nodes enter the player's area.
        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        // Enable the player's CollisionShape2D node.
        // This is done to allow the player to detect other nodes after they have been hit.
        // The player's CollisionShape2D node is disabled after they have been hit to prevent them from continuing to detect other nodes.
        collision_shape.set_disabled(false);
    }
}
