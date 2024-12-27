defmodule FourGallonElixirTest do
  use ExUnit.Case
  doctest FourGallonElixir

  test "greets the world" do
    assert FourGallonElixir.hello() == :world
  end
end
