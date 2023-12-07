defmodule Day01Test do
  use ExUnit.Case
  doctest Day01

  test "part_1" do
    str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
    Day01.parser_1(str)
  end
end
