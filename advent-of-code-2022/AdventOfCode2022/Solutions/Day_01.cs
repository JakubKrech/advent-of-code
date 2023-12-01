using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_01
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "01", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            int currSum = 0;
            int max = 0;

            foreach (var line in input)
            {
                if (line == string.Empty)
                {
                    if (currSum > max) max = currSum;
                    currSum = 0;
                }
                else
                {
                    currSum += int.Parse(line);
                }
            }

            return max;
        }

        private static int Part2(IEnumerable<string> input)
        {
            input = input.Append("");

            int currSum = 0;
            int max1 = 0;
            int max2 = 0;
            int max3 = 0;

            foreach (var line in input)
            {
                if (line == string.Empty)
                {
                    if (currSum > max1)
                    {
                        max3 = max2;
                        max2 = max1;
                        max1 = currSum;
                    }
                    else if (currSum > max2)
                    {
                        max3 = max2;
                        max2 = currSum;
                    }
                    else if (currSum > max3)
                    {
                        max3 = currSum;
                    }

                    currSum = 0;
                }
                else
                {
                    currSum += int.Parse(line);
                }
            }

            return max1 + max2 + max3;
        }
    }
}
