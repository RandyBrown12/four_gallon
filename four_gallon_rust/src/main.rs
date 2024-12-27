use std::vec::Vec;

fn pour_gallons(gallons: &[i8; 2], pouring_to: i8) -> [i8; 2] {
    
    //! # Description
    //! Given two gallons, we will "pour" or transfer 1 unit of water from one gallon to the
    //! other gallon and check for every transfer if we have hit the maximum for the other gallon or if we
    //! have nothing (0) to pour to the other gallon.
    //!
    //! ## Arguments
    //! gallons: `&[i8; 2]`
    //!     An array containing two gallons
    //!
    //! pouringTo: i8
    //!     The index of the array in gallons to where the gallon is being poured to
    //!
    //! ## Return
    //! 
    //! `[i8; 2]`:
    //!     Updated gallons once pouring is complete.

    // Check a mutatable deep copy from gallons
    let mut new_gallons = gallons.clone();

    // Perform the following to which index pouring_to will go.
    if pouring_to == 0 {

        // Continue Pouring until new_gallons[0] has be filled or new_gallons[1] is empty.
        while new_gallons[1] != 0 && new_gallons[0] != 3 {
            
            // Subtract one unit to new_gallons[1]
            new_gallons[1] -= 1;

            // Add one unit to new_gallons[0]
            new_gallons[0] += 1;
        }
    } else if pouring_to == 1 {
        // Continue Pouring until new_gallons[0] is empty or new_gallons[1] is filled.
        while new_gallons[0] != 0 && new_gallons[1] != 5 {
            
            // Add one unit to new_gallons[1]
            new_gallons[1] += 1;

            // Subtract one unit to new_gallons[0]
            new_gallons[0] -= 1;
        }
    }
    return new_gallons;
}

fn four_gallon(gallons: [i8; 2], moves: &Vec<[i8; 2]>) {
    //! # Description
    //!
    //! Solve the problem of 2 kids fetching 4 gallons of water from a stream,
    //! with only a 3-gallon bucket, and an unmarked 5-gallon bucket in less than 15 steps.
    //!
    //! First for initialization, `gallons[0]` will be the 3-gallon bucket and `gallons[1]` will be the 5-gallon bucket.
    //! Next, we will create a path for four_gallon() to introduce recursion with backtracking. We will use this path
    //! to simulate the moves four_gallon() has made.
    //!
    //! For our base cases, we check if a gallon contains 4 units of water and is less than 15 moves. Once we
    //! checked for that, check if a move is unique or the moves taken is greater than or equal to 15.
    //! 
    //! For our induction cases, we wants to initialize all possible moves the kids could perform with the move
    //! being added to the next function call.
    //! A.) Pouring from one gallon to another.
    //! B.) Filling a gallon.
    //! C.) Emptying a gallon.
    //! 
    //! ## Arguments
    //! * gallons: `[i8 ; 2]`
    //!     An address of an array of two gallons with the units of water acting as integers.
    //! * moves: `&Vec<[i8; 2]>`
    //!     A vector of an array of two integers containing the gallons.
    
    // Perform Deep copy of the vector
    let mut new_moves = moves.clone();

    // Append the gallons to the new_moves
    new_moves.push(gallons);

    // Check if one of the gallons is 4 units and if the new_moves is less than 15
    if gallons[0] == 4 || gallons [1] == 4 && new_moves.len() < 15 {
        println!("A legal path in {} moves is: ", new_moves.len());
        // Convert the [i8; 2] into a [String] which can be joined.
        let formatted_moves = new_moves.iter()
        .map(| gallon| 
            format!("[{}, {}]", gallon[0].to_string(), gallon[1].to_string()))
        .collect::<Vec<String>>()
        .join(" -> ");
        
        // Print out the formatted_moves
        println!("{}", formatted_moves);
        return;
    } 
    
    // Check if the move is unique and if we have perform 15 or greater moves.
    if moves.contains(&gallons) || new_moves.len() >= 15 {
        return;
    }

    // Simulate a pour
    four_gallon(pour_gallons(&gallons, 1), &new_moves);
    four_gallon(pour_gallons(&gallons, 0), &new_moves);

    // Simulate a fill
    four_gallon([3, gallons[1]], &new_moves);
    four_gallon([gallons[0], 5], &new_moves);

    // Simulate an empty
    four_gallon([0, gallons[1]], &new_moves);
    four_gallon([gallons[0], 0], &new_moves);
}

fn main() {
    // Beginner Function to perform four_gallon
    four_gallon([0,0], &Vec::new());
}
