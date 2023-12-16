using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_16
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "16", testInput: false);
        }

        private static int board_row_count;
        private static int board_col_count;

        private static int Part1(IEnumerable<string> input)
        {
            List<List<char>> board = GetBoard(input);

            return GetEnergizedScore(board, '>', 0, 0);
        }

        private static int Part2(IEnumerable<string> input)
        {
            List<List<char>> board = GetBoard(input);

            int maxEnergizedScore = 0;

            board_row_count = input.ToList().Count;
            board_col_count = input.ToList()[0].Length;

            // Top and bottom row
            for (int col = 0; col < board_col_count; col++)
            {
                int score = GetEnergizedScore(board, 'v', 0, col);
                maxEnergizedScore = int.Max(maxEnergizedScore, score);

                score = GetEnergizedScore(board, '^', board_row_count - 1, col);
                maxEnergizedScore = int.Max(maxEnergizedScore, score);
            }

            // Left and right col
            for (int row = 0; row < board_row_count; row++)
            {
                int score = GetEnergizedScore(board, '>', row, 0);
                maxEnergizedScore = int.Max(maxEnergizedScore, score);

                score = GetEnergizedScore(board, '<', row, board_row_count - 1);
                maxEnergizedScore = int.Max(maxEnergizedScore, score);
            }

            return maxEnergizedScore;
        }

        private static int GetEnergizedScore(List<List<char>> board, char dir, int start_row, int start_column) 
        {
            List<List<char>> energized = new();

            board_row_count = board.Count;
            board_col_count = board[0].Count;

            for (int row = 0; row < board_row_count; row++)
            {
                List<char> energizedLine = new();
                for (int col = 0; col < board_col_count; col++)
                {
                    energizedLine.Add('.');
                }
                energized.Add(energizedLine);
            }

            List<char> initialDirections = GetInitialDirections(board, dir, start_row, start_column);
            List<Tuple<char, int, int>> toAnalyze = new();
            foreach(var c in initialDirections)
            {
                toAnalyze.Add(new Tuple<char, int, int>(c, start_row, start_column));
            }

            Dictionary<string, List<char>> previousOccurences = new();

            while (toAnalyze.Count > 0)
            {
                char direction = toAnalyze.Last().Item1;
                int row = toAnalyze.Last().Item2;
                int col = toAnalyze.Last().Item3;

                toAnalyze.Remove(toAnalyze.Last());

                energized[row][col] = '#';

                string key = row + "," + col;
                if (previousOccurences.ContainsKey(key) && previousOccurences[key].Contains(direction)) continue;
                else
                {
                    if (previousOccurences.ContainsKey(key))
                    {
                        previousOccurences[key].Add(direction);
                    }
                    else
                    {
                        previousOccurences[key] = new List<char>() { direction };
                    }
                }

                if (!IsInsideBoard(direction, row, col)) continue;

                int new_row = row;
                int new_col = col;

                if (direction == '>') new_col++;
                else if (direction == '<') new_col--;
                else if (direction == 'v') new_row++;
                else /* direction == '^' */ new_row--;

                if (board[new_row][new_col] == '.')
                {
                    if (IsInsideBoard(direction, row, col)) toAnalyze.Add(new Tuple<char, int, int>(direction, new_row, new_col));
                }
                else if (board[new_row][new_col] == '/')
                {
                    energized[new_row][new_col] = '#';
                    if (direction == '>')
                    {
                        if (IsInsideBoard('^', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('^', new_row, new_col));
                    }
                    else if (direction == '<')
                    {
                        if (IsInsideBoard('v', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('v', new_row, new_col));
                    }
                    else if (direction == 'v')
                    {
                        if (IsInsideBoard('<', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('<', new_row, new_col));
                    }
                    else /* direction == '^' */
                    {
                        if (IsInsideBoard('>', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('>', new_row, new_col));
                    }
                }
                else if (board[new_row][new_col] == '\\')
                {
                    energized[new_row][new_col] = '#';
                    if (direction == '>')
                    {
                        if (IsInsideBoard('v', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('v', new_row, new_col));
                    }
                    else if (direction == '<')
                    {
                        if (IsInsideBoard('^', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('^', new_row, new_col));
                    }
                    else if (direction == 'v')
                    {
                        if (IsInsideBoard('>', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('>', new_row, new_col));
                    }
                    else /* direction == '^' */
                    {
                        if (IsInsideBoard('<', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('<', new_row, new_col));
                    }
                }
                else if (board[new_row][new_col] == '|')
                {
                    if ("^v".Contains(direction)) // treat as an empty spot
                    {
                        toAnalyze.Add(new Tuple<char, int, int>(direction, new_row, new_col));
                    }
                    else // '<' or '>' - split to top and bot
                    {
                        energized[new_row][new_col] = '#';
                        if (IsInsideBoard('^', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('^', new_row, new_col));
                        if (IsInsideBoard('v', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('v', new_row, new_col));
                    }
                }
                else if (board[new_row][new_col] == '-')
                {
                    if ("<>".Contains(direction)) // treat as an empty spot
                    {
                        toAnalyze.Add(new Tuple<char, int, int>(direction, new_row, new_col));
                    }
                    else // '^' or 'v' - split to left and right
                    {
                        energized[new_row][new_col] = '#';
                        if (IsInsideBoard('<', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('<', new_row, new_col));
                        if (IsInsideBoard('>', new_row, new_col)) toAnalyze.Add(new Tuple<char, int, int>('>', new_row, new_col));
                    }
                }
            }

            int totalEnergized = 0;

            for (int row = 0; row < board_row_count; row++)
            {
                for (int col = 0; col < board_col_count; col++)
                {
                    if (energized[row][col] == '#') totalEnergized++;
                }
            }

            return totalEnergized;
        }

        private static List<List<char>> GetBoard(IEnumerable<string> input)
        {
            List<string> inputList = input.ToList();
            List<List<char>> board = new();

            for (int row = 0; row < inputList.Count; row++)
            {
                List<char> line = new();
                List<char> energizedLine = new();
                for (int col = 0; col < inputList[0].Length; col++)
                {
                    line.Add(inputList[row][col]);
                    energizedLine.Add('.');
                }
                board.Add(line);
            }

            return board;
        }

        private static bool IsInsideBoard(char direction, int cur_x, int cur_y)
        {
            if (direction == '>')
            {
                if (cur_y + 1 >= board_col_count) return false;
            }
            else if (direction == '<')
            {
                if (cur_y -1 < 0) return false;
            }
            else if (direction == 'v')
            {
                if (cur_x + 1 >= board_row_count) return false;
            }
            else // direction == '^'
            {
                if (cur_x - 1 < 0) return false;
            }

            return true;
        }

        private static List<char> GetInitialDirections(List<List<char>> board, char direction = '>', int row = 0, int col = 0)
        {
            List<char> directions = new();

            if (direction == '>')
            {
                if (board[row][col] == '.') directions.Add('>');
                else if (board[row][col] == '/') directions.Add('^');
                else if (board[row][col] == '\\') directions.Add('v');
                else if (board[row][col] == '|')
                {
                    directions.Add('^');
                    directions.Add('v');
                }
                else /* board[row][col] == '-' */ directions.Add('>');
            }
            else if (direction == '<')
            {
                if (board[row][col] == '.') directions.Add('<');
                else if (board[row][col] == '/') directions.Add('v');
                else if (board[row][col] == '\\') directions.Add('^');
                else if (board[row][col] == '|')
                {
                    directions.Add('^');
                    directions.Add('v');
                }
                else /* board[row][col] == '-' */ directions.Add('<');
            }
            else if (direction == '^')
            {
                if (board[row][col] == '.') directions.Add('^');
                else if (board[row][col] == '/') directions.Add('>');
                else if (board[row][col] == '\\') directions.Add('<');
                else if (board[row][col] == '|') directions.Add('^');
                else /* board[row][row] == '-' */
                {
                    directions.Add('<');
                    directions.Add('>');
                }
            }
            else /*  (direction == 'v') */
            {
                if (board[row][col] == '.') directions.Add('v');
                else if (board[row][col] == '/') directions.Add('<');
                else if (board[row][col] == '\\') directions.Add('>');
                else if (board[row][col] == '|') directions.Add('v');
                else /* board[row][row] == '-' */
                {
                    directions.Add('<');
                    directions.Add('>');
                }
            }

            return directions;
        }
    }
}
