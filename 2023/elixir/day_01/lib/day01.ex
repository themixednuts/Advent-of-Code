defmodule Day01 do
  @moduledoc """
  Documentation for `Day01`.
  """

  @doc """
  Hello world.

  """
  def parser_1(str) do
    sum =
      str
      |> String.split("\n")
      |> Enum.map(&String.trim/1)
      |> Enum.map(&parse_line/1)

    IO.puts(sum)
    sum
  end

  def parse_line(line) do
    chars = String.graphemes(line)

    numbers =
      Enum.reduce(chars, [], fn char, acc ->
        case Integer.parse(char) do
          {number, _} -> acc ++ [number]
          :error -> acc
        end
      end)

    first_number = List.first(numbers)
    last_number = List.last(numbers)

    number =
      ([first_number] ++ [last_number])
      |> Enum.join("")
      |> String.to_integer(10)

    IO.puts("#{line}: #{number}")
  end
end
