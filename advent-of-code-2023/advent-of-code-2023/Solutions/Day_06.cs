using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_06
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "06", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            List<int> times = input.ToList()[0].Split(":")[1].Split().Where(x => x != string.Empty).Select(int.Parse).ToList();
            List<int> distances = input.ToList()[1].Split(":")[1].Split().Where(x => x != string.Empty).Select(int.Parse).ToList();

            int result = 1;

            for(int race = 0; race < times.Count; race++)
            {
                int combinations = 0;

                int time = times[race];
                int speed = time - 1;

                while(speed > 0)
                {
                    if (speed * (time - speed) > distances[race]) combinations++;
                    speed--;
                }

                result *= combinations;
            }

            return result;
        }

        private static int Part2(IEnumerable<string> input)
        {
            List<string> times = input.ToList()[0].Split(":")[1].Split().Where(x => x != string.Empty).ToList();
            List<string> distances = input.ToList()[1].Split(":")[1].Split().Where(x => x != string.Empty).ToList();

            Int64 time = Int64.Parse(string.Join("", times));
            Int64 distance = Int64.Parse(string.Join("", distances));

            Int64 start = 0;
            Int64 end = 0;

            Int64 lowerBound = 2;
            Int64 upperBound = time - 1;

            Int64 speed = lowerBound + (upperBound - lowerBound) / 2;

            // Find lower bound using binary search
            while (true)
            {
                speed = lowerBound + (upperBound - lowerBound) / 2;

                // Current speed is good
                if (speed * (time - speed) > distance)
                {
                    // Current is good, but speed higher by 1 is not good - FOUND THE BOUND!
                    if ((speed - 1) * (time - (speed - 1)) < distance)
                    {
                        start = speed;
                        break;
                    }

                    // Check for lower speed - current speed is new upper bound
                    upperBound = speed;
                }
                else
                {
                    lowerBound = speed;
                }
                // Console.WriteLine(lowerBound + " " + upperBound);
            }

            // Find upper bound using binary search
            lowerBound = 2;
            upperBound = time - 1;

            while (true)
            {
                speed = lowerBound + (upperBound - lowerBound) / 2;

                // Current speed is not good
                if (speed * (time - speed) < distance)
                {
                    // Current is not good, but speed lower by 1 is good - FOUND THE BOUND!
                    if ((speed - 1) * (time - (speed - 1)) > distance)
                    {
                        end = speed;
                        break;
                    }

                    // Check for higher speed - current speed is new upper bound
                    upperBound = speed;
                }
                else
                {
                    lowerBound = speed;
                }
                // Console.WriteLine(lowerBound + " " + upperBound);
            }

            return (int)(end - start);
        }
    }
}
