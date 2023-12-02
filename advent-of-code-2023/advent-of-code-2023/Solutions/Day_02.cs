using AdventOfCode2022.Tools;

using System.Text.RegularExpressions;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_02
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "02", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            // Bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes
            int max_red = 12;
            int max_green = 13;
            int max_blue = 14;

            int sumOfCorrectGameIds = 0;

            string pattern = @"(\d+)\s*([a-zA-Z]+)";
            int gameId = 1;

            foreach (string line in input)
            {
                MatchCollection matches = Regex.Matches(line, pattern);
                bool errorDetected = false;

                foreach (Match match in matches)
                {
                    var values = match.Value.Split(' ');
                    string color = values[1];
                    int number = int.Parse(values[0]);

                    switch (color)
                    {
                        case "blue":
                            if (number > max_blue) errorDetected = true;
                            break;
                        case "green":
                            if (number > max_green) errorDetected = true;
                            break;
                        case "red":
                            if (number > max_red) errorDetected = true;
                            break;
                    }
                }

                if (!errorDetected) sumOfCorrectGameIds += gameId;

                gameId++;
            }

            return sumOfCorrectGameIds;
        }

        private static int Part2(IEnumerable<string> input)
        {
            int totalPower = 0;
            string pattern = @"(\d+)\s*([a-zA-Z]+)";

            foreach (string line in input)
            {
                MatchCollection matches = Regex.Matches(line, pattern);

                int min_red = 0;
                int min_green = 0;
                int min_blue = 0;

                foreach (Match match in matches)
                {
                    var values = match.Value.Split(' ');
                    string color = values[1];
                    int number = int.Parse(values[0]);

                    switch (color)
                    {
                        case "blue":
                            if (number > min_blue) min_blue = number;
                            break;
                        case "green":
                            if (number > min_green) min_green = number;
                            break;
                        case "red":
                            if (number > min_red) min_red = number;
                            break;
                    }
                }

                int power = min_red * min_green * min_blue;
                totalPower += power;
            }

            return totalPower;
        }
    }
}
