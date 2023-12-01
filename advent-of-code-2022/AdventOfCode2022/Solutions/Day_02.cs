using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_02
    {
        const int rockShown = 1;
        const int paperShown = 2;
        const int scissorsShown = 3;

        const int winPoints = 6;
        const int drawPoints = 3;
        const int losePoints = 0;

        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "02", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            // rock = 1p, paper = 2p, scissors = 3p
            // win = 6p, draw = 3p, lose = 0p
            int totalPoints = 0;

            foreach (var pair in input)
            {
                string enemy = pair.Split(' ')[0];
                string me = pair.Split(' ')[1];

                switch (me)
                {
                    case "X": // rock
                        totalPoints += rockShown + SolveDuel(enemy, me);
                        break;
                    case "Y": // paper
                        totalPoints += paperShown + SolveDuel(enemy, me);
                        break;
                    case "Z": // scissors
                        totalPoints += scissorsShown + SolveDuel(enemy, me);
                        break;
                }
            }

            return totalPoints;
        }

        private static int Part2(IEnumerable<string> input)
        {
            // rock = 1p, paper = 2p, scissors = 3p
            // win = 6p, draw = 3p, lose = 0p
            int totalPoints = 0;

            foreach (var pair in input)
            {
                string enemy = pair.Split(' ')[0];
                string resolution = pair.Split(' ')[1];
                string me = FindCorrectMove(enemy, resolution);

                switch (me)
                {
                    case "X": // rock
                        totalPoints += rockShown + SolveDuel(enemy, me);
                        break;
                    case "Y": // paper
                        totalPoints += paperShown + SolveDuel(enemy, me);
                        break;
                    case "Z": // scissors
                        totalPoints += scissorsShown + SolveDuel(enemy, me);
                        break;
                }
            }

            return totalPoints;
        }

        private static int SolveDuel(string enemy, string me)
        {
            switch (enemy)
            {
                case "A": // rock
                    return me == "X" ? drawPoints : me == "Y" ? winPoints : losePoints;
                case "B": // paper
                    return me == "Y" ? drawPoints : me == "Z" ? winPoints : losePoints;
                default:  // case "C": // scissors
                    return me == "Z" ? drawPoints : me == "X" ? winPoints : losePoints;
            }
        }

        private static string FindCorrectMove(string enemy, string resolution)
        {
            switch (resolution)
            {
                case "X": // need to lose
                    return enemy == "A" ? "Z" : enemy == "B" ? "X" : "Y";
                case "Y": // need to draw
                    return enemy == "A" ? "X" : enemy == "B" ? "Y" : "Z";
                default:  // case "Z": // need to win
                    return enemy == "A" ? "Y" : enemy == "B" ? "Z" : "X";
            }
        }
    }
}
