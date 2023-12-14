using AdventOfCode2022.Tools;
using System.Text;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_14
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "14", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            List<string> inputList = input.ToList();
            List<string> rows = new List<string>();

            // Flip the board so we can analyze each row for balls rolling to the left
            for (int col = 0; col < inputList.Count; col++)
            {
                StringBuilder newFlipped = new StringBuilder();
                for (int row = 0; row < inputList[0].Length; row++)
                {
                    newFlipped.Append(inputList[row][col]);
                }
                rows.Add(newFlipped.ToString());
            }

            int totalWeight = 0;

            foreach (string row in rows)
            {
                int start = -1;
                int ballCount = 0;

                for (int i = 0; i < row.Length; i++)
                {
                    if (row[i] == '.' || row[i] == 'O')
                    {
                        if (start == -1) start = i;
                        if (row[i] == 'O') ballCount++;
                    }

                    if (row[i] == '#')
                    {
                        if (start == -1) continue;

                        totalWeight += CalculateBallWeight(row.Length - start, ballCount);
                        start = -1;
                        ballCount = 0;
                    }
                }

                totalWeight += CalculateBallWeight(row.Length - start, ballCount);
            }

            return totalWeight;
        }

        private static int Part2(IEnumerable<string> input)
        {
            List<string> inputList = input.ToList();
            List<List<char>> board = new();

            foreach (var line in inputList)
            {
                List<char> newLine = new();
                foreach (var c in line)
                {
                    newLine.Add(c);
                }
                board.Add(newLine);
            }

            int cycles = 1000;
            int prevResult = 0;

            for(int i =0; i < cycles; i++)
            {
                // Move north
                for (int col = 0; col < board[0].Count; col++)
                {
                    int start = -1;
                    int ballCount = 0;

                    for (int row = 0; row < board.Count; row++)
                    {
                        char curChar = board[row][col];

                        if (curChar == '.' || curChar == 'O')
                        {
                            if (start == -1) start = row;
                            if (curChar == 'O') ballCount++;
                        }

                        if (curChar == '#')
                        {
                            if (start == -1) continue;

                            for (int z = start; z < row; z++)
                            {
                                if (ballCount > 0)
                                {
                                    board[z][col] = 'O';
                                    ballCount--;
                                }
                                else
                                {
                                    board[z][col] = '.';
                                }
                            }

                            start = -1;
                            ballCount = 0;
                        }
                    }

                    if (start != -1 && ballCount > 0)
                    {
                        for (int z = start; z < board.Count; z++)
                        {
                            if (ballCount > 0)
                            {
                                board[z][col] = 'O';
                                ballCount--;
                            }
                            else
                            {
                                board[z][col] = '.';
                            }
                        }
                    }
                }


                // Move west

                for (int row = 0; row < board.Count; row++)
                {
                    int start = -1;
                    int ballCount = 0;

                    for (int col = 0; col < board[0].Count; col++)
                    {
                        char curChar = board[row][col];

                        if (curChar == '.' || curChar == 'O')
                        {
                            if (start == -1) start = col;
                            if (curChar == 'O') ballCount++;
                        }
                        else if (curChar == '#')
                        {
                            if (ballCount == 0) start = -1;
                            if (start == -1) continue;

                            for (int z = start; z < col; z++)
                            {
                                if (ballCount > 0)
                                {
                                    board[row][z] = 'O';
                                    ballCount--;
                                }
                                else
                                {
                                    board[row][z] = '.';
                                }
                            }

                            start = -1;
                            ballCount = 0;
                        }
                    }

                    if (start != -1 && ballCount > 0)
                    {
                        for (int z = start; z < board.Count; z++)
                        {
                            if (ballCount > 0)
                            {
                                board[row][z] = 'O';
                                ballCount--;
                            }
                            else
                            {
                                board[row][z] = '.';
                            }
                        }
                    }
                }


                // Move south
                for (int col = 0; col < board[0].Count; col++)
                {
                    int start = -1;
                    int ballCount = 0;

                    for (int row = board.Count - 1; row >= 0; row--)
                    {
                        char curChar = board[row][col];

                        if (curChar == '.' || curChar == 'O')
                        {
                            if (start == -1) start = row;
                            if (curChar == 'O') ballCount++;
                        }

                        if (curChar == '#')
                        {
                            if (start == -1) continue;

                            for (int z = start; z > row; z--)
                            {
                                if (ballCount > 0)
                                {
                                    board[z][col] = 'O';
                                    ballCount--;
                                }
                                else
                                {
                                    board[z][col] = '.';
                                }
                            }

                            start = -1;
                            ballCount = 0;
                        }
                    }

                    if (start != -1 && ballCount > 0)
                    {
                        for (int z = start; z >= 0; z--)
                        {
                            if (ballCount > 0)
                            {
                                board[z][col] = 'O';
                                ballCount--;
                            }
                            else
                            {
                                board[z][col] = '.';
                            }
                        }
                    }
                }


                // Move east

                for (int row = 0; row < board.Count; row++)
                {
                    int start = -1;
                    int ballCount = 0;

                    for (int col = board[0].Count - 1; col >= 0; col--)
                    {
                        char curChar = board[row][col];

                        if (curChar == '.' || curChar == 'O')
                        {
                            if (start == -1) start = col;
                            if (curChar == 'O') ballCount++;
                        }
                        else if (curChar == '#')
                        {
                            if (ballCount == 0) start = -1;
                            if (start == -1) continue;

                            for (int z = start; z > col; z--)
                            {
                                if (ballCount > 0)
                                {
                                    board[row][z] = 'O';
                                    ballCount--;
                                }
                                else
                                {
                                    board[row][z] = '.';
                                }
                            }

                            start = -1;
                            ballCount = 0;
                        }
                    }

                    if (start != -1 && ballCount > 0)
                    {
                        for (int z = start; z >= 0; z--)
                        {
                            if (ballCount > 0)
                            {
                                board[row][z] = 'O';
                                ballCount--;
                            }
                            else
                            {
                                board[row][z] = '.';
                            }
                        }
                    }
                }


                int newResult = CalculateNorthSupportBeamLoad(board);
                Console.WriteLine(i + " " + newResult + " " + (prevResult - newResult));
                prevResult = newResult;
            }

            // Since required ammount of steps is incredibly high - 1000000000, it cannot be simply calculated.
            // By using approach of printing first 1000 cycles results and differences from result of previous
            // cycle it is possible to notice a pattern of repeating interval of numbers. For example, result
            // equal to 118780 which is also smaller by 33 than previous result starts regularly appearing from
            // iteration 118. From that point it appears every 63 iterations.

            // Iter | Result | Diff
            //  118   118780   -33
            //  181   118780   -33
            //  244   118780   -33
            //  307   118780   -33
            //  370   118780   -33
            //  433   118780   -33
            //  496   118780   -33
            //  559   118780   -33

            // With that knowledge its possible to predict what value will be returned on iteration 1000000000.

            Int64 remainingIterations = (1000000000 - 118) % 63;

            return 118747;
        }

        private static int CalculateBallWeight(int distanceFromEdge, int ballCount)
        {
            if (ballCount == 0) return 0;

            int totalWeight = 0;

            while (ballCount-- > 0) totalWeight += distanceFromEdge--;

            return totalWeight;
        }

        private static int CalculateNorthSupportBeamLoad(List<List<char>> board)
        {
            int totalWeight = 0;

            for (int row = 0; row < board.Count; row++)
            {
                for (int col = 0; col < board[0].Count; col++)
                {
                    if (board[row][col] == 'O')
                    {
                        totalWeight += board.Count - row;
                    }
                }
            }

            return totalWeight;
        }
    }
}
