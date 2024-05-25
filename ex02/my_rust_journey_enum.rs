enum Action {
    Timer(u32),
    Weather(String),
    Music(String),
}

fn dispatch(action: Action) {
    match action {
        Action::Timer(int) => println!("AI -- Timer requested {}", int),
        Action::Weather(message) => println!("AI -- Weather for this city {} requested", message),
        Action::Music(song) => println!("AI -- Play following song requested {}", song),
    }
}

fn main() {
    dispatch(Action::Timer(32));
    dispatch(Action::Music("Fly -- Ludovico Einaudi".to_string()));
    dispatch(Action::Weather("Sedona - Arizona".to_string()));
}
