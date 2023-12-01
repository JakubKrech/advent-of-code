using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_06
    {
        public static void Solve()
        {
            AoCTools<string>.RunMeasureTimeAndLog(Part1, Part2, "06", testInput: false);
        }

        public static string Part1(IEnumerable<string> input) => FindEncodedPackets(input, 4);
        public static string Part2(IEnumerable<string> input) => FindEncodedPackets(input, 14);

        private static int FindEncodedPacket(string input, int lengthOfPacket)
        {
            Queue<char> queue = new();

            int currentChar = 1;

            foreach (char c in input)
            {
                while (queue.Contains(c))
                {
                    queue.Dequeue();
                }

                queue.Enqueue(c);

                if (queue.Count == lengthOfPacket)
                {
                    break;
                }

                currentChar++;
            }

            return currentChar;
        }


        // Function created to allow testing with multiple inputs
        private static string FindEncodedPackets(IEnumerable<string> input, int lengthOfPacket)
        {
            List<int> results = new();

            foreach (string line in input)
            {
                results.Add(FindEncodedPacket(line, lengthOfPacket));
            }

            return string.Join(",", results);
        }
    }
}
