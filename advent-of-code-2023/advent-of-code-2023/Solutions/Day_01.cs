using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_01
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "01", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            int sum = 0;

            foreach (string line in input)
            {
                List<int> digits = new();

                foreach (char c in line) { if (char.IsDigit(c)) digits.Add(int.Parse(c.ToString())); }

                sum += 10 * digits[0] + digits[^1];
            }

            return sum;
        }

        private static int Part2(IEnumerable<string> input)
        {
            int sum = 0;

            foreach (string line in input)
            {
                Dictionary<int, int> indexes = new();
                FindNumericDigitIndexes(ref indexes, line);
                FindSpelledDigitIndexes(ref indexes, line);

                sum += 10 * indexes[indexes.Keys.Min()] + indexes[indexes.Keys.Max()];
            }

            return sum;
        }

        private static void FindNumericDigitIndexes(ref Dictionary<int, int> indexes, string line)
        {
            for (int i = 0; i < line.Length; i++)
            {
                if (char.IsDigit(line[i])) indexes[i] = int.Parse(line[i].ToString());
            }
        }

        private static void FindSpelledDigitIndexes(ref Dictionary<int, int> indexes, string line)
        {
            Dictionary<string, int> validDigits = new()
            {
                ["one"] = 1,
                ["two"] = 2,
                ["three"] = 3,
                ["four"] = 4,
                ["five"] = 5,
                ["six"] = 6,
                ["seven"] = 7,
                ["eight"] = 8,
                ["nine"] = 9
            };

            foreach (string valid in validDigits.Keys)
            {
                int index = line.IndexOf(valid);

                while (index != -1)
                {
                    indexes[index] = validDigits[valid];
                    index = line.IndexOf(valid, index + 1);
                }
            }
        }
    }
}
