mod prelude;
use crate::prelude::*;

    // Ball can be created with initial position and speed
    #[test]
    fn test_ball_creation() {
        let ball = Ball::new(10, 20);
        assert_eq!(ball.x, 5);
        assert_eq!(ball.y, 10);
        assert_eq!(ball.speed, 1);
    }
    
        // Player can be created with initial position, speed, height and width
    #[test]
    fn test_player_creation() {
        let player = Player::new();
        assert_eq!(player.x, 0);
        assert_eq!(player.y, 0);
        assert_eq!(player.speed, 1);
        assert_eq!(player.height, var("LINES").expect("Erro ao coletar 'LINES'").trim().parse::<i32>().expect("Erro ao converter para inteiro")/3);
        assert_eq!(player.width, 2);
    }
    
        // Scoreboard can be created with initial score of 0 for both home and visit
    #[test]
    fn test_scoreboard_creation() {
        let scoreboard = Scoreboard::new();
        assert_eq!(scoreboard.home, 0);
        assert_eq!(scoreboard.visit, 0);
    }
    
        // Ball can be created with odd number of lines and columns
    #[test]
    fn test_ball_odd_lines_columns() {
        let ball = Ball::new(7, 9);
        assert_eq!(ball.x, 3);
        assert_eq!(ball.y, 4);
        assert_eq!(ball.speed, 1);
    }
    
        // Player can be created with odd number of lines in the terminal
    #[test]
    fn test_player_odd_lines_terminal() {
        let player = Player::new();
        assert_eq!(player.x, 0);
        assert_eq!(player.y, 0);
        assert_eq!(player.speed, 1);
        assert_eq!(player.height, var("LINES").expect("Erro ao coletar 'LINES'").trim().parse::<i32>().expect("Erro ao converter para inteiro")/3);
        assert_eq!(player.width, 2);
    }
    
        // Player height is calculated correctly when the number of lines in the terminal is not divisible by 3
    #[test]
    fn test_player_height_calculation() {
        let player = Player::new();
        let lines = var("LINES").expect("Erro ao coletar 'LINES'").trim().parse::<i32>().expect("Erro ao converter para inteiro");
        let expected_height = if lines % 3 == 0 { lines / 3 } else { 5 };
        assert_eq!(player.height, expected_height);
    }
    