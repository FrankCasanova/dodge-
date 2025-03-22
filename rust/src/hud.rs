use godot::classes::{Button, CanvasLayer, ICanvasLayer, Label, Timer};
use godot::prelude::*;


#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct HUD {
    start_button_sfx: OnReady<Gd<AudioStreamPlayer>>,
    base: Base<CanvasLayer>,
}

#[godot_api]
impl ICanvasLayer for HUD {
    // This function initializes a new instance of the HUD struct.
    // It takes a Base<CanvasLayer> as an argument, which represents the base node for the HUD.
    fn init(base: Base<Self::Base>) -> Self {
        // Create a new instance of HUD with the following properties:
        Self {
            // Initialize the start_button_sfx field with an OnReady instance for the "StartButtonSFX" node.
            // OnReady is a utility that allows for deferred node access, meaning it will try to fetch the node when needed.
            start_button_sfx: OnReady::from_node("StartButtonSFX"),
            // Assign the provided base node to the base field of the HUD.
            base
        }
    }
}

#[godot_api]
impl HUD {
    #[signal]
    pub fn start_game();

    #[func]
    /// This function shows a message on the screen.
    ///
    /// It takes a GString as an argument, which represents the message to be displayed.
    /// The message is displayed in the "MessageLabel" node, which is a child of the HUD node.
    ///
    /// The message is displayed for 2 seconds and is then hidden.
    pub fn show_message(&mut self, text: GString) {
        // Get a reference to the "MessageLabel" node.
        let mut message_label = self
            .base()
            .get_node_as::<Label>("MessageLabel");

        // Set the text of the message label to the provided text.
        message_label.set_text(&text);

        // Show the message label.
        message_label.show();

        // Get a reference to the "MessageTimer" node.
        let mut timer  = self
            .base()
            .get_node_as::<Timer>("MessageTimer");

        // Start the timer.
        timer.start();
    }

    pub fn show_game_over(&mut self) {
        // Show the "Done! ;)" message after the game is over.
        self.show_message("Done! ;)".into());

        // Create a new Timer node with a duration of 2 seconds.
        let mut timer = self
            .base()
            .get_tree()
            .unwrap()
            .create_timer(2.0)
            .unwrap();

        // Connect the "timeout" signal of the timer to the "show_start_button" method of the HUD.
        // This will cause the "show_start_button" method to be called after 2 seconds.
        // The purpose of this is to show the start button after the game is over.
        timer.connect("timeout", &self.base().callable("show_start_button"));

    }

    #[func]
    /// This function shows the start button and a message on the screen.
    /// The message is "Fugitive!!" and is displayed in the "MessageLabel" node.
    /// The start button is shown in the "StartButton" node.
    pub fn show_start_button(&mut self) {
        // Get a reference to the "MessageLabel" node.
        let mut message_label = self.base().get_node_as::<Label>("MessageLabel");

        // Set the text of the message label to "Fugitive!!".
        message_label.set_text("Fugitive!!");

        // Show the message label.
        message_label.show();

        // Get a reference to the "StartButton" node.
        let mut start_button = self.base().get_node_as::<Button>("StartButton");

        // Show the start button.
        start_button.show();
    }

    #[func]
    /// This function is called to update the score displayed on the screen.
    /// It takes an i32 as an argument, which represents the current score.
    /// The score is displayed in the "ScoreLabel" node.
    pub fn update_score(&mut self, score: i32) {
        // Get a reference to the "ScoreLabel" node.
        let mut score_label = self.base().get_node_as::<Label>("ScoreLabel");

        // Set the text of the score label to the current score.
        // This is done by converting the score i32 to a GString and then passing that GString to the set_text method.
        score_label.set_text(&score.to_string());
            
    }

    #[func]
    /// This function is called when the start button is pressed.
    /// It is responsible for hiding the start button, emitting a signal to start the game, and playing a sound effect.
    pub fn on_start_button_pressed(&mut self) {
        // Get a reference to the start button node.
        let mut start_button = self.base().get_node_as::<Button>("StartButton");

        // Hide the start button so that it is no longer visible on the screen.
        start_button.hide();

        // Emit a signal to start the game.
        // This signal is used by the Game node to start the game.
        self.signals().start_game().emit();

        // Play a sound effect to indicate that the start button has been pressed.
        // The sound effect is associated with the "StartButtonSFX" node.
        self.start_button_sfx.play();
    }

    #[func]
    /// This function is called when the timer that is associated with the message label times out.
    /// The purpose of this function is to hide the message label after it has been shown for a certain amount of time.
    /// This is done so that the message label is no longer visible on the screen after the timer times out.
    pub fn on_message_timer_timeout(&mut self) {
        // Get a reference to the message label node.
        let mut message_label = self
            .base()
            .get_node_as::<Label>("MessageLabel");
        
        // Hide the message label.
        // This is done so that the message label is no longer visible on the screen.
        message_label.hide();
    }  
}