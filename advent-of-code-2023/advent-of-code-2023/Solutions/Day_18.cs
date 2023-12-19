using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_18
    {
        public static void Solve()
        {
            AoCTools<Int64>.RunMeasureTimeAndLog(Part1, Part2, "18", testInput: false);
        }

        private static Int64 Part1(IEnumerable<string> input)
        {
            List<string> inputList = input.ToList();

            int max_row = 0;
            int min_row = int.MaxValue;

            int max_col = 0;
            int min_col = int.MaxValue;

            int cur_row = 0;
            int cur_col = 0;

            List<Tuple<char, int, string>> parsedData = new();

            foreach (string line in inputList)
            {
                var data = line.Split(' ');
                char direction = data[0][0];
                int distance = int.Parse(data[1]);
                string color = data[2].Substring(1, data[2].Length - 2);

                parsedData.Add(new Tuple<char, int, string>(direction, distance, color));

                // Calculate min and max value of row and col
                if (direction == 'L')
                {
                    cur_col -= distance;
                }
                else if (direction == 'R')
                {
                    cur_col += distance;
                }
                else if (direction == 'U')
                {
                    cur_row -= distance;
                }
                else if (direction == 'D')
                {
                    cur_row += distance;
                }

                max_row = int.Max(max_row, cur_row);
                min_row = int.Min(min_row, cur_row);

                max_col = int.Max(max_col, cur_col);
                min_col = int.Min(min_col, cur_col);
            }

            int row_count = max_row - min_row + 1;
            int col_count = max_col - min_col + 1;

            int starting_row = Math.Abs(min_row) + 1; // Add 1 for padding
            int starting_col = Math.Abs(min_col) + 1; // Add 1 for padding

            List<List<char>> board = new();
            List<List<int>> djikstra = new();

            #region PrepareBoard
            List<char> paddingLine = new();
            List<int> djikstraPadding = new();

            for (int i = 0; i < col_count + 2; i++)
            {
                djikstraPadding.Add(0);
                paddingLine.Add('.');
            }

            board.Add(paddingLine);
            djikstra.Add(djikstraPadding);
            for (int row = 0; row < row_count; row++)
            {
                List<char> line = new();
                List<int> djikstraLine = new();
                for (int col = -1; col <= col_count; col++) // -1 and <= for padding
                {
                    line.Add('.');
                    djikstraLine.Add(0);
                }
                board.Add(line);
                djikstra.Add(djikstraLine);
            }
            board.Add(paddingLine);
            djikstra.Add(djikstraPadding);

            board[starting_row][starting_col] = 'O';
            #endregion PrepareBoard

            cur_row = starting_row;
            cur_col = starting_col;

            // For each instruction paint the board
            foreach (var instruction in parsedData)
            {
                char direction = instruction.Item1;
                int distance = instruction.Item2;

                if (direction == 'L')
                {
                    while (distance-- > 0)
                    {
                        cur_col--;
                        board[cur_row][cur_col] = '#';
                    }
                }
                else if (direction == 'R')
                {
                    while (distance-- > 0)
                    {
                        cur_col++;
                        board[cur_row][cur_col] = '#';
                    }
                }
                else if (direction == 'U')
                {
                    while (distance-- > 0)
                    {
                        cur_row--;
                        board[cur_row][cur_col] = '#';
                    }
                }
                else if (direction == 'D')
                {
                    while (distance-- > 0)
                    {
                        cur_row++;
                        board[cur_row][cur_col] = '#';
                    }
                }
            }

            // Do the Djikstra passthrough starting in 0,0 - color everything outside of area to different char
            for (int row = 0; row < board.Count; row++)
            {
                for (int col = 0; col < board[0].Count; col++)
                {
                    djikstra[row][col] = board[row][col] == '#' ? 1 : 0;
                }
            }

            HashSet<Tuple<int, int>> queue = new() { new Tuple<int, int>(0, 0) };
            int iter = 0;

            while (queue.Count > 0)
            {
                iter++;
                var curr = queue.First();
                queue.Remove(curr);

                board[curr.Item1][curr.Item2] = '-';
                djikstra[curr.Item1][curr.Item2] = 1;

                // Go Down
                if (curr.Item1 + 1 < board.Count && board[curr.Item1 + 1][curr.Item2] != '#' && djikstra[curr.Item1 + 1][curr.Item2] != 1)
                {
                    queue.Add(new Tuple<int, int>(curr.Item1 + 1, curr.Item2));
                }

                // Go Up
                if (curr.Item1 - 1 >= 0 && board[curr.Item1 - 1][curr.Item2] != '#' && djikstra[curr.Item1 - 1][curr.Item2] != 1)
                {
                    queue.Add(new Tuple<int, int>(curr.Item1 - 1, curr.Item2));
                }

                // Go Right
                if (curr.Item2 + 1 < board[0].Count && board[curr.Item1][curr.Item2 + 1] != '#' && djikstra[curr.Item1][curr.Item2 + 1] != 1)
                {
                    queue.Add(new Tuple<int, int>(curr.Item1, curr.Item2 + 1));
                }

                // Go Left
                if (curr.Item2 - 1 >= 0 && board[curr.Item1][curr.Item2 - 1] != '#' && djikstra[curr.Item1][curr.Item2 - 1] != 1)
                {
                    queue.Add(new Tuple<int, int>(curr.Item1, curr.Item2 - 1));
                }
            }

            // Count and return characters different than '-'
            int total = 0;

            for (int row = 0; row < board.Count; row++)
            {
                for (int col = 0; col < board[0].Count; col++)
                {
                    if (board[row][col] != '-') total++;
                }
            }

            return total;
        }

        private static Int64 Part2(IEnumerable<string> input)
        {
            List<string> inputList = input.ToList();
            List<Tuple<char, int>> parsedData = new();

            // int, int, int - row, start, end (included) | always value of start <= end
            List<Tuple<int, int, int>> horizontals = new();

            // int, int, int - col, start, end (included) | always value of start <= end
            List<Tuple<int, int, int>> verticals = new();

            int cur_row = 0;
            int cur_col = 0;

            // Populate the horizontals and verticals lists
            foreach (string line in inputList)
            {
                var data = line.Split(' ');
                string color = data[2].Substring(1, data[2].Length - 2);

                int distance = int.Parse(color.Substring(1, 5), System.Globalization.NumberStyles.HexNumber);
                char direction = NumberToDirection(color.Substring(6, 1)[0]);

                parsedData.Add(new Tuple<char, int>(direction, distance));

                if (direction == 'L')
                {
                    horizontals.Add(new Tuple<int, int, int>(cur_row, cur_col - distance, cur_col));
                    cur_col -= distance;
                }
                else if (direction == 'R')
                {
                    horizontals.Add(new Tuple<int, int, int>(cur_row, cur_col, cur_col + distance));
                    cur_col += distance;
                }
                else if (direction == 'U')
                {
                    verticals.Add(new Tuple<int, int, int>(cur_col, cur_row - distance, cur_row));
                    cur_row -= distance;
                }
                else if (direction == 'D')
                {
                    verticals.Add(new Tuple<int, int, int>(cur_col, cur_row, cur_row + distance));
                    cur_row += distance;
                }
            }

            return CountByHorizontalsAndVerticals(horizontals, verticals);
        }

        private static char NumberToDirection(char number)
        {
            switch (number)
            {
                case '0': return 'R';
                case '1': return 'D';
                case '2': return 'L';
                default: return 'U'; // case '3'
            }
        }

        private static Int64 CountByHorizontalsAndVerticals(List<Tuple<int, int, int>> horizontals, List<Tuple<int, int, int>> verticals)
        {
            horizontals = horizontals.OrderBy(x => x.Item1).ThenBy(x => x.Item2).ToList();
            verticals.Sort((x, y) => x.Item1.CompareTo(y.Item1));

            int min_row = horizontals[0].Item1;
            int max_row = horizontals[horizontals.Count - 1].Item1;

            Int64 total = 0;

            List<Tuple<int, int, int>> calculatableHorizontals = new();

            foreach (var horizontal in horizontals)
            {
                var left = verticals.Find(vert => vert.Item1 == horizontal.Item2 && (vert.Item2 == horizontal.Item1 || vert.Item3 == horizontal.Item1));
                var right = verticals.Find(vert => vert.Item1 == horizontal.Item3 && (vert.Item2 == horizontal.Item1 || vert.Item3 == horizontal.Item1));

                var verticalsInRow = verticals.Where(x => horizontal.Item1 >= x.Item2 && horizontal.Item1 <= x.Item3).ToList();

                int leftIndex = verticalsInRow.FindIndex(x => x == left);
                int rightIndex = verticalsInRow.FindIndex(x => x == right);

                /*
                The idea used in solution below is that in each row we always have a region not included in the figure, then
                a region that is inside the figure, then again outside of figure etc... How do we know what if we are inside
                or outside of the figure? By analyzing the row and seeing how many vertical lines crosses that row. First vertical
                starts the inside-figure region, second vertical ends it. So %2 operation can be used to determine it. The only issue
                are horizontal lines in that row - we need to handle them separately. Handling is different depending if we are inside
                or outside of the figure when the horizontal line is detected.
                */

                // Process horizontal lines, to get to the point where no horizontal line interfere with easy vertical calculations performed later
                
                // Both attached vertical lines are going up, or both are going down
                if (left.Item3 > horizontal.Item1 && right.Item3 > horizontal.Item1 || left.Item2 < horizontal.Item1 && right.Item2 < horizontal.Item1)
                {
                    if (leftIndex % 2 == 1) // We are inside a figure - do not shorten attached verticals, just count the shortened horizontal line
                    {
                        calculatableHorizontals.Add(new Tuple<int, int, int>(horizontal.Item1, horizontal.Item2 + 1, horizontal.Item3 - 1));
                    }
                    else
                    {
                        calculatableHorizontals.Add(new Tuple<int, int, int>(horizontal.Item1, horizontal.Item2, horizontal.Item3));

                        // verticals attached to horizontal are going down - remove top elements from both of them
                        if (left.Item3 > horizontal.Item1 && right.Item3 > horizontal.Item1)
                        {
                            verticals.Remove(left);
                            verticals.Remove(right);

                            verticals.Add(new Tuple<int, int, int>(left.Item1, left.Item2 + 1, left.Item3));
                            verticals.Add(new Tuple<int, int, int>(right.Item1, right.Item2 + 1, right.Item3));

                            verticals.Sort((x, y) => x.Item1.CompareTo(y.Item1));
                        }
                        else // if verticals are not going down, then they are surely both going up - remove bottom element from both of them
                        {
                            verticals.Remove(left);
                            verticals.Remove(right);

                            verticals.Add(new Tuple<int, int, int>(left.Item1, left.Item2, left.Item3 - 1));
                            verticals.Add(new Tuple<int, int, int>(right.Item1, right.Item2, right.Item3 - 1));

                            verticals.Sort((x, y) => x.Item1.CompareTo(y.Item1));
                        }
                    }
                }
                else // One attached vertical is going up, while the other is going down
                {
                    // We are outside of a figure - remove almost entire horizontal, except the rightmost element (update attached verticals)
                    if (leftIndex % 2 == 0)
                    {
                        total += horizontal.Item3 - horizontal.Item2;

                        verticals.Remove(left);
                        verticalsInRow.Remove(left);

                        // re-add vertical without element from the horizontal height
                        int newStart = left.Item2 == horizontal.Item1 ? left.Item2 + 1 : left.Item2;
                        int newEnd = left.Item3 == horizontal.Item1 ? left.Item3 - 1 : left.Item3;

                        verticals.Add(new Tuple<int, int, int>(left.Item1, newStart, newEnd));
                        verticalsInRow.Add(new Tuple<int, int, int>(left.Item1, newStart, newEnd));

                        verticals.Sort((x, y) => x.Item1.CompareTo(y.Item1));
                        verticalsInRow.Sort((x, y) => x.Item1.CompareTo(y.Item1));
                    }
                    else // We are inside of a figure - remove almost entire horizontal, except the left element (update attached verticals)
                    {
                        total += horizontal.Item3 - horizontal.Item2;

                        verticals.Remove(right);
                        verticalsInRow.Remove(right);

                        // re-add vertical without element from the horizontal height
                        int newStart = right.Item2 == horizontal.Item1 ? right.Item2 + 1 : right.Item2;
                        int newEnd = right.Item3 == horizontal.Item1 ? right.Item3 - 1 : right.Item3;

                        verticals.Add(new Tuple<int, int, int>(right.Item1, newStart, newEnd));
                        verticalsInRow.Add(new Tuple<int, int, int>(right.Item1, newStart, newEnd));

                        verticals.Sort((x, y) => x.Item1.CompareTo(y.Item1));
                        verticalsInRow.Sort((x, y) => x.Item1.CompareTo(y.Item1));
                    }
                }
            }

            // For each row either just cound the side horizontal lengths, or perform easy vertical-only calculations
            for(int row = min_row; row <= max_row; row++)
            {
                int totalForRow = 0;
                var calculatableHorizontalsInRow = calculatableHorizontals.Where(x => x.Item1 == row).ToList();

                if(calculatableHorizontalsInRow.Count > 0)
                {
                    foreach(var horiz in calculatableHorizontalsInRow)
                    {
                        totalForRow += horiz.Item3 - horiz.Item2 + 1;
                    }
                }

                var verticalsInRow = verticals.Where(x => row >= x.Item2 && row <= x.Item3).ToList();

                for (int i = 0; i < verticalsInRow.Count; i += 2)
                {
                    totalForRow += verticalsInRow[i + 1].Item1 - verticalsInRow[i].Item1 + 1;
                }

                total += totalForRow;
            }

            return total;
        }
    }
}
