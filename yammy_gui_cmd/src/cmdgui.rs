use std::io;
use yammy_core::Game;
use yammy_gui::GUI;

pub struct CMDGUI {
    _game_name: Option<String>,
}

impl CMDGUI {
    pub fn init() -> Self {
        CMDGUI { _game_name: None }
    }
}

impl GUI for CMDGUI {
    fn ask_string(&mut self, question: &str) -> String {
        println!("{}", question);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input
    }
    fn display(&mut self, to_display: &str) {
        println!("{}", to_display);
        println!("press enter to continue");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
    }
    fn edit(&mut self, _game: Box<dyn Game>) {
        todo!()
    }
    fn set_game_name(&mut self, game_name: String) {
        println!("game name set to {}.", game_name);
        self._game_name = Some(game_name);
    }
}
