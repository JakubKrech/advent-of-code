using AdventOfCode2022.Tools;

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

namespace advent_of_code_2023.Solutions
{
    internal static class Day_10
    {
        private static List<List<char>> pathOnlyMap = new();

        // All available characters: '-', '|', 'F', '7', 'J', 'L'

        // Lists of characters that can be moved into when trying to move in given direction
        private static readonly List<char> rightMoves = new() { '-', 'J', '7' };
        private static readonly List<char> leftMoves = new() { '-', 'L', 'F' };
        private static readonly List<char> downMoves = new() { '|', 'J', 'L' };
        private static readonly List<char> upMoves = new() { '|', 'F', '7' };

        // Lists of characters that can be counted as blocker in either horizontal or vertical direction
        private static readonly List<char> verticals = new() { '|', 'F', '7', 'J', 'L' };
        private static readonly List<char> horizontals = new() { '-', 'F', '7', 'J', 'L' };

        // Lists of character combinations that block in-between pipe movement for given direction
        private static readonly List<string> topClosers = new() { "--", "F-", "-7", "L-", "-J", "LJ", "F7", "FJ", "L7" };
        private static readonly List<string> bottomClosers = new() { "--", "F-", "-7", "L-", "-J", "LJ", "F7", "FJ", "L7" };
        private static readonly List<string> leftClosers = new() { "||", "|F", "L|", "LF", "J7", "J|", "|7", "JF", "L7" };
        private static readonly List<string> rightClosers = new() { "||", "F|", "|L", "FL", "7J", "|J", "7|", "FJ", "7L" };

        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "10", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            List<string> inputList = input.ToList();
            List<List<char>> map = new();

            int s_row = -1, s_col = -1;

            for (int row = 0; row < inputList.Count(); row++)
            {
                List<char> line = new();
                List<char> pathOnlyline = new();

                for (int col = 0; col < inputList.First().Length; col++)
                {
                    line.Add(inputList[row][col]);
                    pathOnlyline.Add('.');

                    if (inputList[row][col] == 'S')
                    {
                        s_row = row; s_col = col;
                    }
                }

                map.Add(line);
                pathOnlyMap.Add(pathOnlyline);
            }

            pathOnlyMap[s_row][s_col] = 'S';

            int curr_row = s_row;
            int curr_col = s_col;

            // Find initial move - right
            if(curr_col + 1 < pathOnlyMap[0].Count && rightMoves.Contains(map[curr_row][curr_col + 1]))
            {
                curr_col++;
            }
            else if (curr_col - 1 >= 0 && leftMoves.Contains(map[curr_row][curr_col - 1]))
            {
                curr_col--;
            }
            else if (curr_row + 1 < pathOnlyMap.Count && downMoves.Contains(map[curr_row + 1][curr_col]))
            {
                curr_row++;
            }
            else if (curr_row - 1 >= 0 && upMoves.Contains(map[curr_row - 1][curr_col]))
            {
                curr_row--;
            }

            char curr = map[curr_row][curr_col];
            pathOnlyMap[curr_row][curr_col] = map[curr_row][curr_col];
            //PrintMap(map);

            int length = 1;

