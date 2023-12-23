using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_23
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "23", testInput: false);
        }

        private static List<List<char>> board = new();
        private static List<List<Tuple<int, int>>> solutions = new();

        private static int Part1(IEnumerable<string> input)
        {
            foreach(string row in input.ToList())
            {
                List<char> newLine = new();
                foreach (char c in row)
                {
                    newLine.Add(c);
                }
                board.Add(newLine);
            }

            Queue<List<Tuple<int, int>>> queue = new();

            List<Tuple<int, int>> initialStep = new()
            {
                new Tuple<int, int>(0, 1)
            };
            queue.Enqueue(new List<Tuple<int, int>>(initialStep));

            int row_count = board.Count;
            int col_count = board[0].Count;

            while(queue.Count > 0)
            {
                var previousSteps = queue.Dequeue();
                var lastStep = previousSteps.Last();
                int row = lastStep.Item1;
                int col = lastStep.Item2;

                // If we are at the end point - save the result and end current path processing
                if(row == row_count - 1 && col == col_count - 2)
                {
                    solutions.Add(previousSteps);
                    continue;
                }

                // We try to move to each direction from current position

                // Go right
                if(col + 1 < col_count && !"#<".Contains(board[row][col + 1]))
                {
                    if(!previousSteps.Contains(new Tuple<int, int>(row, col + 1)))
                    {
                        int newCol = col + 1;
                        List<Tuple<int, int>> newSteps = new(previousSteps) { new Tuple<int, int>(row, newCol) };
                        bool canMove = true;

                        // Check if there is no slope
                        if (board[row][newCol] == '>')
                        {
                            newCol++;
                            newSteps.Add(new Tuple<int, int>(row, newCol));

                            if (previousSteps.Contains(new Tuple<int, int>(row, newCol))) canMove = false;
                        }

                        if(canMove)
                        {
                            queue.Enqueue(new List<Tuple<int, int>>(newSteps));
                        }
                    }
                }

                // Go left
                if (col - 1 >= 0 && !"#>".Contains(board[row][col - 1]))
                {
                    if (!previousSteps.Contains(new Tuple<int, int>(row, col - 1)))
                    {
                        int newCol = col - 1;
                        List<Tuple<int, int>> newSteps = new(previousSteps) { new Tuple<int, int>(row, newCol) };
                        bool canMove = true;

                        // Check if there is no slope
                        if (board[row][newCol] == '<')
                        {
                            newCol--;
                            newSteps.Add(new Tuple<int, int>(row, newCol));

                            if (previousSteps.Contains(new Tuple<int, int>(row, newCol))) canMove = false;
                        }

                        if (canMove)
                        {
                            queue.Enqueue(new List<Tuple<int, int>>(newSteps));
                        }
                    }
                }

                // Go down
                if (row + 1 < row_count && !"#^".Contains(board[row + 1][col]))
                {
                    if (!previousSteps.Contains(new Tuple<int, int>(row + 1, col)))
                    {
                        int newRow = row + 1;
                        List<Tuple<int, int>> newSteps = new(previousSteps) { new Tuple<int, int>(newRow, col) };
                        bool canMove = true;

                        // Check if there is no slope
                        if (board[newRow][col] == 'v')
                        {
                            newRow++;
                            newSteps.Add(new Tuple<int, int>(newRow, col));

                            if (previousSteps.Contains(new Tuple<int, int>(newRow, col))) canMove = false;
                        }

                        if (canMove)
                        {
                            queue.Enqueue(new List<Tuple<int, int>>(newSteps));
                        }
                    }
                }

                // Go up
                if (row - 1 > 0 && !"#v".Contains(board[row - 1][col]))
                {
                    if (!previousSteps.Contains(new Tuple<int, int>(row - 1, col)))
                    {
                        int newRow = row - 1;
                        List<Tuple<int, int>> newSteps = new(previousSteps) { new Tuple<int, int>(newRow, col) };
                        bool canMove = true;

                        // Check if there is no slope
                        if (board[newRow][col] == '^')
                        {
                            newRow--;
                            newSteps.Add(new Tuple<int, int>(newRow, col));

                            if (previousSteps.Contains(new Tuple<int, int>(newRow, col))) canMove = false;
                        }

                        if (canMove)
                        {
                            queue.Enqueue(new List<Tuple<int, int>>(newSteps));
                        }
                    }
                }
            }

            return solutions.Max(x => x.Count) - 1; // subtract the initial position
        }

        private static int Part2(IEnumerable<string> input)
        {
            // key: start location, value: List of lengths + source_row + source_col
            Dictionary<Tuple<int, int>, HashSet<Tuple<int, int, int>>> crossroads = new();

            int row_count = board.Count;
            int col_count = board[0].Count;

            // start location, coming from direction
            Queue<Tuple<int, int, char>> queue = new();
            queue.Enqueue(new Tuple<int, int, char>(0, 1, 'v'));

            while (queue.Count > 0)
            {
                var newCrossroad = queue.Dequeue();
                int row = newCrossroad.Item1;
                int col = newCrossroad.Item2;
                char comingFrom = newCrossroad.Item3;
                int length = 1;

                // Initial move to correct position out of intersection
                if (comingFrom == '>') col++;
                else if (comingFrom == '<') col--;
                else if (comingFrom == '^') row--;
                else if (comingFrom == 'v') row++;

                bool initialMove = true;

                while (!"><^v".Contains(board[row][col]) || initialMove)
                {
                    if (row == board.Count - 1 && col == board[0].Count - 2) break;

                    initialMove = false;
                    length++;

                    if (col + 1 < col_count && board[row][col + 1] != '#' && comingFrom != '<')
                    {
                        col++;
                        comingFrom = '>';
                    }

                    // Go left
                    else if (col - 1 >= 0 && board[row][col - 1] != '#' && comingFrom != '>')
                    {
                        col--;
                        comingFrom = '<';
                    }

                    // Go down
                    else if (row + 1 < row_count && board[row + 1][col] != '#' && comingFrom != '^')
                    {
                        row++;
                        comingFrom = 'v';
                    }

                    // Go up
                    else if (row - 1 > 0 && board[row - 1][col] != '#' && comingFrom != 'v')
                    {
                        row--;
                        comingFrom = '^';
                    }
                }

                if (row == board.Count - 1 && col == board[0].Count - 2)
                {
                    var kkey = new Tuple<int, int>(newCrossroad.Item1, newCrossroad.Item2);
                    if (crossroads.ContainsKey(kkey))
                    {
                        crossroads[kkey].Add(new Tuple<int, int, int>(length, row, col));
                    }
                    else
                    {
                        crossroads[kkey] = new HashSet<Tuple<int, int, int>> { new Tuple<int, int, int>(length, row, col) };
                    }

                    continue;
                }

                length++;

                if (comingFrom == '>') col++;
                else if (comingFrom == '<') col--;
                else if (comingFrom == '^') row--;
                else if (comingFrom == 'v') row++;

                // We are now inside the intersection, try adding new section to calculate
                if (board[row][col + 1] == '>') queue.Enqueue(new Tuple<int, int, char>(row, col, '>'));
                if (board[row][col - 1] == '<') queue.Enqueue(new Tuple<int, int, char>(row, col, '<'));
                if (board[row + 1][col] == 'v') queue.Enqueue(new Tuple<int, int, char>(row, col, 'v'));
                if (board[row - 1][col] == '^') queue.Enqueue(new Tuple<int, int, char>(row, col, '^'));

                // Add current calculated section to the dictionary of intersections
                var key = new Tuple<int, int>(newCrossroad.Item1, newCrossroad.Item2);
                if(crossroads.ContainsKey(key))
                {
                    crossroads[key].Add(new Tuple<int, int, int>(length, row, col));
                }
                else
                {
                    crossroads[key] = new HashSet<Tuple<int, int, int>> { new Tuple<int, int, int>(length, row, col) };
                }
            }

            var longestWalk = GetLongestWalk(crossroads);

            return longestWalk;
        }

        // Crossroads: key = start location, value= List of lengths + source_row + source_col
        private static int GetLongestWalk(Dictionary<Tuple<int, int>, HashSet<Tuple<int, int, int>>> crossroads)
        {
            // crossroadId_1, crossroadId_2, distance
            List<Tuple<int, int, int>> roads = new();

            int current_id = 0;

            Dictionary<Tuple<int, int>, int> crossroadNames = new();

            foreach(var crossroad in crossroads)
            {
                var crossroad_location = crossroad.Key;
                crossroadNames[crossroad_location] = current_id++;
            }

            // Add last final crossroad
            crossroadNames[new Tuple<int, int>(board.Count - 1, board[0].Count - 2)] = current_id;

            // Get list of roads (from, to, distance)
            foreach(var key in crossroads.Keys)
            {
                var start_crossroad = key;

                foreach(var paths in crossroads[key])
                {
                    var end_crossroad = new Tuple<int, int>(paths.Item2, paths.Item3);

                    roads.Add(new Tuple<int, int, int>(crossroadNames[key], crossroadNames[end_crossroad], paths.Item1));
                }
            }

            for(int i = 0; i < roads.Count; i++)
            {
                var curr = roads[i];

                if(curr.Item1 > curr.Item2)
                {
                    roads[i] = new Tuple<int, int, int>(curr.Item2, curr.Item1, curr.Item3);
                }
            }

            // Tuple: crossroad id, previous visited crossroads ids
            HashSet<Tuple<int, List<int>>> queue = new()
            {
                new Tuple<int, List<int>>(0, new List<int>(){ 0 })
            };

            int max_length = 0;

            while (queue.Count > 0)
            {
                var curr = queue.First();
                queue.Remove(curr);
                var crossroad_id = curr.Item1;
                var listOfPreviousCrossroads = curr.Item2;

                if (crossroad_id == current_id)
                {
                    int length = 0;

                    for (int i = 0; i + 1 < listOfPreviousCrossroads.Count; i++)
                    {
                        int road_length = roads.Where(x => (listOfPreviousCrossroads[i] == x.Item1 && listOfPreviousCrossroads[i + 1] == x.Item2) || (listOfPreviousCrossroads[i] == x.Item2 && listOfPreviousCrossroads[i + 1] == x.Item1)).ToList()[0].Item3;
                        length += road_length;
                    }

                    if (length > max_length) max_length = length;
                }

                var availableRoads = roads.Where(x => x.Item1 == crossroad_id || x.Item2 == crossroad_id).ToList();

                foreach (var road in availableRoads)
                {
                    var crossroad_1 = road.Item1;
                    var crossroad_2 = road.Item2;

                    var start = road.Item1 == crossroad_id ? road.Item1 : road.Item2;
                    var end = road.Item1 == crossroad_id ? road.Item2 : road.Item1;

                    if (listOfPreviousCrossroads.Contains(end)) continue;

                    List<int> newListOfPreviousCrossroads = new();

                    foreach (var x in listOfPreviousCrossroads) newListOfPreviousCrossroads.Add(x);

                    newListOfPreviousCrossroads.Add(end);

                    queue.Add(new Tuple<int, List<int>>(end, newListOfPreviousCrossroads));
                }
            }

            return max_length;
        }
    }
}
