using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_09
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "09", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            int sumOfExtrapolatedValues = 0;

            foreach (var line in input)
            {
                List<List<int>> histories = GetInitialHistories(line);

                histories.Last().Add(0); // Add new trailing zero

                // Go up and add new trailing value for each row except the lowest row
                for (int i = histories.Count - 2; i >= 0; i--)
                {
                    histories[i].Add(histories[i].Last() + histories[i + 1].Last());
                }

                sumOfExtrapolatedValues += histories[0].Last();
            }

            return sumOfExtrapolatedValues;
        }

        private static int Part2(IEnumerable<string> input)
        {
            int sumOfExtrapolatedValues = 0;

            foreach (var line in input)
            {
                List<List<int>> histories = GetInitialHistories(line);

                histories.Last().Insert(0, 0); // Add new front zero

                // Go up and add new front value for each row except the lowest row
                for (int i = histories.Count - 2; i >= 0; i--)
                {
                    int diff = histories[i][0] - histories[i + 1][0];
                    histories[i].Insert(0, diff);
                }

                sumOfExtrapolatedValues += histories[0][0];
            }

            return sumOfExtrapolatedValues;
        }

        private static List<List<int>> GetInitialHistories(string initialHistory)
        {
            List<int> initialHistoryList = initialHistory.Split(' ').Select(int.Parse).ToList();
            List<List<int>> histories = new() { initialHistoryList };

            while (true)
            {
                List<int> newHistory = new();
                bool allZeros = true;

                for (int i = 0; i + 1 < histories[histories.Count - 1].Count; i++)
                {
                    int diff = histories[histories.Count - 1][i + 1] - histories[histories.Count - 1][i];
                    newHistory.Add(diff);
                    if (diff != 0) allZeros = false;
                }

                histories.Add(newHistory);

                if (allZeros) break;
            }

            return histories;
        }
    }
}
