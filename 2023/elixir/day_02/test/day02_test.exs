defmodule Day02Test do
  use ExUnit.Case
  doctest Day02

  test "part1" do
    content =
      File.read!("../../inputs/ex2.txt")
      |> String.replace("\r\n", "\n")

    sum =
      content
      |> Day02.parser()
      |> Day02.solution1()

    assert sum == 8
    # IO.inspect(sum)
  end

  test "part2" do
    content =
      File.read!("../../inputs/ex2.txt")
      |> String.replace("\r\n", "\n")

    result =
      content
      |> Day02.parser()
      |> Day02.solution2()

    IO.inspect(result)
  end
end
