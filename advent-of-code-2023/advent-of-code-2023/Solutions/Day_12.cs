using AdventOfCode2022.Tools;
using System.Text;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_12
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "12", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            int totalArrangements = 0;

            foreach (string str in input)
            {
                var data = str.Split(' ');

                string initialLine = "." + data[0] + ".";
                List<int> originalDamagedSprings = data[1].Split(',').Select(x => int.Parse(x)).ToList();
                List<int> damagedSprings = data[1].Split(',').Select(x => int.Parse(x)).ToList();

                HashSet<string> arrangements = new() { initialLine };

                while(damagedSprings.Count > 0)
                {
                    HashSet<string> newArrangements = new();

                    int currentDamaged = damagedSprings[0];
                    damagedSprings.Remove(currentDamaged);

                    foreach(var arr in arrangements)
                    {
                        for (int i = 0; i < initialLine.Length; i++)
                        {
                            StringBuilder lineCopy = new StringBuilder(arr);
                            int remainingDamaged = currentDamaged;
                            int j = i;

                            if (lineCopy[j] == '.' || lineCopy[j - 1] == '#') continue;

                            while (remainingDamaged > 0)
                            {
                                if (lineCopy[j] == '#' || lineCopy[j] == '?')
                                {
                                    lineCopy[j] = '#';
                                    remainingDamaged--;
                                    j++;
                                }
                                else
                                {
                                    break;
                                }
                            }

                            if (remainingDamaged == 0 && lineCopy[j] != '#')
                            {
                                newArrangements.Add(lineCopy.ToString());
                            }
                        }
                    }

                    arrangements.Clear();
                    arrangements = newArrangements;
                }

                HashSet<string> validArrangements = new();

                foreach (var arr in arrangements)
                {
                    var damagedStringsInNewArr = GetExistingDamagedStrings(arr);
                    bool valid = true;

                    for (int i = 0; i < damagedStringsInNewArr.Count; i++)
                    {
                        if (damagedStringsInNewArr.Count != originalDamagedSprings.Count ||
                            damagedStringsInNewArr[i] != originalDamagedSprings[i]) valid = false;
                    }

                    if (valid) validArrangements.Add(arr);
                }

                totalArrangements += validArrangements.Count;
            }

            return totalArrangements;
        }

        private static int Part2(IEnumerable<string> input)
        {
            return 0;
        }

        private static List<int> GetExistingDamagedStrings(string line)
        {
            List<int> damagedStrings = new();

            int start = -1;

            for (int i = 0; i < line.Length; i++)
            {
                if (line[i] == '#' && start == -1) start = i;

                if (line[i] != '#' && start != -1)
                {
                    damagedStrings.Add(i - start);
                    start = -1;
                }
            }

            return damagedStrings;
        }
    }
}
