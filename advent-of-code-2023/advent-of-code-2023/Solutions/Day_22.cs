using AdventOfCode2022.Tools;

using Brick = System.Tuple<int, int, int, int, int, int, bool>;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_22
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "22", testInput: false);
        }

        // x1, x2, y1, y2, z1, z2, ID, grounded
        private static List<Brick> bricks = new();

        private static int Part1(IEnumerable<string> input)
        {
            List<string> inputList = input.ToList();

            int max_x = 0;
            int max_y = 0;
            int max_z = 0;

            foreach(var line in inputList)
            {
                var data = line.Split('~');
                var left = data[0].Split(',').Select(int.Parse).ToList();
                var right = data[1].Split(',').Select(int.Parse).ToList();

                bool grounded = left[2] == 1;

                if (right[0] > max_x) max_x = right[0];
                if (right[1] > max_y) max_y = right[1];
                if (right[2] > max_z) max_z = right[2];

                bricks.Add(new Brick(left[0], right[0], left[1], right[1], left[2], right[2], grounded));
            }

            bricks.Sort((a,b) => a.Item5.CompareTo(b.Item5));

            // Make all bricks land either on the ground or on another brick
            while(bricks.Any(x => x.Item7 == false)) // while at least one brick is still falling
            {
                var brick = bricks.First(x => x.Item7 == false);

                var landed_under = bricks
                    .Where(x => x.Item7 == true)
                    .Where(x => DoBricksOverlapXY(x, brick))
                    .ToList();

                landed_under.Sort((x, y) => y.Item6.CompareTo(x.Item6));

                int new_base_z = 1; // by default brick will land on the ground (z = 1)

                foreach (var under in landed_under)
                {
                    if(DoBricksOverlapXY(brick, under))
                    {
                        new_base_z = under.Item6 + 1; // brick landed on top of this brick
                        break;
                    }
                }

                int brick_height = brick.Item6 - brick.Item5;

                bricks.Remove(brick);
                bricks.Add(new Brick(brick.Item1, brick.Item2, brick.Item3, brick.Item4, new_base_z, new_base_z + brick_height, true));
                bricks.Sort((a, b) => a.Item5.CompareTo(b.Item5));
            }

            // For each brick, starting from top, check if they can be desintegrated.
            // They can be, if they are not supporting any other block, or if the block they are supporting is supported by some other block.

            int canBeDesintegratedCounter = 0;

            for(int i = bricks.Count - 1; i >= 0; i--)
            {
                var brick = bricks[i];

                if (CanBrickBeSafelyDesintegrated(bricks, brick)) canBeDesintegratedCounter++;
            }

            return canBeDesintegratedCounter;
        }

        private static int Part2(IEnumerable<string> input)
        {
            int total = 0;
            int max_layer = bricks.Max(x => x.Item6);

            for (int i = 0; i < bricks.Count; i++)
            {
                var initiallyDesintegrated = bricks[i];

                // If desintegrating current brick causes no other bricks to fall - nothing to calculate
                if (CanBrickBeSafelyDesintegrated(bricks, initiallyDesintegrated)) continue;

                HashSet<Brick> desintegrated = new() { initiallyDesintegrated };
                Queue<Brick> toCheck = new();
                toCheck.Enqueue(initiallyDesintegrated);

                while(toCheck.Count > 0)
                {
                    var currentlyCheckedBrick = toCheck.Dequeue();

                    var bricksStartingLayerAbove = bricks
                        .Where(x => x.Item5 == currentlyCheckedBrick.Item6 + 1)
                        .ToList();

                    var bricksOnLayerAboveSupportedByCurrentBrick = bricksStartingLayerAbove
                        .Where(x => DoBricksOverlapXY(x, currentlyCheckedBrick))
                        .ToList();

                    // Check if those bricks are supported by any other brick that has not yet been desintegrated
                    foreach (var needSupport in bricksOnLayerAboveSupportedByCurrentBrick)
                    {
                        var supports = bricks
                            .Where(x => x.Item6 == needSupport.Item5 - 1)
                            .Where(x => DoBricksOverlapXY(x, needSupport))
                            .ToList();

                        var remainingSupports = supports
                            .Where(x => !desintegrated.Contains(x))
                            .ToList();

                        // If there is no more support left, mark it as desintegrated and add to queue to check
                        if (remainingSupports.Count == 0)
                        {
                            if (desintegrated.Add(needSupport))
                            {
                                toCheck.Enqueue(needSupport);
                            }
                        }
                    }
                }

                desintegrated.Remove(initiallyDesintegrated);
                total += desintegrated.Count;
            }

            return total;
        }

        private static bool DoBricksOverlapXY(Brick brick1, Brick brick2)
        {
            bool x_overlap = true;
            bool y_overlap = true;

            // Check x overlap
            if(brick1.Item2 < brick2.Item1 || brick1.Item1 > brick2.Item2)
            {
                x_overlap = false;
            }

            // Check y overlap
            if (brick1.Item4 < brick2.Item3 || brick1.Item3 > brick2.Item4)
            {
                x_overlap = false;
            }

            return x_overlap && y_overlap;
        }

        private static bool CanBrickBeSafelyDesintegrated(List<Brick> bricks, Brick brick)
        {
            var bricksOnLayerAboveBrick = bricks
                    .Where(x => x.Item5 == brick.Item6 + 1)
                    .ToList();

            var bricksOnTopThatNeedSupport = bricksOnLayerAboveBrick
                .Where(x => DoBricksOverlapXY(brick, x))
                .ToList();

            // There are no bricks on the layer above that need support, can be desintegrated
            if (bricksOnTopThatNeedSupport.Count == 0)
            {
                return true;
            }

            bool allBricksCanBeSupportedByOtherBlock = true;

            var otherBricksEndingOnTheCurrentLayer = bricks
                .Where(x => x.Item6 == brick.Item6)
                .Where(x => x != brick)
                .ToList();

            foreach (var needSupport in bricksOnTopThatNeedSupport)
            {
                bool otherBlockFound = false;
                foreach (var otherCandidateBrick in otherBricksEndingOnTheCurrentLayer)
                {
                    if (DoBricksOverlapXY(needSupport, otherCandidateBrick))
                    {
                        otherBlockFound = true;
                        break;
                    }
                }

                if (!otherBlockFound)
                {
                    allBricksCanBeSupportedByOtherBlock = false;
                    break;
                }
            }

            if (allBricksCanBeSupportedByOtherBlock) return true;
            
            return false;
        }
    }
}
