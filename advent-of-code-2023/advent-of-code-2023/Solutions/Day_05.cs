using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_05
    {
        private static List<Int64> Part2Results = new List<Int64>();
        private static List<List<List<Int64>>> globalData;

        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "05", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            return 0;

            List<Int64> seeds = input.First().Split(':')[1].Trim().Split().Select(Int64.Parse).ToList();
            var inputList = input.ToList();

            List<Int64> results = new List<Int64>();
            var data = ParseAlmanacData(input);

            foreach (Int64 seed in seeds)
            {
                Int64 current = seed;

                //Console.Write("Seed " + seed + ": ");

                foreach (var transition in data)
                {
                    bool transitionFound = false;

                    foreach (var rule in transition)
                    {
                        if (transitionFound)
                        {
                            transitionFound = false;
                            break;
                        }

                        Int64 destination = rule[0];
                        Int64 source = rule[1];
                        Int64 length = rule[2];

                        if(current >= source && current < source + length)
                        {
                            Int64 shift = destination - source;

                            current += shift;
                            transitionFound = true;

                            Console.WriteLine("Transition found " + current + " -> " + (current - shift));
                        }
                    }

                    if(!transitionFound)
                    {
                        Console.WriteLine("NOT found: " + current);
                    }

                    //Console.Write(current + ", ");
                }

                results.Add(current);
                Console.WriteLine("\nSeed " + seed + " -> Location " + current);
            }

            foreach (Int64 result in results)
            {
                Console.WriteLine(result);
            }

            Console.WriteLine("RESULT: " + results.Min());
            return (int)results.Min();
        }

        private static int Part2(IEnumerable<string> input)
        {
            List<Int64> seeds = input.First().Split(':')[1].Trim().Split().Select(Int64.Parse).ToList();
            var inputList = input.ToList();

            List<Int64> results = new List<Int64>();
            globalData = ParseAlmanacData(input);

            for(int i = 0; i < seeds.Count; i += 2)
            {
                Int64 rangeStart = seeds[i];
                Int64 rangeEnd = rangeStart + seeds[i + 1] - 1;
                Console.WriteLine("STARTING NEW: " + rangeStart  + " - " + rangeEnd);
                RecurrentSeedAnalysis(rangeStart, rangeEnd, 0);
            }

            Console.WriteLine("RESULT: " + Part2Results.Min());

            Part2Results.Sort();
            return (int)Part2Results.Min();
        }

        private static List<List<List<Int64>>> ParseAlmanacData(IEnumerable<string> input)
        {
            List<List<List<Int64>>> data = new List<List<List<Int64>>>();
            var inputList = input.ToList();

            List<List<Int64>> transitionData = new List<List<Int64>>();

            for (int i = 2; i < inputList.Count(); i++)
            {
                string line = inputList[i];

                if (line == string.Empty)
                {
                    data.Add(transitionData);
                    continue;
                }
                else if (!char.IsDigit(line[0]))
                {
                    transitionData = new List<List<Int64>>();
                    continue;
                }

                List<Int64> intValues = line.Trim().Split(' ').Select(x => Int64.Parse(x)).ToList();
                transitionData.Add(intValues);
            }

            data.Add(transitionData);

            return data;
        }

        private static void RecurrentSeedAnalysis(Int64 rangeStart, Int64 rangeEnd, int transitionLayer)
        {
            //Console.WriteLine("RSA: " + rangeStart + "-" + rangeEnd + "(" + transitionLayer + ")");

            if(transitionLayer == globalData.Count)
            {
                Console.WriteLine(" >>>>>>>>>>>>>>> RESULT: " + rangeStart + " " + rangeEnd);
                Part2Results.Add(rangeStart);
                return;
            }

            var transition = globalData[transitionLayer];

            // sort rules by beginning of the rule range
            transition.Sort((x, y) => { return x[1] > y[1] ? 1 : 0; });
            transition.Sort(new ListComparer());

            //foreach (var x in transition)
            //{
            //    Console.WriteLine(x[1] + " " + (x[1] + x[2] - 1));
            //}

            foreach (var rule in transition)
            {
                Int64 ruleStart = rule[1];
                Int64 ruleEnd = rule[1] + rule[2] - 1;
                Int64 shift = rule[0] - rule[1];

                // 1) Entire range inside the rule - forward and break
                if(rangeStart >= ruleStart && rangeEnd <= ruleEnd) 
                {
                    RecurrentSeedAnalysis(rangeStart + shift, rangeEnd + shift, transitionLayer + 1);
                    rangeStart = rangeEnd + 1;
                    break;
                }

                // 2) Range and rule do not overlap at all - continue
                if(rangeEnd < ruleStart || rangeStart > ruleEnd)
                {
                    continue;
                }

                // 3) Rule inside the range - forward beginning, forward ruled middle, continue
                if(rangeStart < ruleStart && rangeEnd > ruleEnd)
                {
                    // Forward beginning
                    RecurrentSeedAnalysis(rangeStart, ruleStart - 1, transitionLayer + 1);

                    // Forward ruled middle
                    RecurrentSeedAnalysis(ruleStart + shift, ruleEnd + shift, transitionLayer + 1);

                    // update rangeStart
                    rangeStart = ruleEnd + 1;
                    continue;
                }

                // 4) Overlap on the left end - forward beginning, forward ruled rest, break
                if(rangeStart < ruleStart && rangeEnd > ruleStart && rangeEnd < ruleEnd)
                {
                    // Forward beginning
                    RecurrentSeedAnalysis(rangeStart, ruleStart - 1, transitionLayer + 1);

                    // Forward ruled rest
                    RecurrentSeedAnalysis(ruleStart + shift, rangeEnd + shift, transitionLayer + 1);

                    rangeStart = rangeEnd + 1;
                    break;
                }

                // 5) Overlap on the right end - forward ruled beginning, continue
                // update rangeStart
                if(rangeStart > ruleStart && rangeStart < ruleEnd && rangeEnd > ruleEnd)
                {
                    // Forward ruled beginning
                    RecurrentSeedAnalysis(rangeStart + shift, ruleEnd + shift, transitionLayer + 1);

                    rangeStart = ruleEnd + 1;
                    continue;
                }
            }

            // 6) If some range is remaining - forward
            if (rangeStart <= rangeEnd)
            {
                RecurrentSeedAnalysis(rangeStart, rangeEnd, transitionLayer + 1);
            }
        }

        // Custom comparer for sorting by the second element of the inner lists
        public class ListComparer : IComparer<List<Int64>>
        {
            public int Compare(List<Int64> x, List<Int64> y)
            {
                // Compare the second elements of the lists
                return x[1].CompareTo(y[1]);
            }
        }
    }
}
