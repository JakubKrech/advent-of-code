using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_10
    {
        public static void Solve()
        {
            //var input = InputData.ReadTestInput("Day_10.txt");
            var input = InputData.ReadInput("Day_10.txt");

            Console.WriteLine("=== Day 10 ===");
            Console.WriteLine($"Part 1: {Part1(input)}");
            Console.WriteLine($"Part 2:"); Part2(input);
        }

        private static int Part1(IEnumerable<string> input)
        {
            int x = 1;
            int cycle = 1;
            int sumOfSignalStrengths = 0;

            Func<int, int> GetSignalStrength = cycle => cycle % 40 == 20 ? cycle * x : 0;

            foreach (var line in input)
            {
                var data = line.Split(" ");

                switch (data[0])
                {
                    case "noop":
                        sumOfSignalStrengths += GetSignalStrength(cycle);
                        cycle += 1;
                        break;

                    case "addx":
                        sumOfSignalStrengths += GetSignalStrength(cycle);
                        cycle += 1;
                        sumOfSignalStrengths += GetSignalStrength(cycle);
                        cycle += 1;

                        x += int.Parse(data[1]);
                        break;
                }
            }

            return sumOfSignalStrengths;
        }

        private static void Part2(IEnumerable<string> input)
        {
            int x = 1;
            int cycle = 1;

            Func<int, string> GetOptionalNewline = cycle => cycle % 40 == 0 ? "\n" : "";
            Func<int, string> GetCharToPrint =
                cycle => Math.Abs(((cycle - 1) % 40) - x) <= 1 ? 
                "X" + GetOptionalNewline(cycle) : 
                " " + GetOptionalNewline(cycle);

            foreach (var line in input)
            {
                var data = line.Split(" ");

                switch (data[0])
                {
                    case "noop":
                        Console.Write(GetCharToPrint(cycle));
                        cycle += 1;
                        break;

                    case "addx":
                        Console.Write(GetCharToPrint(cycle));
                        cycle += 1;

                        Console.Write(GetCharToPrint(cycle));
                        cycle += 1;

                        x += int.Parse(data[1]);
                        break;
                }
            }
            Console.WriteLine();
        }
    }
}
