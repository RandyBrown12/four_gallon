from typing import List

def four_gallon(gallons : List[int], moves : List[List[int]]):
    """
    Description
    -----------
    Solve the problem of 2 kids fetching 4 gallons of water from a stream,
    with only a 3-gallon bucket, and an unmarked 5-gallon bucket in less than 15 steps.

    First for initialization, gallons[0] will be the 3-gallon bucket and gallons[1] will be the 5-gallon bucket.

    Next, we will create a path for four_gallon() to introduce recursion with backtracking. We will use this path
    to simulate the moves four_gallon() has made.

    For our base cases, we check if a gallon contains 4 units of water and is less than 15 moves. Once we
    checked for that, check if a move is unique or the moves taken is greater than or equal to 15.

    For our induction cases, we wants to initialize all possible moves the kids could perform with the move
    being added to the next function call.
    A.) Pouring from one gallon to another.
    B.) Filling a gallon.
    C.) Emptying a gallon.

    Arguments
    ---------
    gallons: List[int]
        An array of two gallons with the units of water acting as integers.
         
    moves: List[List[int]]
        A list of a list of integers containing the gallons for.
    """

    # Perform a deep copy and appends the gallon as a new move
    new_moves = moves.copy()
    new_moves.append(gallons)

    # Perform our base cases
    # Check if one of the gallons is 4 and there is less than 15 moves made.
    if gallons[0] == 4 or gallons[1] == 4 and len(new_moves) < 15:
        print(f"A legal path in {len(new_moves)} moves is:")
        print(" -> ".join(map(str, new_moves)))
        return
    
    # Check if this move (gallons) is not unique or if there is more than or equal to 15 moves.
    if gallons in moves or len(new_moves) >= 15:
        return

    # Simulate a Pour from 3-gallon to 5-gallon
    four_gallon(pour_gallons(gallons, 1), new_moves)

    # Simulate a Pour from 5-gallon to 3-gallon
    four_gallon(pour_gallons(gallons, 0), new_moves)

    # Simulate a Fill for the 3-gallon
    four_gallon([3, gallons[1]], new_moves)

    # Simulate a Fill for the 5-gallon
    four_gallon([gallons[0], 5], new_moves)

    # Simulate an Empty gallon for the 3-gallon
    four_gallon([0, gallons[1]], new_moves)

    # Simulate an Empty gallon for the 5-gallon
    four_gallon([gallons[0], 0], new_moves)

def pour_gallons(gallons : List[int], pouringTo: int):
    """
    Description
    -----------
    Given two gallons, we will "pour" or transfer 1 unit of water from one gallon to the
    other gallon and check for every transfer if we have hit the maximum for the other gallon or if we
    have nothing (0) to pour to the other gallon.

    Arguments
    ---------
    gallons: List[int]
        An array containing two gallons

    pouringTo: int
        The index of the array in gallons to where the gallon is being poured to

    Return
    ------
    int[]:
        Updated gallons once pouring is complete.
    """

    # Perform a deep copy to not mess with the gallons parameter
    new_gallons = gallons.copy()

    # Check if pouringTo is an integer and go to while loop to perform "pouring".
    if pouringTo == 0:

        # Continue Pouring until gallon[0] has be filled or gallon[1] is empty.
        while new_gallons[1] != 0 and new_gallons[0] != 3:

            # Remove 1 unit from 5-gallon
            new_gallons[1] -= 1

            # Add 1 unit to 3-gallon
            new_gallons[0] += 1

    elif pouringTo == 1:

        # Continue Pouring until gallon[1] has be filled or gallon[0] is empty.
        while new_gallons[0] != 0 and new_gallons[1] != 5:

            # Add 1 unit to 5-gallon
            new_gallons[1] += 1

            # Remove 1 unit from 3-gallon
            new_gallons[0] -= 1

    # Return the array of the gallon that have been finished pouring.
    return new_gallons

# Call the function inside four_gallon.py and nothing else
if __name__ == "__main__":
    
    # Initialize four_gallon() with empty gallons and no moves ready
    four_gallon([0, 0], [])