            // Traverse the path until coming back to initial 'S' location
            while (curr != 'S')
            {
                if (curr == '-')
                {
                    if (curr_col - 1 >= 0 && pathOnlyMap[curr_row][curr_col - 1] == '.')
                    {
                        if (TryMove(-1, 0, map[curr_row][curr_col - 1])) curr_col--;
                    }
                    else if (curr_col + 1 < map[0].Count && pathOnlyMap[curr_row][curr_col + 1] == '.')
                    {
                        if (TryMove(1, 0, map[curr_row][curr_col + 1])) curr_col++;
                    }
                    else
                    {
                        break;
                    }
                }
                else if (curr == '|')
                {
                    if (curr_row - 1 >= 0 && pathOnlyMap[curr_row - 1][curr_col] == '.')
                    {
                        if (TryMove(0, -1, map[curr_row - 1][curr_col])) curr_row--;
                    }
                    else if (curr_row + 1 < map[0].Count && pathOnlyMap[curr_row + 1][curr_col] == '.')
                    {
                        if (TryMove(0, 1, map[curr_row + 1][curr_col])) curr_row++;
                    }
                    else
                    {
                        break;
                    }
                }
                else if (curr == 'L')
                {
                    if (curr_row - 1 >= 0 && pathOnlyMap[curr_row - 1][curr_col] == '.')
                    {
                        if (TryMove(0, -1, map[curr_row - 1][curr_col])) curr_row--;
                    }
                    else if (curr_col + 1 < map[0].Count && pathOnlyMap[curr_row][curr_col + 1] == '.')
                    {
                        if (TryMove(1, 0, map[curr_row][curr_col + 1])) curr_col++;
                    }
                    else
                    {
                        break;
                    }
                }
                else if (curr == 'J')
                {
                    if (curr_row - 1 >= 0 && pathOnlyMap[curr_row - 1][curr_col] == '.')
                    {
                        if (TryMove(0, -1, map[curr_row - 1][curr_col])) curr_row--;
                    }
                    else if (curr_col - 1 >= 0 && pathOnlyMap[curr_row][curr_col - 1] == '.')
                    {
                        if (TryMove(-1, 0, map[curr_row][curr_col - 1])) curr_col--;
                    }
                    else
                    {
                        break;
                    }
                }
                else if (curr == '7')
                {
                    if (curr_row + 1 < map[0].Count && pathOnlyMap[curr_row + 1][curr_col] == '.')
                    {
                        if (TryMove(0, 1, map[curr_row + 1][curr_col])) curr_row++;
                    }
                    else if (curr_col - 1 >= 0 && pathOnlyMap[curr_row][curr_col - 1] == '.')
                    {
                        if (TryMove(-1, 0, map[curr_row][curr_col - 1])) curr_col--;
                    }
                    else
                    {
                        break;
                    }
                }
                else if (curr == 'F')
                {
                    if (curr_row + 1 < map[0].Count && pathOnlyMap[curr_row + 1][curr_col] == '.')
                    {
                        if (TryMove(0, 1, map[curr_row + 1][curr_col])) curr_row++;
                    }
                    else if (curr_col + 1 < map[0].Count && pathOnlyMap[curr_row][curr_col + 1] == '.')
                    {
                        if (TryMove(1, 0, map[curr_row][curr_col + 1])) curr_col++;
                    }
                    else
                    {
                        break;
                    }
                }

                curr = map[curr_row][curr_col];
                pathOnlyMap[curr_row][curr_col] = map[curr_row][curr_col];
                //PrintMap(pathOnlyMap);
                length++;
            }

            //PrintMap(pathOnlyMap);

            return 1 + length / 2;
        }

        private static int Part2(IEnumerable<string> input)
        {
            // Replace S with its real pipe character
            ReplaceStartingPoint();

            // Run Djikstra search for removal of outer dots
            dotsToBeRemoved.Add(new Tuple<int, int>(0, 0));

            while(dotsToBeRemoved.Count > 0)
            {
                var x = dotsToBeRemoved.Last();
                RemoveDots(x.Item1, x.Item2);
                dotsToBeRemoved.Remove(x);
            }
            
            //PrintMap(pathOnlyMap);
            int remainingDots = CountRemainingDots();

            while(true)
            {
                StartPipeTraversal();
                int newRemainingDots = CountRemainingDots();

                if (newRemainingDots == remainingDots) break;
                else remainingDots = newRemainingDots;
            }
            //PrintMap(pathOnlyMap);

            return remainingDots;
        }

        

        private static bool TryMove(int col_dir, int row_dir, char destinationChar)
        {
            if(col_dir == 1) // go right
            {
                if(rightMoves.Contains(destinationChar)) return true;
            }
            else if (col_dir == -1) // go left
            {
                if (leftMoves.Contains(destinationChar)) return true;
            }
            else if (row_dir == 1) // go down
            {
                if (downMoves.Contains(destinationChar)) return true;
            }
            else if (row_dir == -1) // go up
            {
                if (upMoves.Contains(destinationChar)) return true;
            }

            return false;
        }

        private static void PrintMap<T>(List<List<T>> map)
        {
            Console.WriteLine();
            for(int row = 0; row < map.Count; row++)
            {
                Console.Write(row % 10 + ":");
                for (int col = 0; col < map[0].Count; col++)
                {
                    Console.Write(map[row][col]);
                }
                Console.WriteLine();
            }
        }

        private static HashSet<Tuple<int, int>> dotsToBeRemoved = new HashSet<Tuple<int, int>>();

