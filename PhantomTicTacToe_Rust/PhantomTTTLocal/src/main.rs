mod game;
mod player;

fn main() {
    let mut game = game::Game::new();
    game.display_instructions();

    loop {
        game.take_turn();
        game.switch_player();
        println!();
        if game.is_game_over() {
            game.display_final();
            println!("Thanks for playing!");
            break;
        }
        game.display();
    }
}
