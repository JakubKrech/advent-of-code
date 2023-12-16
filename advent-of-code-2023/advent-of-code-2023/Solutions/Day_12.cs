using AdventOfCode2022.Tools;
using System.Text;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_12
    {
        public static void Solve()
        {
            AoCTools<Int64>.RunMeasureTimeAndLog(Part1, Part2, "12", testInput: false);
        }

        private static Int64 Part1(IEnumerable<string> input)
        {
            Int64 result = 0;

            foreach (string str in input)
            {
                var data = str.Split(' ');

                string shortData = data[0];
                List<int> shortDamagedSprings = data[1].Split(',').Select(x => int.Parse(x)).ToList();

                result += FindCombinationsRecurrent(shortData, shortDamagedSprings);
            }

            return result;
        }

        private static Dictionary<string, Int64> cache = new Dictionary<string, Int64>();

        private static Int64 Part2(IEnumerable<string> input)
        {
            Int64 result = 0;

            foreach(string str in input)
            {
                var data = str.Split(' ');

                StringBuilder longData = new();
                List<int> longDamagedSprings = new List<int>();

                for (int i = 0; i < 5; i++)
                {
                    longData.Append(data[0] + "?");
                    longDamagedSprings.AddRange(data[1].Split(',').Select(x => int.Parse(x)).ToList());
                }

                longData.Remove(longData.Length - 1, 1); // remove trailing question mark

                result += FindCombinationsRecurrent(longData.ToString(), longDamagedSprings);
            }

            return result;
        }

        private static Int64 FindCombinationsRecurrent(string line, List<int> damaged)
        {
            string cacheKey = GetCacheKey(line, damaged);

            if (cache.TryGetValue(GetCacheKey(line, damaged), out Int64 cachedResult)) return cachedResult;

            // Check if remaining line is long enough to fit all damaged
            int requiredLength = damaged.Sum() + (damaged.Count - 1);
            if (line.Length < requiredLength) return 0;

            // Check if there is enough '?' and '#' spots to fit all damaged
            int remainingHashAndQuestionMarks = line.Count(c => c == '?') + line.Count(c => c == '#');
            int remainingDamagedSum = damaged.Sum();
            if (remainingHashAndQuestionMarks < remainingDamagedSum) return 0;

            // Check if the longest existing sequence of # is not longer than biggest remaining damaged
            if (GetLengthOfLongestSequenceOfHash(line) > damaged.Max()) return 0;

            Int64 result = 0;
            int currentDamaged = damaged[0];
            bool existingHashTouched = false;

            // Fit first damaged in every possible spot and make recurrent call
            for(int i = 0; i < line.Length; i++)
            {
                string remainingSubstring = line.Substring(i);

                // If line is already too short
                if (i + currentDamaged > line.Length) break;

                if (line[i] == '.')
                {
                    if (existingHashTouched) break;
                    continue; // do nothing, go to next character
                }

                else if (line[i] == '#') // damaged needs to start here and have correct length
                {
                    existingHashTouched = true;

                    // Check if currentDamaged can be placed here
                    var xxx = line.Substring(i, currentDamaged);
                    if (xxx.Contains('.')) continue;

                    // Check if it can be followed by immediate '.'
                    if (i + currentDamaged < line.Length && line[i + currentDamaged] == '#') continue;

                    // Check if previous char can be dot
                    if (i - 1 >= 0 && line[i - 1] == '#') break;

                    // Make recurrent call with line substring starting after post-currentDamaged dot
                    string newSubstring = i + currentDamaged + 1 < line.Length ? line.Substring(i + currentDamaged + 1) : string.Empty;

                    if (damaged.Count == 1)
                    {
                        if (newSubstring.Contains('#')) continue;
                        else
                        {
                            result += 1;
                            continue;
                        }
                    }
                    else if (i + currentDamaged + 1 < line.Length)
                    {
                        result += FindCombinationsRecurrent(newSubstring, damaged.GetRange(1, damaged.Count - 1));
                    }
                }

                else if (line[i] == '?')
                {
                    // If line is already too short
                    if (i + currentDamaged > line.Length) break;

                    // Check if currentDamaged can be placed here
                    var xxx = line.Substring(i, currentDamaged);
                    if (xxx.Contains('.')) continue;

                    // Check if previous char can be dot
                    if (i - 1 >= 0 && line[i - 1] == '#') break;

                    // Check if it can be followed by immediate '.'
                    if (i + currentDamaged < line.Length && line[i + currentDamaged] == '#') continue;

                    // Make recurrent call with line substring starting after post-currentDamaged dot
                    string newSubstring = i + currentDamaged + 1 < line.Length ? line.Substring(i + currentDamaged + 1) : string.Empty;

                    if (damaged.Count == 1)
                    {
                        if (newSubstring.Contains('#')) continue;
                        else
                        {
                            result += 1;
                            continue;
                        }
                    }
                    else if (i + currentDamaged + 1 < line.Length)
                    {
                        result += FindCombinationsRecurrent(line.Substring(i + currentDamaged + 1), damaged.GetRange(1, damaged.Count - 1));
                    }
                }
            }

            cache[GetCacheKey(line, damaged)] = result;
            
            return result;
        }

        private static string GetCacheKey(string line, List<int> damaged)
        {
            StringBuilder cacheKey = new(line);
            cacheKey.Append(':');

            foreach (int d in damaged)
            {
                cacheKey.Append(d);
                cacheKey.Append(',');
            }

            return cacheKey.ToString();
        }

        private static int GetLengthOfLongestSequenceOfHash(string line)
        {
            int longest = -1;
            int current = 0;

            foreach(char c in line)
            {
                if(c == '#')
                {
                    current++;
                    if(current > longest) longest = current;
                }
                else
                {
                    current = 0;
                }
            }

            return longest;
        }
    }
}
