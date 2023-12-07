input =
  File.read!("../../inputs/day2.txt")
  |> String.replace("\r\n", "\n")

Benchee.run(
  %{
    :solution1 => fn ->
      input
      |> Day02.parser()
      |> Day02.solution1()
    end,
    :solution2 => fn ->
      input
      |> Day02.parser()
      |> Day02.solution2()
    end
  },
  parallel: 6,
  memory_time: 5
)
