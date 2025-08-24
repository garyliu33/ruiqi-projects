mod game;
mod player;

fn main() {
    let mut game = game::Game::new();
    game.display_instructions();

    loop {
        game.take_turn();
        game.display();
        if game.is_game_over() {
            println!("Thanks for playing!");
            break;
        }
        game.switch_player();
    }
}
