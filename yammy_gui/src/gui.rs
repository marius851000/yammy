use std::collections::HashMap;
use yammy_core::Game;

/// A generic trait for implement a Graphical User Interface. It happen in two phase:
/// - The user configure the Game. The control is given to the caller
/// - The modification interface. The GUI have a full control of itself. It is used to modify a mod.
pub trait GUI {
    /// Ask a question with multiple answer to the user. It return the key of the answer select by the user
    /// The key of the answer may be displayed to user depending on the GUI
    fn ask_option(&mut self, question: &str, answer: &HashMap<String, String>) -> String {
        let mut question = String::from(question);
        let mut answer_vec: Vec<(String, String)> = Vec::new();
        for (key, value) in answer {
            answer_vec.push((key.clone(), value.clone()));
        }
        for (answer_id, item) in answer_vec.iter().enumerate() {
            question += &format!("\n{}: {}", answer_id, item.1);
        }
        loop {
            let user_answer = self.ask_string(&question);
            let user_answer = user_answer.trim();
            let number = match usize::from_str_radix(&user_answer, 10) {
                Ok(number) => number,
                Err(_) => {
                    self.display("The number is invalid");
                    continue;
                }
            };
            if number >= answer_vec.len() {
                self.display("The number is too big");
                continue;
            };
            return answer_vec[number].0.clone();
        }
    }
    /// Ask a user to enter a string. Return the entered string
    fn ask_string(&mut self, question: &str) -> String;
    /// Display a string to the user
    fn display(&mut self, to_display: &str);
    /// Open the GUI to edit a game
    fn edit(&mut self, game: Box<dyn Game>);
    /// Define the name of the game
    fn set_game_name(&mut self, _game_name: String) {}
}
