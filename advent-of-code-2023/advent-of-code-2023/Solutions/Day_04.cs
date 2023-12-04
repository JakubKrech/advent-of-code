using AdventOfCode2022.Tools;
using System.Text.RegularExpressions;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_04
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "04", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            int totalReward = 0;

            foreach (string line in input)
            {
                (var winning_numbers, var player_numbers) = ParseLineData(line);
                int matching_count = player_numbers.Where(winning_numbers.Contains).Count();
                totalReward += (int)Math.Pow(2, matching_count - 1);
            }

            return totalReward;
        }

        private static int Part2(IEnumerable<string> input)
        {
            int cardNumber;

            // dictionary holds: Card Number as key, ammount of owned cards of that number as value
            Dictionary<int, int> ownedCards = new Dictionary<int, int>();
            for(cardNumber = 1; cardNumber <= input.Count(); cardNumber++)
            {
                ownedCards[cardNumber] = 1; // at the start player owns one of each card
            }

            cardNumber = 1;
            foreach (string line in input)
            {
                (var winning_numbers, var player_numbers) = ParseLineData(line);

                int matching_count = player_numbers.Where(x => winning_numbers.Contains(x)).Count();

                int addingCardIdIncrement = 1;
                while(addingCardIdIncrement <= matching_count)
                {
                    ownedCards[cardNumber + addingCardIdIncrement] += ownedCards[cardNumber];
                    addingCardIdIncrement++;
                }

                cardNumber++;
            }

            return ownedCards.Sum(x => x.Value);
        }

        private static Tuple<List<int>, List<int>> ParseLineData(string line)
        {
            string pattern = @"Card +\d+: +([\d ]*) \| +([\d ]*)";
            Match match = Regex.Match(line, pattern);

            var winning_numbers = match.Groups[1].Value.Split().Where(n => n != string.Empty).Select(int.Parse).ToList();
            var player_numbers = match.Groups[2].Value.Split().Where(n => n != string.Empty).Select(int.Parse).ToList();

            return Tuple.Create(winning_numbers, player_numbers);
        }
    }
}
