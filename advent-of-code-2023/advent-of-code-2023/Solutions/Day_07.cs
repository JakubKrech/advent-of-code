using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_07
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "07", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            int totalWinnings = 0;
            List<Tuple<string, int, int>> handBidPower = ParseHandBidPower(input, jokersEnabled: false);

            handBidPower.Sort(new HandComparer());

            for(int i = 0; i < handBidPower.Count; i++)
            {
                totalWinnings += (i + 1) * handBidPower[i].Item2;
            }

            return totalWinnings;
        }

        private static int Part2(IEnumerable<string> input)
        {
            // J card is now a Joker
            cardValues['J'] = 1;

            int totalWinnings = 0;
            List<Tuple<string, int, int>> handBidPower = ParseHandBidPower(input, jokersEnabled: true);

            handBidPower.Sort(new HandComparer());

            for (int i = 0; i < handBidPower.Count; i++)
            {
                totalWinnings += (i + 1) * handBidPower[i].Item2;
            }

            return totalWinnings;
        }

        private static List<Tuple<string, int, int>> ParseHandBidPower(IEnumerable<string> input, bool jokersEnabled)
        {
            List<Tuple<string, int, int>> handBidPower = new ();

            foreach (string s in input)
            {
                var data = s.Split().ToArray();
                string hand = data[0];
                int bid = int.Parse(data[1]);

                int power = GetHandPower(hand, jokersEnabled);
                handBidPower.Add(new Tuple<string, int, int>(hand, bid, power));
            }

            return handBidPower;
        }

        private static Dictionary<char, int> cardValues = new Dictionary<char, int>()
        {
            { '2', 2 },
            { '3', 3 },
            { '4', 4 },
            { '5', 5 },
            { '6', 6 },
            { '7', 7 },
            { '8', 8 },
            { '9', 9 },
            { 'T', 10 },
            { 'J', 11 },
            { 'Q', 12 },
            { 'K', 13 },
            { 'A', 14 }
        };

        private static int GetHandPower(string hand, bool jokersEnabled)
        {
            Dictionary<char, int> cards = new Dictionary<char, int>();
            int power = 0;

            foreach (char card in hand)
            {
                if (cards.TryGetValue(card, out int val)) cards[card] = val + 1;
                else cards[card] = 1;
            }

            // --- Hand powers ---
            // Five of a kind  = 7
            // Four of a kind  = 6
            // Full house      = 5
            // Three of a kind = 4
            // Two pair        = 3
            // One pair        = 2
            // High card       = 1

            cards.TryGetValue('J', out int jokerCount);

            // If jokers are disabled treat it as there are no jokers in the hand
            if(!jokersEnabled) jokerCount = 0;

            // High card
            if (cards.Count == 5) 
            {
                power = 1;

                if(jokerCount == 1)
                {
                    // Joker creates one pair
                    power = 2;
                }
            }
            // One pair
            else if(cards.Count == 4)
            {
                power = 2;

                if (jokerCount >= 1)
                {
                    // One or two jokers create three of a kind
                    power = 4;
                }
            }
            else if (cards.Count == 3)
            {
                // Three of a kind
                if(cards.Any(x => x.Value == 3))
                {
                    power = 4;

                    if (jokerCount > 0)
                    {
                        // Joker creates one four of a kind
                        power = 6;
                    }
                }
                else // Two pair
                {
                    power = 3;

                    if (jokerCount == 1)
                    {
                        // Joker creates full house
                        power = 5;
                    }
                    else if (jokerCount == 2)
                    {
                        // Joker creates FOUR of a kind
                        power = 6;
                    }
                }
            }
            else if (cards.Count == 2)
            {
                // Four of a kind
                if(cards.Any(x => x.Value == 4))
                {
                    power = 6;

                    if (jokerCount > 0)
                    {
                        // Joker creates five of a kind
                        power = 7;
                    }
                }
                else // Full house (three of a kind + two of a kind)
                {
                    power = 5;

                    if (jokerCount > 0)
                    {
                        // Two or three jokers create five of a kind
                        power = 7;
                    }
                }
            }
            // Five of a kind
            else if (cards.Count == 1)
            {
                power = 7;

                // Joker cant improve the score
            }

            return power;
        }

        private class HandComparer : IComparer<Tuple<string, int, int>>
        {
            // Compare by 3rd tuple elements (power), if powers are equal compare
            // by 1st tuple element (hand) by comparing successive cards values.
            int IComparer<Tuple<string, int, int>>.Compare(Tuple<string, int, int>? x, Tuple<string, int, int>? y)
            {
                if (x.Item3 > y.Item3) return 1;
                else if (x.Item3 < y.Item3) return -1;
                else
                {
                    for (int i = 0; i < x.Item1.Length; i++)
                    {
                        if (cardValues[x.Item1[i]] > cardValues[y.Item1[i]])
                        {
                            return 1;
                        }
                        else if (cardValues[x.Item1[i]] < cardValues[y.Item1[i]])
                        {
                            return -1;
                        }
                    }
                    return 1;
                }
            }
        }
    }
}
