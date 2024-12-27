defmodule FourGallonElixir do

  @doc """
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
        A list of two gallons with the units of water acting as integers.

    moves: List[List[int]]
        A list of a list of integers containing the gallons for.
    """
  def four_gallon(gallons, moves) do
    [first_gallon, second_gallon] = gallons
    new_moves = moves
    new_moves = new_moves ++ [gallons]
    cond do
      (first_gallon == 4 or second_gallon == 4) and Kernel.length(new_moves) < 15 ->
        IO.puts("A legal path in #{Kernel.length(new_moves)} moves is: ")
        new_moves |> Enum.map(fn pair -> "[#{Enum.join(pair, ", ")}]" end) |> Enum.join(" -> ") |> IO.puts
      gallons in moves or Kernel.length(new_moves) >= 15 ->
        nil
      true ->
        #Simulate a Pour
        four_gallon(pour_gallons(gallons, 1), new_moves)
        four_gallon(pour_gallons(gallons, 0), new_moves)

        #Simulate a Fill
        four_gallon(List.replace_at(gallons, 0, 3), new_moves)
        four_gallon(List.replace_at(gallons, 1, 5), new_moves)

        #Simulate an Empty
        four_gallon(List.replace_at(gallons, 0, 0), new_moves)
        four_gallon(List.replace_at(gallons, 1, 0), new_moves)
    end
  end

  @doc """
    Description
    -----------
    Given two gallons, we will "pour" or transfer 1 unit of water from one gallon to the
    other gallon and check for every transfer if we have hit the maximum for the other gallon or if we
    have nothing (0) to pour to the other gallon.

    Arguments
    ---------
    gallons: List[int]
        An array containing two gallons.

    pouringTo: int
        The index of the array in gallons to where the gallon is being poured to.

    Return
    ------
    int[]:
        Updated gallons once pouring is complete.
  """
  def pour_gallons(gallons, pouringTo) do

    # Perform recusion to pour our gallons and return
    case {gallons, pouringTo} do
      # The first four are base cases that will return gallons
      {[_, 0], 0} -> gallons
      {[3, _], 0} -> gallons
      {[0, _], 1} -> gallons
      {[_, 5], 1} -> gallons

      # The other two are recursive cases that will pour to the other gallon and call the function again until its reaches a base case
      {[first_gal, second_gal], 0} -> pour_gallons([first_gal + 1, second_gal - 1], 0)
      {[first_gal, second_gal], 1} -> pour_gallons([first_gal - 1, second_gal + 1], 1)

      # Any other case is an error found
      _ -> IO.puts(:standard_error, "pour_gallons pouringTo is not a 0 or 1")
    end
  end

end

FourGallonElixir.four_gallon([0,0], [])
