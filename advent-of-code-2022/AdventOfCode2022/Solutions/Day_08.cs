using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_08
    {
        public static void Solve()
        {
            // var input = InputData.ReadTestInput("Day_08.txt");
            var input = InputData.ReadInput("Day_08.txt");

            var (part1, part2) = Solution(input);

            Console.WriteLine("=== Day 08 ===");
            Console.WriteLine($"Part 1: {part1}");
            Console.WriteLine($"Part 2: {part2}\n");
        }

        private static (int, int) Solution(IEnumerable<string> input)
        {
            int lineAmmount = input.Count();
            int lineLength = input.First().Length;

            int[,] trees2dList = new int[lineAmmount, lineLength];
            int totalVisibleTrees = lineAmmount * 2 + lineLength * 2 - 4; // outline minus doubled corners
            int maxScenicScore = 0;

            for (int i = 0; i < lineAmmount; i++)
            {
                for (int j = 0; j < lineLength; j++)
                {
                    trees2dList[i, j] =int.Parse(input.ElementAt(i)[j].ToString());
                }
            }

            for (int i = 1; i < lineAmmount - 1; i++)
            {
                for (int j = 1; j < lineLength - 1; j++)
                {
                    int current = trees2dList[i, j];
                    List<bool> hidden = new() { false, false, false, false };
                    List<int> scenicScores = new() { 0, 0, 0, 0 };

                    // check left side
                    for (int x = j - 1; x >= 0; x--)
                    {
                        scenicScores[(int)Directions.Left] += 1;
                        if (current <= trees2dList[i, x])
                        {
                            hidden[(int)Directions.Left] = true;
                            break;
                        }
                    }

                    // check right side
                    for (int x = j + 1; x < lineLength; x++)
                    {
                        scenicScores[(int)Directions.Right] += 1;
                        if (current <= trees2dList[i, x])
                        {
                            hidden[(int)Directions.Right] = true;
                            break;
                        }
                    }

                    // check top side
                    for (int x = i - 1; x >= 0; x--)
                    {
                        scenicScores[(int)Directions.Top] += 1;
                        if (current <= trees2dList[x, j])
                        {
                            hidden[(int)Directions.Top] = true;
                            break;
                        }
                    }

                    // check bottom side
                    for (int x = i + 1; x < lineAmmount; x++)
                    {
                        scenicScores[(int)Directions.Bottom] += 1;
                        if (current <= trees2dList[x, j])
                        {
                            hidden[(int)Directions.Bottom] = true;
                            break;
                        }
                    }

                    if (hidden.Contains(false))
                    {
                        totalVisibleTrees += 1;
                    }

                    maxScenicScore = int.Max(maxScenicScore, scenicScores.Aggregate((x, y) => x * y));
                }
            }

            return (totalVisibleTrees, maxScenicScore);
        }

        enum Directions
        {
            Left,
            Right,
            Top,
            Bottom
        }
    }
}