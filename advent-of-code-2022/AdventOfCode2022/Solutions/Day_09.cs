using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_09
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "09", testInput: false);
        }

        public static int Part1(IEnumerable<string> input) => Solution(input, 2);
        public static int Part2(IEnumerable<string> input) => Solution(input, 10);

        private static int Solution(IEnumerable<string> input, int ammountOfRopeKnots)
        {
            // calculate max left, right, up and down coordinate that head will reach
            int max_x = 0, max_y = 0, min_x = 0, min_y = 0;

            Coordinates current = new();

            foreach (var line in input)
            {
                var data = line.Split(' ');
                string moveDirection = data[0];
                int moveDistance = int.Parse(data[1]);

                switch (moveDirection)
                {
                    case "R":
                        current.x += moveDistance;
                        max_x = int.Max(max_x, current.x);
                        break;
                    case "L":
                        current.x -= moveDistance;
                        min_x = int.Min(min_x, current.x);
                        break;
                    case "U":
                        current.y += moveDistance;
                        max_y = int.Max(max_y, current.y);
                        break;
                    case "D":
                        current.y -= moveDistance;
                        min_y = int.Min(min_y, current.y);
                        break;
                }
            }

            // Prepare 2d array and segments initial coordinates
            int required_x = 1 + max_x + (min_x < 0 ? -min_x : 0);
            int required_y = 1 + max_y + (min_y < 0 ? -min_y : 0);
            int start_x = 1 + (min_x < 0 ? -min_x - 1 : 0);
            int start_y = 1 + (min_y < 0 ? -min_y - 1 : 0);
            
            bool[,] table2d = new bool[required_x, required_y];
            table2d[start_x, start_y] = true; // initial position of head and tail is visited
            
            List<Coordinates> ropeKnots = new();
            int head = 0;
            int tail = ammountOfRopeKnots - 1;

            for(int i = 0; i < ammountOfRopeKnots; i++)
            {
                ropeKnots.Add(new Coordinates(start_x, start_y));
            }

            // Move head and adjust other rope knots as needed
            foreach (var line in input)
            {
                var data = line.Split(' ');
                string moveDirection = data[0];
                int moveDistance = int.Parse(data[1]);

                for(int i = 0; i < moveDistance; i++)
                {
                    // Update head position
                    switch (moveDirection)
                    {
                        case "R":
                            ropeKnots[head].x++;
                            break;
                        case "L":
                            ropeKnots[head].x--;
                            break;
                        case "U":
                            ropeKnots[head].y++;
                            break;
                        case "D":
                            ropeKnots[head].y--;
                            break;
                    }

                    // If needed, move knot after knot, stop after first knot that did not move
                    for(int ropeId = 1; ropeId < ammountOfRopeKnots; ropeId++)
                    {
                        bool moved = false;
                        int previousKnotId = ropeId - 1;

                        if (ropeKnots[previousKnotId].x != ropeKnots[ropeId].x && ropeKnots[previousKnotId].y != ropeKnots[ropeId].y)
                        {
                            if (Math.Abs(ropeKnots[previousKnotId].x - ropeKnots[ropeId].x) + Math.Abs(ropeKnots[previousKnotId].y - ropeKnots[ropeId].y) > 2)
                            {
                                ropeKnots[ropeId].x += ropeKnots[previousKnotId].x > ropeKnots[ropeId].x ? 1 : -1;
                                ropeKnots[ropeId].y += ropeKnots[previousKnotId].y > ropeKnots[ropeId].y ? 1 : -1;
                                moved = true;
                            }
                        }
                        else if (Math.Abs(ropeKnots[previousKnotId].x - ropeKnots[ropeId].x) > 1)
                        {
                            ropeKnots[ropeId].x += ropeKnots[previousKnotId].x > ropeKnots[ropeId].x ? 1 : -1;
                            moved = true;
                        }
                        else if (Math.Abs(ropeKnots[previousKnotId].y - ropeKnots[ropeId].y) > 1)
                        {
                            ropeKnots[ropeId].y += ropeKnots[previousKnotId].y > ropeKnots[ropeId].y ? 1 : -1;
                            moved = true;
                        }

                        if (!moved) break;
                    }

                    table2d[ropeKnots[tail].x, ropeKnots[tail].y] = true;
                }
            }

            int totalVisitedByTail = 0;

            for(int x = 0; x < required_x; x++)
            {
                for(int y = 0; y < required_y; y++)
                {
                    // Console.Write(table2d[x, y] == true ? "X" : ".");
                    totalVisitedByTail += table2d[x, y] == true ? 1 : 0;
                }
                // Console.WriteLine();
            }

            return totalVisitedByTail;
        }

        private class Coordinates
        {
            public int x = 0;
            public int y = 0;

            public Coordinates(){}

            public Coordinates(int xx, int yy)
            {
                x = xx;
                y = yy;
            }
        }
    }
}
