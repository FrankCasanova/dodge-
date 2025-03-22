use crate::{mob, player, hud};

use godot::classes::{Marker2D, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;

use rand::seq::IndexedMutRandom;
use rand::Rng as _;
use std::f32::consts::PI;


#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameScene {
    mob_scene: OnReady<Gd<PackedScene>>,
    player: OnReady<Gd<player::Player>>,
    hud: OnReady<Gd<hud::HUD>>,
    music: OnReady<Gd<AudioStreamPlayer>>,
    death_sound: OnReady<Gd<AudioStreamPlayer>>,

    fire_sound: OnReady<Gd<AudioStreamPlayer>>,
    fire_sound2: OnReady<Gd<AudioStreamPlayer>>,
    fire_sound3: OnReady<Gd<AudioStreamPlayer>>,
    fire_sound4: OnReady<Gd<AudioStreamPlayer>>,
    fire_sound5: OnReady<Gd<AudioStreamPlayer>>,

    score5: OnReady<Gd<AudioStreamPlayer>>,
    score10: OnReady<Gd<AudioStreamPlayer>>,
    score20: OnReady<Gd<AudioStreamPlayer>>,
    score25: OnReady<Gd<AudioStreamPlayer>>,
    score35: OnReady<Gd<AudioStreamPlayer>>,
    score50: OnReady<Gd<AudioStreamPlayer>>,

    score: i32,

    base: Base<Node>,
}

#[godot_api]
impl INode for GameScene {
    /// This function initializes a new `GameScene` instance.
    ///
    /// It takes a `Base<Node>` as an argument, which represents the base node for the game scene.
    ///
    /// It returns a new instance of `GameScene` with the following properties:
    ///
    /// - `mob_scene`: A reference to the "res://mob.tscn" scene which is used to create new mob instances.
    /// - `player`: A reference to the "Player" node which is the player character.
    /// - `hud`: A reference to the "HUD" node which is the heads-up display.
    /// - `music`: A reference to the "GameSoundTrack" node which is the background music.
    /// - `death_sound`: A reference to the "DeathSound" node which is the sound effect played when the player dies.
    /// - `fire_sound`, `fire_sound2`, `fire_sound3`, `fire_sound4`, `fire_sound5`: A reference to the "FireSound", "FireSound2", "FireSound3", "FireSound4" and "FireSound5" nodes which are the sound effects played when the player fires their weapon.
    /// - `score5`, `score10`, `score20`, `score25`, `score35`, `score50`: A reference to the "Score5", "Score10", "Score20", "Score25", "Score35" and "Score50" nodes which are the sound effects played when the player scores 5, 10, 20, 25, 35 and 50 points respectively.
    /// - `score`: An integer which is the current score of the player.
    ///
    /// The `OnReady` type is used to wait for the scene to finish loading and for the nodes to be ready.
    fn init(base: Base<Node>) -> Self {
        Self {
            mob_scene: OnReady::from_loaded("res://mob.tscn"),
            player: OnReady::from_node("Player"),
            hud: OnReady::from_node("HUD"),
            music: OnReady::from_node("GameSoundTrack"),
            death_sound: OnReady::from_node("DeathSound"),
            fire_sound: OnReady::from_node("FireSound"),
            fire_sound2: OnReady::from_node("FireSound2"),
            fire_sound3: OnReady::from_node("FireSound3"),
            fire_sound4: OnReady::from_node("FireSound4"),
            fire_sound5: OnReady::from_node("FireSound5"),
            score5: OnReady::from_node("Score5"),
            score10: OnReady::from_node("Score10"),
            score20: OnReady::from_node("Score20"),
            score25: OnReady::from_node("Score25"),
            score35: OnReady::from_node("Score35"),
            score50: OnReady::from_node("Score50"),
            score: 0,
            base
        }
    }

    fn ready(&mut self) {
        // Get a reference to the main node.
        let main = self.to_gd();

        // Connect the "hit" signal of the player to the "game_over" method of the main node.
        // This means that when the player is hit, the game_over method will be called.
        self.player
            .signals() // Get the signals of the player node.
            .hit() // Get the "hit" signal of the player node.
            .connect_obj( // Connect the "hit" signal to a method of an object.
                &main, // The object to connect the signal to.
                Self::game_over // The method to call when the signal is emitted.
            );

        // Connect the "start_game" signal of the HUD to the "new_game" method of the main node.
        // This means that when the start button is pressed, the new_game method will be called.
        self.hud
            .signals() // Get the signals of the HUD node.
            .start_game() // Get the "start_game" signal of the HUD node.
            .connect_obj( // Connect the "start_game" signal to a method of an object.
                &main, // The object to connect the signal to.
                Self::new_game // The method to call when the signal is emitted.
            );
    }
}

#[godot_api]
impl GameScene {

    fn spawn_fireball(&mut self) {
        // Create an array of mutable references to the fire sound nodes.
        // These nodes correspond to different fireball sound effects in the game.
        let mut fireballs = [
            &mut self.fire_sound,
            &mut self.fire_sound2,
            &mut self.fire_sound3,
            &mut self.fire_sound4,
            &mut self.fire_sound5,
        ];
        
        // Create a random number generator.
        // This will be used to select a random sound from the array.
        let mut rng = rand::rng();

        // Use the 'choose_mut' method to select a random element from the 'fireballs' array.
        // This method returns an Option containing a mutable reference to a randomly selected element.
        if let Some(random_sound) = fireballs.choose_mut(&mut rng) {
            // If a sound was successfully selected, play the sound.
            // This effectively plays a random fireball sound effect.
            random_sound.play();
        }
    }
    
