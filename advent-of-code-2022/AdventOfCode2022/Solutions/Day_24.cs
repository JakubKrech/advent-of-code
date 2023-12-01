using AdventOfCode2022.Tools;
using System.Numerics;

// 824 too high
// 500 too high

namespace AdventOfCode2022.Solutions
{
    internal static class Day_24
    {
        public static void Solve()
        {
            AoCTools<int>.RunMeasureTimeAndLog(Part1, Part2, dayId: "24", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            int height = input.Count();
            int width = input.ElementAt(0).Length;
            int ammountOfTurns = 200;

            int[,,] tab = new int[height - 2, width - 2, ammountOfTurns + 1];
            List<Blizzard> blizzards = new();

            // Read input and create Blizzards
            for (int x = 1; x < height - 1; x++)
            {
                for(int y = 1; y < width - 1; y++)
                {
                    if (input.ElementAt(x)[y] != '.')
                    {
                        blizzards.Add(new Blizzard(x - 1, y - 1, input.ElementAt(x)[y]));
                        tab[x - 1, y - 1, 0]++;
                    }
                }
            }

            Console.WriteLine($"Created {blizzards.Count} blizzards");

            PrintBlizzardCount(tab, 0);

            for(int turn = 1; turn < ammountOfTurns; turn++)
            {
                for (int x = 0; x < height - 2; x++)
                {
                    for (int y = 0; y < width - 2; y++)
                    {
                        tab[x, y, turn] = tab[x, y, turn - 1];
                    }
                }

                foreach (Blizzard blizzard in blizzards)
                {
                    tab[blizzard.x, blizzard.y, turn]--;

                    // Update blizzard x
                    blizzard.x += blizzard.x_delta;
                    if (blizzard.x < 0) blizzard.x = height - 2 - 1;
                    else if (blizzard.x >= height - 2) blizzard.x = 0;

                    // Update blizzard y
                    blizzard.y += blizzard.y_delta;
                    if (blizzard.y < 0) blizzard.y = width - 2 - 1;
                    else if (blizzard.y >= width - 2) blizzard.y = 0;

                    tab[blizzard.x, blizzard.y, turn]++;
                }

                //PrintBlizzardCount(tab, turn);
            }

            List<int> possiblePaths = new();

            Explore(tab, possiblePaths, -1, 0, 0);

            return possiblePaths.Count > 0 ? possiblePaths.Min() : -1;
        }

        private static int Part2(IEnumerable<string> input)
        {

            return -1;
        }

        public static void Explore(int[,,] tab, List<int> possiblePaths, int x, int y, int turn)
        {
            // Console.WriteLine($"Explore {x},{y} turn {turn}");

            int minimumPath = possiblePaths.Count == 0 ? int.MaxValue : possiblePaths.Min();

            int height = tab.GetLength(0);
            int width = tab.GetLength(1);

            int distanceFromExit = ((height - 1) - x) + (width - 1) - y; 

            if (turn + distanceFromExit >= Math.Min(200, minimumPath - 1))
                return;

            if (x == height - 1 && y == width - 1)
            {
                Console.WriteLine($"FOUND {turn + 1}");
                possiblePaths.Add(turn + 1);
                return;
            }

            if(x != -1)
            {
                // Check right
                if (y + 1 < width && tab[x, y + 1, turn + 1] == 0)
                {
                    //Console.WriteLine("right");
                    Explore(tab, possiblePaths, x, y + 1, turn + 1);
                }

                // Check left
                if (y - 1 >= 0 && tab[x, y - 1, turn + 1] == 0)
                {
                    //Console.WriteLine("left");
                    Explore(tab, possiblePaths, x, y - 1, turn + 1);
                }

                // Check down
                if (x + 1 < height && tab[x + 1, y, turn + 1] == 0)
                {
                    //Console.WriteLine("down");
                    Explore(tab, possiblePaths, x + 1, y, turn + 1);
                }

                // Check up
                if (x - 1 >= 0 && tab[x - 1, y, turn + 1] == 0)
                {
                    //Console.WriteLine("up");
                    Explore(tab, possiblePaths, x - 1, y, turn + 1);
                }

                // Wait
                if (tab[x, y, turn + 1] == 0)
                {
                    //Console.WriteLine("Wait");
                    Explore(tab, possiblePaths, x, y, turn + 1);
                }
            }
            else
            {
                if(tab[0, 0, turn + 1] == 0)
                {
                    Explore(tab, possiblePaths, 0, 0, turn + 1);
                }
                else
                {
                    Explore(tab, possiblePaths, -1, 0, turn + 1);
                }
            }
        }

        public static void PrintBlizzardCount(int[,,] tab, int turn)
        {
            int height = tab.GetLength(0);
            int width = tab.GetLength(1);

            Console.WriteLine($"Turn {turn}:");

            for (int x = 0; x < height; x++)
            {
                for (int y = 0; y < width; y++)
                {
                    Console.Write(tab[x, y, turn]);
                }
                Console.WriteLine();
            }
            Console.WriteLine();
        }

        public class Blizzard
        {
            public int x; // up/down
            public int y; // right/left

            public int x_delta = 0;
            public int y_delta = 0;

            public Blizzard(int x, int y, char sign)
            {
                this.x = x;
                this.y = y;

                switch(sign)
                {
                    case '>':
                        this.y_delta = 1;
                        break;
                    case '<':
                        this.y_delta = -1;
                        break;
                    case 'v':
                        this.x_delta = 1;
                        break;
                    case '^':
                        this.x_delta = -1;
                        break;
                }

                // Console.WriteLine($"New Blizzard {sign} at: {x},{y}  delta: {x_delta},{y_delta}");
            }
        }
    }
}