        private static void RemoveDots(int row, int col, bool runPipeTraversal = true)
        {
            if (pathOnlyMap[row][col] == '.') pathOnlyMap[row][col] = ' ';

            if(row - 1 >= 0 && pathOnlyMap[row - 1][col] == '.') dotsToBeRemoved.Add(new Tuple<int, int>(row - 1, col));
            if(col - 1 >= 0 && pathOnlyMap[row][col - 1] == '.') dotsToBeRemoved.Add(new Tuple<int, int>(row, col - 1));
            if(row + 1 < pathOnlyMap.Count && pathOnlyMap[row + 1][col] == '.') dotsToBeRemoved.Add(new Tuple<int, int>(row + 1, col));
            if(col + 1 < pathOnlyMap[0].Count && pathOnlyMap[row][col + 1] == '.') dotsToBeRemoved.Add(new Tuple<int, int>(row, col + 1));

            // top - left
            if(row - 1 >= 0 && col - 1 >= 0 && pathOnlyMap[row - 1][col - 1] == '.') dotsToBeRemoved.Add(new Tuple<int, int>(row - 1, col - 1));

            // top - right
            if (row - 1 >= 0 && col + 1 < pathOnlyMap[0].Count && pathOnlyMap[row - 1][col + 1] == '.') dotsToBeRemoved.Add(new Tuple<int, int>(row - 1, col + 1));

            // bottom - left
            if (row + 1 < pathOnlyMap.Count && col - 1 >= 0 && pathOnlyMap[row + 1][col - 1] == '.') dotsToBeRemoved.Add(new Tuple<int, int>(row + 1, col - 1));

            // bottom - right
            if (row + 1 < pathOnlyMap.Count && col + 1 < pathOnlyMap[0].Count && pathOnlyMap[row + 1][col + 1] == '.') dotsToBeRemoved.Add(new Tuple<int, int>(row + 1, col + 1));
        }

        private static void StartPipeTraversal()
        {
            for(int row = 0; row < pathOnlyMap.Count; row++)
            {
                for(int col = 0; col < pathOnlyMap[0].Count; col++)
                {
                    if (pathOnlyMap[row][col] != '.') continue;

                    if (TraverseThePipe(row - 1, col - 1, row - 1, col, PipeDirections.Up) ||
                        TraverseThePipe(row - 1, col, row - 1, col + 1, PipeDirections.Up) ||
                        TraverseThePipe(row, col - 1, row - 1, col - 1, PipeDirections.Left) ||
                        TraverseThePipe(row + 1, col - 1, row, col - 1, PipeDirections.Left) ||
                        TraverseThePipe(row - 1, col + 1, row, col + 1, PipeDirections.Right) ||
                        TraverseThePipe(row, col + 1, row + 1, col + 1, PipeDirections.Right) ||
                        TraverseThePipe(row + 1, col - 1, row + 1, col, PipeDirections.Down) ||
                        TraverseThePipe(row + 1, col, row + 1, col + 1, PipeDirections.Down))
                    {
                        dotsToBeRemoved.Add(new Tuple<int, int>(row, col));
                    }

                    while (dotsToBeRemoved.Count > 0)
                    {
                        var x = dotsToBeRemoved.Last();
                        RemoveDots(x.Item1, x.Item2);
                        dotsToBeRemoved.Remove(x);
                    }
                }
            }
        }

        private static int CountRemainingDots()
        {
            int count = 0;

            foreach(var row in pathOnlyMap)
            {
                foreach(var elem in row)
                {
                    if (elem == '.') count++;
                }
            }

            return count;
        }

        private static void ReplaceStartingPoint()
        {
            int s_row = -1, s_col = -1;

            for(int row = 0; row < pathOnlyMap.Count; row++)
            {
                for (int col = 0; col < pathOnlyMap[0].Count; col++)
                {
                    if (pathOnlyMap[row][col] == 'S')
                    {
                        s_row = row;
                        s_col = col;
                    }
                }
            }

            char leftNeighbor = pathOnlyMap[s_row][s_col - 1];
            char rightNeighbor = pathOnlyMap[s_row][s_col + 1];
            char lowerNeighbor = pathOnlyMap[s_row + 1][s_col];
            char upperNeighbor = pathOnlyMap[s_row - 1][s_col];

            char newSChar = 'X';

            // '-'
            if (horizontals.Contains(leftNeighbor) && horizontals.Contains(rightNeighbor)) newSChar = '-';

            // '|'
            else if (verticals.Contains(leftNeighbor) && verticals.Contains(rightNeighbor)) newSChar = '|';

            // 'F'
            else if (verticals.Contains(lowerNeighbor) && horizontals.Contains(rightNeighbor)) newSChar = 'F';

            // '7'
            else if (horizontals.Contains(leftNeighbor) && verticals.Contains(lowerNeighbor)) newSChar = '7';

            // 'J'
            else if (horizontals.Contains(leftNeighbor) && verticals.Contains(upperNeighbor)) newSChar = 'J';

            // 'L'
            else if (horizontals.Contains(rightNeighbor) && verticals.Contains(upperNeighbor)) newSChar = 'L';

            pathOnlyMap[s_row][s_col] = newSChar;
        }

