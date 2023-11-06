mod prelude;
use prelude::*;

    // The function receives a valid player and board, and returns a tuple with the updated player and board.
    #[test]
    fn test_valid_player_and_board() {
        // Arrange
        let player = Player {
            speed: 1,
            x: 1,
            y: 1,
            width: 1,
            height: 1,
        };
        let board = vec![vec![' '; 3]; 3];
    
        // Act
        let result = position_player(player, board);
    
        // Assert
        assert_eq!(result.0.x, 1);
        assert_eq!(result.0.y, 1);
        assert_eq!(result.1[1][1], '|');
    }
    
        // The player is positioned correctly on the board according to its dimensions.
    #[test]
    fn test_player_positioned_correctly() {
        // Arrange
        let player = Player {
            speed: 1,
            x: 0,
            y: 0,
            width: 2,
            height: 2,
        };
        let board = vec![vec![' '; 3]; 3];
    
        // Act
        let result = position_player(player, board);
    
        // Assert
        assert_eq!(result.0.x, 0);
        assert_eq!(result.0.y, 0);
        assert_eq!(result.1[0][0], '|');
        assert_eq!(result.1[0][1], '|');
        assert_eq!(result.1[1][0], '|');
        assert_eq!(result.1[1][1], '|');
    }
    
        // The player is positioned correctly on the board when it is at the edge of the board.
    #[test]
    fn test_player_positioned_at_edge() {
        // Arrange
        let player = Player {
            speed: 1,
            x: 2,
            y: 2,
            width: 1,
            height: 1,
        };
        let board = vec![vec![' '; 3]; 3];
    
        // Act
        let result = position_player(player, board);
    
        // Assert
        assert_eq!(result.0.x, 2);
        assert_eq!(result.0.y, 2);
        assert_eq!(result.1[2][2], '|');
    }
    
        // The function receives a player with negative x and y coordinates, and returns a tuple with the player positioned at (0,0) and the original board.
    #[test]
    fn test_negative_coordinates() {
        // Arrange
        let player = Player {
            speed: 1,
            x: -1,
            y: -1,
            width: 1,
            height: 1,
        };
        let board = vec![vec![' '; 3]; 3];
    
        // Act
        let result = position_player(player, board);
    
        // Assert
        assert_eq!(result.0.x, 0);
        assert_eq!(result.0.y, 0);
        assert_eq!(result.1[0][0], '|');
    }
    
        // The function receives a player with dimensions greater than the board's dimensions, and returns a tuple with the original player and board.
    #[test]
    fn test_player_dimensions_greater_than_board() {
        // Arrange
        let player = Player {
            speed: 1,
            x: 0,
            y: 0,
            width: 3,
            height: 3,
        };
        let board = vec![vec![' '; 2]; 2];
        let expect_board = vec![vec!['|'; 2]; 2];
        
        // Act
        let result = position_player(player, board);
    
        // Assert
        assert_eq!(result.0.x, 0);
        assert_eq!(result.0.y, 0);
        assert_eq!(result.1, expect_board);
    }
    
        // The function receives a board with dimensions 0x0, and returns a tuple with the original player and board.
    #[test]
    fn test_empty_board() {
        // Arrange
        let player = Player {
            speed: 1,
            x: 1,
            y: 1,
            width: 1,
            height: 1,
        };
        let board: Vec<Vec<char>> = vec![];
        let expect_board: Vec<Vec<char>> = vec![];
    
        // Act
        let result = position_player(player, board);
    
        // Assert
        assert_eq!(result.0.x, 1);
        assert_eq!(result.0.y, 1);
        assert_eq!(result.1, expect_board);
    }
    
