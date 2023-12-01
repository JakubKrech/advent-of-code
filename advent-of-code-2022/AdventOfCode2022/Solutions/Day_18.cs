using AdventOfCode2022.Tools;
using System.Collections.Generic;

// 3336 too high
// 3292 too high

namespace AdventOfCode2022.Solutions
{
    internal static class Day_18
    {
        public static void Solve()
        {
            AoCTools<int>.RunMeasureTimeAndLog(Part1, Part2, dayId: "18", testInput: false);
        }

        //public static int Part1(IEnumerable<string> input) => Part1(input, false);
        //public static int Part2(IEnumerable<string> input) => Part1(input, true);

        private static int Part1(IEnumerable<string> input/*, bool checkForEmptyBubbles = false*/)
        {
            return -1;
            List<Cube> cubes = new List<Cube>();
            //int max_x = 0, max_y = 0, max_z = 0;

            foreach (var line in input)
            {
                Console.WriteLine(line);
                var xyz = line.Split(',');
                var x = int.Parse(xyz[0]);
                var y = int.Parse(xyz[1]);
                var z = int.Parse(xyz[2]);

                Cube newCube = new(x, y, z);

                //max_x = Math.Max(max_x, x);
                //max_y = Math.Max(max_y, y);
                //max_z = Math.Max(max_z, z);

                Console.WriteLine($"Cube: {x},{y},{z}");

                // Check Left side
                var neighbour = cubes.Find(drop => drop.x == x - 1 && drop.y == y && drop.z == z);
                if (neighbour != null)
                {
                    newCube.sides[(int)Sides.Left] = (int)SideState.Covered;
                    neighbour.sides[(int)Sides.Right] = (int)SideState.Covered;
                }

                // Check Right side
                neighbour = cubes.Find(drop => drop.x == x + 1 && drop.y == y && drop.z == z);
                if (neighbour != null)
                {
                    newCube.sides[(int)Sides.Right] = (int)SideState.Covered;
                    neighbour.sides[(int)Sides.Left] = (int)SideState.Covered;
                }

                // Check Top side
                neighbour = cubes.Find(drop => drop.x == x && drop.y == y - 1 && drop.z == z);
                if (neighbour != null)
                {
                    newCube.sides[(int)Sides.Top] = (int)SideState.Covered;
                    neighbour.sides[(int)Sides.Bottom] = (int)SideState.Covered;
                }

                // Check Bottom side
                neighbour = cubes.Find(drop => drop.x == x && drop.y == y + 1 && drop.z == z);
                if (neighbour != null)
                {
                    newCube.sides[(int)Sides.Bottom] = (int)SideState.Covered;
                    neighbour.sides[(int)Sides.Top] = (int)SideState.Covered;
                }

                // Check Further side
                neighbour = cubes.Find(drop => drop.x == x && drop.y == y && drop.z == z - 1);
                if (neighbour != null)
                {
                    newCube.sides[(int)Sides.Further] = (int)SideState.Covered;
                    neighbour.sides[(int)Sides.Closer] = (int)SideState.Covered;
                }

                // Check Closer side
                neighbour = cubes.Find(drop => drop.x == x && drop.y == y && drop.z == z + 1);
                if (neighbour != null)
                {
                    newCube.sides[(int)Sides.Closer] = (int)SideState.Covered;
                    neighbour.sides[(int)Sides.Further] = (int)SideState.Covered;
                }

                cubes.Add(newCube);
            }

            bool checkForEmptyBubbles = true;
            // Remove empty air bubbles
            if (checkForEmptyBubbles)
            {
                foreach (var cube in cubes)
                {
                    Console.WriteLine($"Checking sides for possible air bubbles: {cube.x},{cube.y},{cube.z}");
                    // Check Left side
                    if (cube.sides[(int)Sides.Left] == (int)SideState.Exposed)
                    {
                        if (IsThisAirBubble(cubes, cube.x - 1, cube.y, cube.z)) cube.sides[(int)Sides.Left] = (int)SideState.Covered;
                    }
                    // Check Right side
                    if (cube.sides[(int)Sides.Right] == (int)SideState.Exposed)
                    {
                        if (IsThisAirBubble(cubes, cube.x + 1, cube.y, cube.z)) cube.sides[(int)Sides.Right] = (int)SideState.Covered;
                    }
                    // Check Top side
                    if (cube.sides[(int)Sides.Top] == (int)SideState.Exposed)
                    {
                        if (IsThisAirBubble(cubes, cube.x, cube.y - 1, cube.z)) cube.sides[(int)Sides.Top] = (int)SideState.Covered;
                    }
                    // Check Bottom side
                    if (cube.sides[(int)Sides.Bottom] == (int)SideState.Exposed)
                    {
                        if (IsThisAirBubble(cubes, cube.x, cube.y + 1, cube.z)) cube.sides[(int)Sides.Bottom] = (int)SideState.Covered;
                    }
                    // Check Further side
                    if (cube.sides[(int)Sides.Further] == (int)SideState.Exposed)
                    {
                        if (IsThisAirBubble(cubes, cube.x, cube.y, cube.z - 1)) cube.sides[(int)Sides.Further] = (int)SideState.Covered;
                    }
                    // Check Closer side
                    if (cube.sides[(int)Sides.Closer] == (int)SideState.Exposed)
                    {
                        if (IsThisAirBubble(cubes, cube.x, cube.y, cube.z + 1)) cube.sides[(int)Sides.Closer] = (int)SideState.Covered;
                    }
                }
            }

            // Count exposed sides
            int totalExposedSides = 0;
            foreach (var cube in cubes)
            {
                //Console.WriteLine(cube);

                foreach (var side in cube.sides)
                {
                    if (side == (int)SideState.Exposed) totalExposedSides++;
                }

            }

            //Console.WriteLine($"Creating array of size {max_x}x{max_y}x{max_z}");

            return totalExposedSides;
        }