    fn game_over(&mut self) {

        // Stop the score timer.
        // This is used to increment the score at certain intervals.
        // Stopping the timer prevents the score from increasing during the game over screen.
        self
            .base()
            .get_node_as::<Timer>("ScoreTimer")
            .stop();

        // Stop the mob timer.
        // This is used to spawn new mob instances at certain intervals.
        // Stopping the timer prevents new mobs from spawning during the game over screen.
        self
            .base()
            .get_node_as::<Timer>("MobTimer")
            .stop();

        // Show the game over screen.
        // This is done by calling the show_game_over method on the HUD node.
        self.hud.bind_mut().show_game_over();

        // Stop the background music.
        // This is done by calling the stop method on the music node.
        self.music.stop();

        // Play the death sound effect.
        // This is done by calling the play method on the death_sound node.
        self.death_sound.play();
    }

    fn new_game(&mut self) {
        // Get a reference to the starting position marker.
        // This is used to determine where the player should be positioned at the start of the game.
        let start_position = self
            .base()
            .get_node_as::<Marker2D>("StartPosition");

        // Start the timer associated with the start of the game.
        // This timer may be used to delay certain actions, such as spawning mobs.
        self
            .base()
            .get_node_as::<Timer>("StartTimer")
            .start();

        // Optionally clear all mobs from the scene.
        // Uncomment the line below if you want to clear mobs when starting a new game.
        // self.base().get_tree().unwrap().call_group("mobs", "queue_free", &[]);

        // Reset the player's score to 0 at the start of a new game.
        self.score = 0;

        // Set the player's position to the starting position and make the player visible.
        // This ensures the player starts the game at the correct location and is visible on the screen.
        self.player.bind_mut().start(start_position.get_position());

        // Update the HUD to reflect the new score and show a starting message.
        // The score is reset to 0, and a message "Die!" is displayed to the player.
        let mut hud = self.hud.bind_mut(); // Create a mutable reference to the HUD.
        hud.update_score(self.score);
        hud.show_message("Die!".into());

        // Play the background music for the game.
        // This creates an immersive environment for the player as they play the game.
        self.music.play();
    }

    #[func]
    pub fn on_start_timer_timeout(&mut self) {
        self.base_mut().get_node_as::<Timer>("MobTimer").start();
        self.base_mut().get_node_as::<Timer>("ScoreTimer").start();
    }

    #[func]
    pub fn on_score_timer_timeout(&mut self) {
        // This function is called every second (because the timer interval is set to 1 second).
        // It increments the player's score by 1 and updates the HUD to reflect the new score.

        // Increment the player's score.
        self.score += 1;

        // Update the HUD to reflect the new score.
        self.hud.bind_mut().update_score(self.score);

        // Play a sound effect when the player reaches certain score milestones.
        // The sound effects are played when the player scores 5, 10, 20, 25, 35 and 50 points.
        match self.score {
            5 => {
                // Play the sound effect for scoring 5 points.
                self.score5.play();
            },
            10 => {
                // Play the sound effect for scoring 10 points.
                self.score10.play();
            },
            20 => {
                // Play the sound effect for scoring 20 points.
                self.score20.play();
            },
            25 => {
                // Play the sound effect for scoring 25 points.
                self.score25.play();
            },
            35 => {
                // Play the sound effect for scoring 35 points.
                self.score35.play();
            },
            50 => {
                // Play the sound effect for scoring 50 points.
                self.score50.play();
            },
            _ => {
                // Do nothing if the player's score is not one of the milestone scores.
            },
        }
    }

    #[func]
    pub fn on_mob_timer_timeout(&mut self) {
        // This function is called when the `MobTimer` times out.
        // It is responsible for spawning a new mob instance at a random location along the `MobPath`.
        // The mob is also given a random direction and speed.

        // Get a reference to the `MobSpawnLocation` PathFollow2D node.
        // This node is used to determine the position of the mob spawn location.
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");

        // Instantiate a new mob scene using the `mob_scene` PackedScene.
        // The `mob_scene` is a reference to the packed scene of the mob.
        let mut mob_scene = self
            .mob_scene
            .instantiate_as::<RigidBody2D>();

        // Generate a random progress value between 0.0 and 1.0.
        // This value is used to determine the position of the mob spawn location.
        let mut rng = rand::rng(); //adaptation to 0.9 rand
        let progress = rng.random_range(u16::MIN..u16::MAX); //adaptation to 0.9 rand

        // Set the progress of the `MobSpawnLocation` node to the random value.
        // This sets the position of the mob spawn location.
        mob_spawn_location.set_progress(progress as f32);

        // Set the position of the mob scene to the position of the mob spawn location.
        mob_scene.set_position(mob_spawn_location.get_position());

        // Generate a random direction for the mob.
        // The direction is a value between -PI/4 and PI/4 radians.
        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        direction += rng.random_range(-PI / 4.0..PI / 4.0);

        // Set the rotation of the mob scene to the random direction.
        mob_scene.set_rotation(direction);

        // Add the mob scene to the scene tree.
        self.base_mut()
            .add_child(&mob_scene);

        // Get a reference to the mob instance.
        let mut mob = mob_scene.cast::<mob::Mob>();

        // Generate a random speed value between the minimum and maximum speed of the mob.
        let range = {
            let mob = mob.bind();
            rng.random_range(mob.min_speed..mob.max_speed)
        }; //adaptation to 0.9 rand

        // Set the linear velocity of the mob to the random speed value.
        // The direction of the velocity is determined by the rotation of the mob scene.
        mob.set_linear_velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));

        // Call the `spawn_fireball` function to play a random fireball sound effect.
        self.spawn_fireball();
    }
}
