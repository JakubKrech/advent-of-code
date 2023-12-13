using AdventOfCode2022.Tools;
using System.Text;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_13
    {
        public static List<Tuple<string, int, int>> Part1Solutions = new List<Tuple<string, int, int>>();

        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "13", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            return GetReflectionResult(input, false);
        }

        private static int Part2(IEnumerable<string> input)
        {
            return GetReflectionResult(input, true);
        }

        private static int GetReflectionResult(IEnumerable<string> input, bool part2)
        {
            List<List<string>> horizontalFigures = GetHorizontalLines(input);
            List<List<string>> verticalFigures = GetVerticalLines(input, horizontalFigures);

            int horizontalPointsPerRow = 100;
            int verticalPointsPerColumn = 1;

            int totalResult = 0;

            for (int figIndex = 0; figIndex < horizontalFigures.Count; figIndex++)
            {
                List<string> figureHorizontally = horizontalFigures[figIndex];
                List<string> figureVertically = verticalFigures[figIndex];
                bool mirrorAlreadyFound = false;

                // Look for horizontal mirror
                for (int i = 0; i + 1 < figureHorizontally.Count; i++)
                {
                    bool smudgePossible = part2;

                    if (part2 && Part1Solutions[figIndex].Item1 == "horizontal")
                    {
                        if (Part1Solutions[figIndex].Item2 == i && Part1Solutions[figIndex].Item3 == i + 1)
                        {
                            continue;
                        }
                    }

                    if (AreTwoLinesEqual(figureHorizontally[i], figureHorizontally[i + 1], ref smudgePossible))
                    {
                        Tuple<int, int>? horizontalReflectionIndexes = new Tuple<int, int>(i, i + 1);

                        if (horizontalReflectionIndexes != null)
                        {
                            int up = horizontalReflectionIndexes.Item1 - 1;
                            int down = horizontalReflectionIndexes.Item2 + 1;

                            bool reflectionReachedBorder = true;

                            while (up >= 0 && down < figureHorizontally.Count)
                            {
                                if (!AreTwoLinesEqual(figureHorizontally[up], figureHorizontally[down], ref smudgePossible))
                                {
                                    reflectionReachedBorder = false;
                                    break;
                                }

                                up--;
                                down++;
                            }

                            if (reflectionReachedBorder)
                            {
                                totalResult += horizontalPointsPerRow * (horizontalReflectionIndexes.Item1 + 1);
                                mirrorAlreadyFound = true;

                                if(!part2)
                                {
                                    Part1Solutions.Add(new Tuple<string, int, int>(
                                        "horizontal", horizontalReflectionIndexes.Item1, horizontalReflectionIndexes.Item2));
                                }

                                break;
                            }
                        }
                    }
                }

                if (mirrorAlreadyFound) continue;

                // Look for vertical mirror
                for (int i = 0; i + 1 < figureVertically.Count; i++)
                {
                    bool smudgePossible = part2;

                    if (part2 && Part1Solutions[figIndex].Item1 == "vertical")
                    {
                        if (Part1Solutions[figIndex].Item2 == i && Part1Solutions[figIndex].Item3 == i + 1)
                        {
                            continue;
                        }
                    }

                    if (AreTwoLinesEqual(figureVertically[i], figureVertically[i + 1], ref smudgePossible))
                    {
                        Tuple<int, int>? verticalReflectionIndexes = new Tuple<int, int>(i, i + 1);
                        if (verticalReflectionIndexes != null)
                        {
                            int left = verticalReflectionIndexes.Item1 - 1;
                            int right = verticalReflectionIndexes.Item2 + 1;

                            bool reflectionReachedBorder = true;

                            while (left >= 0 && right < figureVertically.Count)
                            {
                                if (!AreTwoLinesEqual(figureVertically[left], figureVertically[right], ref smudgePossible))
                                {
                                    reflectionReachedBorder = false;
                                    break;
                                }

                                left--;
                                right++;
                            }

                            if (reflectionReachedBorder)
                            {
                                totalResult += verticalPointsPerColumn * (verticalReflectionIndexes.Item1 + 1);

                                if(!part2)
                                {
                                    Part1Solutions.Add(new Tuple<string, int, int>(
                                        "vertical", verticalReflectionIndexes.Item1, verticalReflectionIndexes.Item2));
                                }

                                break;
                            }
                        }
                    }
                }
            }

            return totalResult;
        }

        private static List<List<string>> GetHorizontalLines(IEnumerable<string> input)
        {
            List<List<string>> figures = new();

            List<string> figureLines = new();
            foreach(string line in input)
            {
                if(line != string.Empty)
                {
                    figureLines.Add(line);
                }
                else
                {
                    figures.Add(figureLines);
                    figureLines = new();
                }
            }

            figures.Add(figureLines);

            return figures;
        }

        private static List<List<string>> GetVerticalLines(IEnumerable<string> input, List<List<string>> horizontalFigures)
        {
            List<List<string>> lines = new List<List<string>>();

            foreach (var figure in horizontalFigures)
            {
                List<string> figureLines = new();
                for (int colIndex = 0; colIndex < figure[0].Length; colIndex++)
                {
                    StringBuilder column = new();

                    foreach(var figureRow in figure)
                    {
                        column.Append(figureRow[colIndex]);
                    }

                    figureLines.Add(column.ToString());
                }

                lines.Add(figureLines);
            }

            return lines;
        }

        private static bool AreTwoLinesEqual(string a, string b, ref bool smudgePossible)
        {
            int differences = 0;

            for (int index = 0; index < a.Length; index++)
            {
                if (a[index] != b[index]) differences++;
            }

            if (differences > 0)
            {
                if (differences == 1 && smudgePossible == true) return true;
                else return false;
            }

            return true;
        }
    }
}
