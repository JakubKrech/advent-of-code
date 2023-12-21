using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_21
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "21", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            var inputList = input.ToList();

            int s_row = -1;
            int s_col = -1;

            List<List<char>> board = new List<List<char>>();
            foreach(var line in inputList.Select((value, row) => new {row, value}))
            {
                List<char> newLine = new List<char>();
                foreach(var c in line.value.Select((value, col) => new {col, value}))
                {
                    newLine.Add(c.value);

                    if(c.value == 'S')
                    {
                        s_row = line.row;
                        s_col = c.col;
                    }
                }
                board.Add(newLine);
            }

            board[s_row][s_col] = '.'; // remove the S

            int row_count = board.Count;
            int col_count = board[0].Count;

            Queue<Tuple<int, int>> currentLocations = new();
            currentLocations.Enqueue(new Tuple<int, int>(s_row, s_col));

            HashSet<Tuple<int, int>> newLocations = new();

            int remainingSteps = 64;

            while(remainingSteps > 0)
            {
                remainingSteps--;

                while (currentLocations.Count > 0)
                {
                    var location = currentLocations.Dequeue();
                    int row = location.Item1;
                    int col = location.Item2;

                    // go up
                    if (row - 1 >= 0 && board[row - 1][col] != '#')
                    {
                        newLocations.Add(new Tuple<int, int>(row - 1, col));
                    }

                    // go down
                    if (row + 1 < row_count && board[row + 1][col] != '#')
                    {
                        newLocations.Add(new Tuple<int, int>(row + 1, col));
                    }

                    // go left
                    if (col - 1 >= 0 && board[row][col - 1] != '#')
                    {
                        newLocations.Add(new Tuple<int, int>(row, col - 1));
                    }

                    // go right
                    if (col + 1 < col_count && board[row][col + 1] != '#')
                    {
                        newLocations.Add(new Tuple<int, int>(row, col + 1));
                    }
                }

                foreach(var loc in newLocations)
                {
                    currentLocations.Enqueue(new Tuple<int, int>(loc.Item1, loc.Item2));
                }

                newLocations.Clear();
            }

            return currentLocations.Count;
        }

        private static int Part2(IEnumerable<string> input)
        {
            return 0;
        }
    }
}