        // Check if current empty coordinate is covered from all sides by cubes
        public static bool IsThisAirBubble(List<Cube> cubes, int x, int y, int z)
        {
            Cube? cube = cubes.Find(drop => drop.x == x && drop.y == y && drop.z == z);

            if (cube != null)
                return false;

            Console.WriteLine($"Checking for air bubble: {x},{y},{z}");

            // Check Left side
            if (cubes.Find(drop => drop.x - 1 == x && drop.y == y && drop.z == z) == null)
            {
                return false;
            }

            // Check Right side
            if (cubes.Find(drop => drop.x + 1 == x && drop.y == y && drop.z == z) == null)
            {
                return false;
            }

            // Check Top side
            if (cubes.Find(drop => drop.x == x && drop.y - 1 == y && drop.z == z) == null)
            {
                return false;
            }

            // Check Bottom side
            if (cubes.Find(drop => drop.x == x && drop.y + 1 == y && drop.z == z) == null)
            {
                return false;
            }

            // Check Further side
            if (cubes.Find(drop => drop.x == x && drop.y == y && drop.z - 1 == z) == null)
            {
                return false;
            }

            // Check Closer side
            if (cubes.Find(drop => drop.x == x && drop.y == y && drop.z + 1 == z) == null)
            {
                return false;
            }

            Console.WriteLine("THIS IS AIR BUBBLE!");
            return true;
        }

        enum Sides
        {
            Left,    // x - 1
            Right,   // x + 1
            Top,     // y - 1
            Bottom,  // y + 1
            Further, // z - 1
            Closer   // z + 1
        }

        enum SideState
        {
            Exposed = 0,
            Covered = 1
        }

        public class Cube
        {
            public int x;
            public int y;
            public int z;

            public int[] sides;

            public Cube(int x, int y, int z, int initialSideValue = 0)
            {
                this.x = x;
                this.y = y;
                this.z = z;

                sides = new int[6] { initialSideValue, initialSideValue, initialSideValue, initialSideValue, initialSideValue, initialSideValue };
            }

            public override string ToString()
            {
                return $"{x},{y},{z}: {sides[0]} {sides[1]} {sides[2]} {sides[3]} {sides[4]} {sides[5]}";
            }
        }

