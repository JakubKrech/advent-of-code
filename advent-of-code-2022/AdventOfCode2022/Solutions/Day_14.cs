using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_14
    {
        public static void Solve()
        {
            AoCTools<int>.RunMeasureTimeAndLog(Part1, Part2, dayId: "14", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            List<List<Tuple<int, int>>> rockLines = ParseInputAndFindXYRanges(input, out int min_x, out int max_x, out int min_y, out int max_y);
            int[,] tab = Initialize2dTabWithStartingValues(rockLines, max_x + 5, max_y + 5, false);

            // Drop the sand until it falls out under max y point
            bool anySandFallenIntoAbyss = false;
            int sand_id = 0;

            while (!anySandFallenIntoAbyss)
            {
                sand_id++;
                int current_x = 500;
                int current_y = 0;

                while (true)
                {
                    if (current_y > max_y)
                    {
                        anySandFallenIntoAbyss = true;
                        break;
                    }

                    if (tab[current_x, current_y + 1] == (int)StructureType.Air) // sand can freely fall down
                    {
                        current_y++;
                    }
                    else if(tab[current_x - 1, current_y + 1] == (int)StructureType.Air) // sand try to go left - down
                    {
                        current_x--;
                        current_y++;
                    }
                    else if (tab[current_x + 1, current_y + 1] == (int)StructureType.Air) // sand try to go right - down
                    {
                        current_x++;
                        current_y++;
                    }
                    else
                    {
                        tab[current_x, current_y] = (int)StructureType.Sand;
                        break;
                    }
                }
            }

            // PrintTab(tab, min_x, max_x, min_y, max_y);

            return sand_id - 1;
        }

        private static int Part2(IEnumerable<string> input)
        {
            List<List<Tuple<int, int>>> rockLines = ParseInputAndFindXYRanges(input, out int min_x, out int max_x, out int min_y, out int max_y);
            int[,] tab = Initialize2dTabWithStartingValues(rockLines, max_x * 2, max_y + 2, true);

            // Drop the sand until it fills sand spawn point with sand
            bool spawnPointFilledWithSand = false;
            int sand_id = 0;

            while (!spawnPointFilledWithSand)
            {
                sand_id++;
                int current_x = 500;
                int current_y = 0;

                while (true)
                {
                    if (tab[500, 0] == (int)StructureType.Sand)
                    {
                        spawnPointFilledWithSand = true;
                        break;
                    }

                    if (tab[current_x, current_y + 1] == (int)StructureType.Air) // sand can freely fall down
                    {
                        current_y++;
                    }
                    else if (tab[current_x - 1, current_y + 1] == (int)StructureType.Air) // sand try to go left - down
                    {
                        current_x--;
                        current_y++;
                    }
                    else if (tab[current_x + 1, current_y + 1] == (int)StructureType.Air) // sand try to go right - down
                    {
                        current_x++;
                        current_y++;
                    }
                    else
                    {
                        tab[current_x, current_y] = (int)StructureType.Sand;
                        break;
                    }
                }
            }

            // PrintTab(tab, min_x - max_y, max_x + max_y, min_y, max_y);

            return sand_id - 1;
        }

        enum StructureType
        {
            Air = 0,
            Rock = 1,
            Sand = 2,
            SandSpawnPoint = 3
        }

        public static List<List<Tuple<int, int>>> ParseInputAndFindXYRanges(IEnumerable<string> input, out int min_x, out int max_x, out int min_y, out int max_y)
        {
            min_x = int.MaxValue; // just for pretty result printing
            min_y = int.MaxValue; // just for pretty result printing

            max_x = 0;
            max_y = 0;

            List<List<Tuple<int, int>>> rockLines = new();

            foreach (var line in input)
            {
                var points = line.Split(" -> ");
                List<Tuple<int, int>> rockPoints = new List<Tuple<int, int>>();

                foreach (var p in points)
                {
                    var xy = p.Split(',');

                    int x = int.Parse(xy[0]);
                    int y = int.Parse(xy[1]);

                    min_x = Math.Min(min_x, x); // just for pretty result printing
                    min_y = Math.Min(min_y, y); // just for pretty result printing

                    max_x = Math.Max(max_x, x);
                    max_y = Math.Max(max_y, y);

                    rockPoints.Add(new Tuple<int, int>(x, y));
                }

                rockLines.Add(rockPoints);
            }

            return rockLines;
        }

        public static int[,] Initialize2dTabWithStartingValues(List<List<Tuple<int, int>>> rockLines, int max_x, int max_y, bool AddFloor = false)
        {
            int[,] tab = new int[max_x, max_y + 1];

            // Add sand spawn point
            tab[500, 0] = (int)StructureType.SandSpawnPoint;

            // Optionally add floor
            if (AddFloor)
            {
                // Add floor
                for (int x = 0; x < max_x; x++)
                {
                    tab[x, max_y] = (int)StructureType.Rock;
                }
            }

            // Add rock lines
            foreach (var rockPoints in rockLines)
            {
                for (int i = 0; i < rockPoints.Count - 1; i++)
                {
                    var point1 = rockPoints[i];
                    var point2 = rockPoints[i + 1];

                    if (point1.Item1 == point2.Item1) // x coord is the same, iterate over y
                    {
                        var start = point1.Item2 < point2.Item2 ? point1.Item2 : point2.Item2;
                        var end = point1.Item2 < point2.Item2 ? point2.Item2 : point1.Item2;

                        int x = point1.Item1;

                        for (int y = start; y <= end; y++)
                        {
                            tab[x, y] = (int)StructureType.Rock;
                        }
                    }
                    else // x coord is different, iterate over it
                    {
                        var start = point1.Item1 < point2.Item1 ? point1.Item1 : point2.Item1;
                        var end = point1.Item1 < point2.Item1 ? point2.Item1 : point1.Item1;

                        int y = point1.Item2;

                        for (int x = start; x <= end; x++)
                        {
                            tab[x, y] = (int)StructureType.Rock;
                        }
                    }
                }
            }

            return tab;
        }

        public static void PrintTab(int[,] tab, int min_x, int max_x, int min_y, int max_y)
        {
            Console.WriteLine();

            min_x = Math.Max(min_x - 5, 0);
            min_y = 0;

            for (int y = min_y; y < max_y; y++)
            {
                for (int x = min_x; x < max_x; x++)
                {
                    if (tab[x, y] == (int)StructureType.Air)
                    {
                        Console.Write(" ");
                    }
                    else if (tab[x, y] == (int)StructureType.Rock)
                    {
                        Console.Write("#");
                    }
                    else if (tab[x, y] == (int)StructureType.Sand)
                    {
                        Console.Write("o");
                    }
                    else if (tab[x,y] == 4)
                    {
                        Console.Write("~");
                    }
                    else // Sand spawn point
                    {
                        Console.Write("+");
                    }
                }
                Console.WriteLine();
            }

            for(int i = min_x; i < max_x; i++) Console.Write("-");
            Console.WriteLine();
        }
    }
}
