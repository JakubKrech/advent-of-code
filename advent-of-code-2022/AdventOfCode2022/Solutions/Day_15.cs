using AdventOfCode2022.Tools;
using System.Text.RegularExpressions;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_15
    {
        public static void Solve()
        {
            AoCTools<int, double>.RunMeasureTimeAndLog(Part1, Part2, dayId: "15", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            Regex regex = new(@"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)");
            List<Sensor> sensors = new();

            // Parse input into sensors with calculated manthattan distance to closest beacon
            foreach (var line in input)
            {
                var matches = regex.Match(line).Groups;
                sensors.Add(new Sensor(matches[1].Value, matches[2].Value, matches[3].Value, matches[4].Value));
            }

            // Detect overlaps from every sensor at targetY
            int targetY = 2000000;
            List<Tuple<int, int>> overlapFragments = new();
            int minimumCoord = int.MaxValue;
            int maximumCoord = int.MinValue;

            foreach (Sensor sensor in sensors)
            {
                int overlapHeight = Math.Abs(targetY - sensor.y);

                if (overlapHeight <= sensor.distanceToBeacon)
                {
                    int heightLeft = sensor.distanceToBeacon - overlapHeight;
                    int start = sensor.x - heightLeft;
                    int end = sensor.x + heightLeft;
                    overlapFragments.Add(new Tuple<int, int>(start, end));

                    minimumCoord = int.Min(minimumCoord, start);
                    maximumCoord = int.Max(maximumCoord, end);
                }
            }

            int[] tab = new int[Math.Abs(minimumCoord - maximumCoord) + 1];

            foreach(var overlap in overlapFragments)
            {
                for(int i = overlap.Item1; i <= overlap.Item2; i++)
                {
                    tab[i + Math.Abs(minimumCoord)] = 1;
                }
            }

            // Remove 1 for each beacon that already exists in that line
            foreach (Sensor sensor in sensors)
            {
                if(sensor.closestBeaconY == targetY)
                {
                    if(sensor.closestBeaconX >= minimumCoord && sensor.closestBeaconX <= maximumCoord)
                    {
                        tab[sensor.closestBeaconX + Math.Abs(minimumCoord)] = 0;
                    }
                }
            }

            return tab.Sum();
        }

        private static double Part2(IEnumerable<string> input)
        {
            Regex regex = new(@"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)");
            List<Sensor> sensors = new();
            Int64 result = -1;

            // Parse input into sensors with calculated manthattan distance to closest beacon
            foreach (var line in input)
            {
                var matches = regex.Match(line).Groups;
                sensors.Add(new Sensor(matches[1].Value, matches[2].Value, matches[3].Value, matches[4].Value));
            }

            // Detect overlaps from every sensor at targetY
            for(int targetY = 0; targetY <= 4000000; targetY++)
            {
                List<Tuple<int, int>> overlapFragments = new();
                int minimumCoord = int.MaxValue;
                int maximumCoord = 4000000;

                foreach (Sensor sensor in sensors)
                {
                    int overlapHeight = Math.Abs(targetY - sensor.y);

                    if (overlapHeight <= sensor.distanceToBeacon)
                    {
                        int heightLeft = sensor.distanceToBeacon - overlapHeight;
                        int start = Math.Max(0, sensor.x - heightLeft);
                        int end = Math.Min(4000000, sensor.x + heightLeft);
                        overlapFragments.Add(new Tuple<int, int>(start, end));

                        minimumCoord = int.Min(minimumCoord, start);
                        maximumCoord = int.Max(maximumCoord, end);
                    }
                }

                // Merge overlaps until
                overlapFragments.Sort((x, y) => x.Item1.CompareTo(y.Item1));

                // int min = overlapFragments[0].Item1;
                Int64 max = overlapFragments[0].Item2;

                for(int i = 1; i < overlapFragments.Count; i++)
                {
                    if(max >= overlapFragments[i].Item1)
                    {
                        if(max < overlapFragments[i].Item2) max = overlapFragments[i].Item2;
                    }
                    else
                    {
                        result = ((max + 1) * 4000000) + targetY;
                        break;
                    }
                }
            }

            return result;
        }

        public class Sensor
        {
            public int x;
            public int y;

            public int closestBeaconX;
            public int closestBeaconY;

            public int distanceToBeacon;

            public Sensor(string xx, string yy, string cbx, string cby)
            {
                this.x = int.Parse(xx);
                this.y = int.Parse(yy);
                this.closestBeaconX = int.Parse(cbx);
                this.closestBeaconY = int.Parse(cby);

                this.distanceToBeacon = Math.Abs(x - closestBeaconX) + Math.Abs(y - closestBeaconY);
            }
        }
    }
}