        private static int Part2(IEnumerable<string> input)
        {
            bool[,,] array3d;
            List<Cube> cubes = new List<Cube>();
            int max_x = 0, max_y = 0, max_z = 0;

            foreach (string line in input)
            {
                Console.WriteLine(line);
                var xyz = line.Split(',');
                var x = int.Parse(xyz[0]);
                var y = int.Parse(xyz[1]);
                var z = int.Parse(xyz[2]);

                Cube newCube = new(x, y, z, (int)SideState.Covered);

                max_x = Math.Max(max_x, x);
                max_y = Math.Max(max_y, y);
                max_z = Math.Max(max_z, z);

                Console.WriteLine($"Cube: {x},{y},{z}");
                cubes.Add(newCube);
            }

            Console.WriteLine($"Creating array of size {max_x + 2}x{max_y + 2}x{max_z + 2}");
            array3d = new bool[max_x + 2, max_y + 2, max_z + 2];

            //foreach(Cube cube in cubes)
            //{
            //    array3d[cube.x, cube.y, cube.z] = cube;
            //}

            VisitAndSetCubeSides(array3d, cubes, 0, 0, 0);

            for (int i = 0; i < max_x + 2; i++)
            {
                for (int j = 0; j < max_y + 2; j++)
                {
                    for (int k = 0; k < max_z + 2; k++)
                    {
                        if (array3d[i, j, k] == false) Console.Write(".");
                        else Console.Write("x");
                    }
                    Console.WriteLine();
                }

                Console.WriteLine("\n");
            }

            // Count exposed sides
            int totalExposedSides = 0;
            foreach (var cube in cubes)
            {
                //Console.WriteLine(cube);

                foreach (var side in cube.sides)
                {
                    if (side == (int)SideState.Exposed) totalExposedSides++;
                }

            }

            return totalExposedSides;
        }

        public static bool VisitAndSetCubeSides(bool[,,] array3d, List<Cube> cubes, int x, int y, int z)
        {
            if (array3d[x, y, z] == true)
                return true;

            Cube? neighbour;
            array3d[x, y, z] = true; // visited

            Console.WriteLine($"Visiting and setting {x},{y},{z}");

            // Check Left side
            if (x - 1 >= 0)
            {
                neighbour = cubes.Find(drop => drop.x == x - 1 && drop.y == y && drop.z == z);
                if (neighbour != null)
                {
                    neighbour.sides[(int)Sides.Right] = (int)SideState.Exposed;
                }
                else if (array3d[x - 1, y, z] == false)
                {
                    VisitAndSetCubeSides(array3d, cubes, x - 1, y, z);
                }
            }

            // Check Right side
            if (x + 1 < array3d.GetLength(0))
            {
                neighbour = cubes.Find(drop => drop.x == x + 1 && drop.y == y && drop.z == z);
                if (neighbour != null)
                {
                    neighbour.sides[(int)Sides.Left] = (int)SideState.Exposed;
                }
                else if (array3d[x + 1, y, z] == false)
                {
                    VisitAndSetCubeSides(array3d, cubes, x + 1, y, z);
                }
            }

            // Check Top side
            if (y - 1 >= 0)
            {
                neighbour = cubes.Find(drop => drop.x == x && drop.y == y - 1 && drop.z == z);
                if (neighbour != null)
                {
                    neighbour.sides[(int)Sides.Bottom] = (int)SideState.Exposed;
                }
                else if (array3d[x, y - 1, z] == false)
                {
                    VisitAndSetCubeSides(array3d, cubes, x, y - 1, z);
                }
            }
                

            // Check Bottom side
            if(y + 1 < array3d.GetLength(1))
            {
                neighbour = cubes.Find(drop => drop.x == x && drop.y == y + 1 && drop.z == z);
                if (neighbour != null)
                {
                    neighbour.sides[(int)Sides.Top] = (int)SideState.Exposed;
                }
                else if (array3d[x, y + 1, z] == false)
                {
                    VisitAndSetCubeSides(array3d, cubes, x, y + 1, z);
                }
            }
            

            // Check Further side
            if (z - 1 >= 0)
            {
                neighbour = cubes.Find(drop => drop.x == x && drop.y == y && drop.z == z - 1);
                if (neighbour != null)
                {
                    neighbour.sides[(int)Sides.Closer] = (int)SideState.Exposed;
                }
                else if (array3d[x, y, z - 1] == false)
                {
                    VisitAndSetCubeSides(array3d, cubes, x, y, z - 1);
                }
            }


            // Check Closer side
            if (z + 1 < array3d.GetLength(2))
            {
                neighbour = cubes.Find(drop => drop.x == x && drop.y == y && drop.z == z + 1);
                if (neighbour != null)
                {
                    neighbour.sides[(int)Sides.Further] = (int)SideState.Exposed;
                }
                else if (array3d[x, y, z + 1] == false)
                {
                    VisitAndSetCubeSides(array3d, cubes, x, y, z + 1);
                }
            }

            return true;
        }
    }
}
