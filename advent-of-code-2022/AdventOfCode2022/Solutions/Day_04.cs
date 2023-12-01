using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_04
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "04", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            int mutually_contained_assignments = 0;

            foreach(string line in input)
            {
                ParseInputLine(line, out int left_start, out int left_end, out int right_start, out int right_end);

                if (left_start <= right_start && left_end >= right_end ||
                    left_start >= right_start && left_end <= right_end)
                {
                    mutually_contained_assignments++;
                }
            }

            return mutually_contained_assignments;
        }

        private static int Part2(IEnumerable<string> input)
        {
            int overlapping_assignments = 0;

            foreach (string line in input)
            {
                ParseInputLine(line, out int left_start, out int left_end, out int right_start, out int right_end);

                if (left_start <= right_start && left_end >= right_start ||
                    right_start <= left_start && right_end >= left_start)
                {
                    overlapping_assignments++;
                    continue;
                }
            }

            return overlapping_assignments;
        }

        private static void ParseInputLine(string input_line, out int left_start, out int left_end, out int right_start, out int right_end)
        {
            string left = input_line.Split(',')[0];
            string right = input_line.Split(',')[1];

            left_start = int.Parse(left.Split("-")[0]);
            left_end = int.Parse(left.Split("-")[1]);

            right_start = int.Parse(right.Split("-")[0]);
            right_end = int.Parse(right.Split("-")[1]);
        }

        private static void DrawAssignments(int ll, int lr, int rl, int rr)
        {
            int max = rr > lr ? rr + 2 : lr + 2;

            // Draw first assignment
            for(int i = 0; i < max; i++)
            {
                if (i >= ll && i <= lr) Console.Write("x");
                else Console.Write(".");
            }
            Console.WriteLine($"   {ll}-{lr}");

            // Draw second assignment
            for (int i = 0; i < max; i++)
            {
                if (i >= rl && i <= rr) Console.Write("x");
                else Console.Write(".");
            }
            Console.WriteLine($"   {rl}-{rr}\n");
        }
    }
}
