using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_08
    {
        public static void Solve()
        {
            AoCTools<Int64>.RunMeasureTimeAndLog(Part1, Part2, "08", testInput: false);
        }

        private static Int64 Part1(IEnumerable<string> input)
        {
            List<string> inputList = input.ToList();
            Dictionary<string, Tuple<string, string>> maps = GetMaps(inputList);
            string path = inputList[0];

            string currentMap = "AAA";
            int currentStep = 0;

            while (currentMap != "ZZZ")
            {
                currentMap = (path[currentStep % path.Length] == 'L' ? maps[currentMap].Item1 : maps[currentMap].Item2);
                currentStep++;
            }

            return currentStep;
        }

        private static Int64 Part2(IEnumerable<string> input)
        {
            List<string> inputList = input.ToList();
            Dictionary<string, Tuple<string, string>> maps = GetMaps(inputList);
            string path = inputList[0];

            List<string> currentMaps = new List<string>();

            foreach (var map in maps)
            {
                if (map.Key.EndsWith('A')) currentMaps.Add(map.Key);
            }

            // In this part it is required to notice that each of maps reach the correct
            // state (with Z as the last letter) on an interval. By finding interval for each map
            // it is possible to find the step that fits all the maps in a sane ammount of time.
            List<int> intervals = new List<int>();
            foreach (var _ in currentMaps) intervals.Add(-1);
            int currentStep = 0;

            while (true)
            {
                for (int i = 0; i < currentMaps.Count; i++)
                {
                    currentMaps[i] = (path[currentStep % path.Length] == 'L' ? maps[currentMaps[i]].Item1 : maps[currentMaps[i]].Item2);
                    if (currentMaps[i].EndsWith('Z'))
                    {
                        if (intervals[i] == -1)
                        {
                            intervals[i] = currentStep + 1;
                        }
                    }
                }

                currentStep++;
                if(!intervals.Contains(-1)) break;
            }

            // Find the smallest common multiple number of all the intervals
            List<int> divisors = new ();
            int divisor = 2;

            while(true)
            {
                bool atLeastOneDivisionHappened = true;

                while (atLeastOneDivisionHappened)
                {
                    atLeastOneDivisionHappened = false;

                    for (int i = 0; i < intervals.Count; i++)
                    {
                        if (intervals[i] % divisor == 0)
                        {
                            atLeastOneDivisionHappened = true;
                            intervals[i] /= divisor;
                        }
                    }

                    if (atLeastOneDivisionHappened) divisors.Add(divisor);
                }

                divisor++;

                if (intervals.All(interval => interval == 1)) break;
            }

            return divisors.Aggregate(1L, (sum, d) => sum *= d);
        }

        private static Dictionary<string, Tuple<string, string>> GetMaps(List<string> inputList)
        {
            Dictionary<string, Tuple<string, string>> maps = new();

            for (int i = 2; i < inputList.Count; i++)
            {
                var data = inputList[i][..3];
                var left = inputList[i].Substring(7, 3);
                var right = inputList[i].Substring(12, 3);

                maps[data] = Tuple.Create(left, right);
            }

            return maps;
        }
    }
}
