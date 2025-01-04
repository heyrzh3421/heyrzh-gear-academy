#[cfg(test)]
mod tests {
    use gtest::{Program, System};
    use pebbles_game_io::*;


    #[test]
    fn test_init() {
        let system = System::new();

        system.init_logger();

  }
}
