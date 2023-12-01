using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_12
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "12", testInput: false);
        }

        public static int Part1(IEnumerable<string> input) => Part1(input, -1, -1);

        private static int Part1(IEnumerable<string> input, int start_x = -1, int start_y = -1)
        {
            int height = input.Count();
            int width = input.First().Length;

            Point[,] table2d = new Point[height, width];

            int target_x = -1;
            int target_y = -1;

            for (int i = 0; i < height; i++)
            {
                for(int j = 0; j < width; j++)
                {
                    char c = input.ElementAt(i)[j];

                    if(c == 'S')
                    {
                        if (start_x == -1) start_x = i;
                        if (start_y == -1) start_y = j;
                        table2d[i, j] = new Point('a', 'S');
                    }
                    else if (c == 'E')
                    {
                        target_x = i;
                        target_y = j;
                        table2d[i, j] = new Point('z', 'E');
                    }
                    else
                    {
                        table2d[i, j] = new Point(c);
                    }
                }
            }

            Explore(table2d, start_x, start_y, target_x, target_y, -1);

            return table2d[target_x, target_y].minimumSteps;
        }

        public class Point
        {
            public int value;
            public bool visited = false;
            public int minimumSteps = int.MaxValue;
            public char printChar;

            public Point(char c, char pc = '.')
            {
                value = c;
                printChar = pc;
            }
        }

        public static void ColorPrintFinal(Point[,] table2d, int start_x, int start_y, int target_x, int target_y)
        {
            int curr_x = target_x;
            int curr_y = target_y;

            int height = table2d.GetLength(0);
            int width = table2d.GetLength(1);

            int curr_minimumSteps = table2d[target_x, target_y].minimumSteps;

            while (curr_x != start_x || curr_y != start_y)
            {
                if (curr_y - 1 >= 0 && table2d[curr_x, curr_y - 1].minimumSteps == curr_minimumSteps - 1)
                {
                    table2d[curr_x, curr_y].printChar = '>';
                    curr_y--;
                    curr_minimumSteps = table2d[curr_x, curr_y].minimumSteps;
                }
                else if (curr_y + 1 < width && table2d[curr_x, curr_y + 1].minimumSteps == curr_minimumSteps - 1)
                {
                    table2d[curr_x, curr_y].printChar = '<';
                    curr_y++;
                    curr_minimumSteps = table2d[curr_x, curr_y].minimumSteps;
                }
                else if (curr_x - 1 >= 0 && table2d[curr_x - 1, curr_y].minimumSteps == curr_minimumSteps - 1)
                {
                    table2d[curr_x, curr_y].printChar = 'v';
                    curr_x--;
                    curr_minimumSteps = table2d[curr_x, curr_y].minimumSteps;
                }
                else if (curr_x + 1 < height && table2d[curr_x + 1, curr_y].minimumSteps == curr_minimumSteps - 1)
                {
                    table2d[curr_x, curr_y].printChar = '^';
                    curr_x++;
                    curr_minimumSteps = table2d[curr_x, curr_y].minimumSteps;
                }
                else
                {
                    break;
                }
            }
        }

        private static void Explore(Point[,] table2d, int x, int y, int target_x, int target_y, int steps)
        {
            steps += 1;

            int height = table2d.GetLength(0); 
            int width = table2d.GetLength(1);

            table2d[x, y].minimumSteps = Math.Min(table2d[x, y].minimumSteps, steps);
            table2d[x, y].visited = true;

            if (x == target_x && y == target_y)
                return;

            int currValue = table2d[x, y].value;

            // go left
            if (y - 1 >= 0)
            {
                int leftValue = table2d[x, y - 1].value;

                if (leftValue <= currValue + 1)
                {
                    if (table2d[x, y - 1].visited == false || table2d[x, y - 1].minimumSteps > steps + 1)
                    {
                        Explore(table2d, x, y - 1, target_x, target_y, steps);
                    }
                }
            }

            // go right
            if (y + 1 < width)
            {
                int leftValue = table2d[x, y + 1].value;

                if (leftValue <= currValue + 1)
                {
                    if (table2d[x, y + 1].visited == false || table2d[x, y + 1].minimumSteps > steps + 1)
                    {
                        Explore(table2d, x, y + 1, target_x, target_y, steps);
                    }
                }
            }

            // go up
            if (x - 1 >= 0)
            {
                int upValue = table2d[x - 1, y].value;

                if (upValue <= currValue + 1)
                {
                    if(table2d[x - 1, y].visited == false || table2d[x - 1, y].minimumSteps > steps + 1)
                    {
                        Explore(table2d, x - 1, y, target_x, target_y, steps);
                    }
                }
            }

            // go down
            if (x + 1 < height)
            {
                int downValue = table2d[x + 1, y].value;

                if (downValue <= currValue + 1)
                {
                    if (table2d[x + 1, y].visited == false || table2d[x + 1, y].minimumSteps > steps + 1)
                    {
                        Explore(table2d, x + 1, y, target_x, target_y, steps);
                    }
                }
            }
        }

        private static int Part2(IEnumerable<string> input)
        {
            int height = input.Count();
            int width = input.First().Length;

            int shortestPathFromDownhill = int.MaxValue;

            for (int i = 0; i < height; i++)
            {
                for (int j = 0; j < width; j++)
                {
                    char c = input.ElementAt(i)[j];
                    bool acceptableStartPoint = false;

                    if (c == 'S' || c == 'a')
                    {
                        if (j - 1 >= 0 && input.ElementAt(i)[j - 1] == 'b')
                        {
                            acceptableStartPoint = true;
                        }
                        else if (j + 1 < width && input.ElementAt(i)[j + 1] == 'b')
                        {
                            acceptableStartPoint = true;
                        }
                        else if (i - 1 >= 0 && input.ElementAt(i - 1)[j] == 'b')
                        {
                            acceptableStartPoint = true;
                        }
                        else if (i + 1 < height && input.ElementAt(i + 1)[j] == 'b')
                        {
                            acceptableStartPoint = true;
                        }
                    }

                    if(acceptableStartPoint)
                    {
                        shortestPathFromDownhill = Math.Min(shortestPathFromDownhill, Part1(input, i, j));
                    }
                }
            }

            return shortestPathFromDownhill;
        }
    }
}