        enum PipeDirections
        {
            Up,
            Down,
            Left,
            Right
        }

        private static bool TraverseThePipe(int row_1, int col_1, int row_2, int col_2, PipeDirections direction)
        {
            if (row_1 - 1 < 0 || row_2 - 1 < 0 || row_1 + 1 >= pathOnlyMap.Count || row_2 + 1 >= pathOnlyMap.Count ||
                col_1 - 1 < 0 || col_2 - 1 < 0 || col_1 >= pathOnlyMap[0].Count || col_2 >= pathOnlyMap[0].Count) return false;

            char ch_1 = pathOnlyMap[row_1][col_1];
            char ch_2 = pathOnlyMap[row_2][col_2];

            string mergedChars = new string("" + ch_1 + ch_2);

            if (ch_1 == '.' && ch_2 == '.') return false;
            if (ch_1 == ' ' || ch_2 == ' ') return true;

            if (direction == PipeDirections.Up && topClosers.Contains(mergedChars)) return false;
            else if (direction == PipeDirections.Down && bottomClosers.Contains(mergedChars)) return false;
            else if (direction == PipeDirections.Left && leftClosers.Contains(mergedChars)) return false;
            else if (direction == PipeDirections.Right && rightClosers.Contains(mergedChars)) return false;

            if (direction == PipeDirections.Up)
            {
                bool rightSuccess = false;
                bool leftSuccess = false;

                // Turn to the right
                if (ch_2 == 'F') rightSuccess = TraverseThePipe(row_2 - 1, col_2, row_2, col_2, PipeDirections.Right);

                // Turn to the left
                if (ch_1 == '7') leftSuccess = TraverseThePipe(row_1, col_1, row_1 - 1, col_1, PipeDirections.Left);

                if (leftSuccess || rightSuccess) return true;

                // Try straight up
                if (verticals.Contains(ch_1) && verticals.Contains(ch_2))
                {
                    return TraverseThePipe(row_1 - 1, col_1, row_2 - 1, col_2, PipeDirections.Up);
                }
                else return false;
            }
            else if (direction == PipeDirections.Down)
            {
                bool rightSuccess = false;
                bool leftSuccess = false;

                // Turn to the right
                if (ch_2 == 'L') rightSuccess = TraverseThePipe(row_2, col_2, row_2 + 1, col_2, PipeDirections.Right);

                // Turn to the left
                if (ch_1 == 'J') leftSuccess = TraverseThePipe(row_1 + 1, col_1, row_1, col_1, PipeDirections.Left);

                if (leftSuccess || rightSuccess) return true;

                // Try straight down
                if (verticals.Contains(ch_1) && verticals.Contains(ch_2))
                {
                    return TraverseThePipe(row_1 + 1, col_1, row_2 + 1, col_2, PipeDirections.Down);
                }
                else return false;
            }
            else if (direction == PipeDirections.Left)
            {
                bool upSuccess = false;
                bool downSuccess = false;

                // Turn up
                if (ch_2 == 'L') upSuccess = TraverseThePipe(row_2, col_2 - 1, row_2, col_2, PipeDirections.Up);

                // Turn down
                if (ch_1 == 'F') downSuccess = TraverseThePipe(row_1, col_1 - 1, row_1, col_1, PipeDirections.Down);

                if (upSuccess || downSuccess) return true;

                // Try straight left
                if (horizontals.Contains(ch_1) && horizontals.Contains(ch_2))
                {
                    return TraverseThePipe(row_1, col_1 - 1, row_2, col_2 - 1, PipeDirections.Left);
                }
                else return false;
            }
            else if (direction == PipeDirections.Right)
            {
                bool upSuccess = false;
                bool downSuccess = false;

                // Turn up
                if (ch_1 == 'J') upSuccess = TraverseThePipe(row_1, col_1, row_1, col_1 + 1, PipeDirections.Up);

                // Turn down
                if (ch_2 == '7') downSuccess = TraverseThePipe(row_2, col_2, row_2, col_2 + 1, PipeDirections.Down);

                if (upSuccess || downSuccess) return true;

                // Try straight right
                if (horizontals.Contains(ch_1) && horizontals.Contains(ch_2))
                {
                    return TraverseThePipe(row_1, col_1 + 1, row_2, col_2 + 1, PipeDirections.Right);
                }
                else return false;
            }

            return false;
        }
    }
}
