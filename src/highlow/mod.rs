mod game;

pub fn play_game() {
    println!("\nWelcome to high low!\nNumbers range from 0 to 255.\n");
    let mut hl = game::start_game();
    loop {
        let is_finished = hl.play_game_round();
        if is_finished {
            break;
        }
    }
}
