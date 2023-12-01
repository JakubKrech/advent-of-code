using System.Text;
using System.Text.RegularExpressions;

using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_05
    {
        public static void Solve()
        {
            AoCTools<string>.RunMeasureTimeAndLog(Part1, Part2, "05", testInput: false);
        }

        public static string Part1(IEnumerable<string> input) => Solution(input, moveMultipleAtOnce: false);
        public static string Part2(IEnumerable<string> input) => Solution(input, moveMultipleAtOnce: true);

        private static string Solution(IEnumerable<string> input, bool moveMultipleAtOnce)
        {
            List<Stack<string>> stacks = new();
            
            List<string> stackInputLines = new();
            List<string> movesInputLines = new();

            bool emptyLineReached = false;

            // Split input data between initial stack data and crates movement data
            foreach(string line in input)
            {
                if(line == "")
                {
                    emptyLineReached = true;
                    continue;
                }

                if (!emptyLineReached) stackInputLines.Add(line);
                else movesInputLines.Add(line);
            }

            // Initialize stacks
            var stackNumbers = stackInputLines.Last<string>().Split(' ', StringSplitOptions.RemoveEmptyEntries);
            int numberOfStacks = int.Parse(stackNumbers.Last());

            for (int i = 0; i < numberOfStacks; i++)
            {
                stacks.Add(new Stack<string>());
            }

            string regexPattern = @"(\[[A-Z]\]|[ ]{3}).?";
            Regex regex = new(regexPattern);

            for (int i = stackInputLines.Count - 2; i >= 0; i--)
            {
                MatchCollection matches = regex.Matches(stackInputLines[i]);
                
                for(int j = 0; j < numberOfStacks; j++)
                {
                    if(!string.IsNullOrWhiteSpace(matches[j].Value))
                    {
                        stacks[j].Push(matches[j].Value.Substring(1,1));
                    }
                }
            }

            // Move crates around
            regexPattern = "move (\\d+) from (\\d+) to (\\d+)";
            Regex regex2 = new(regexPattern);

            foreach(var moveCommand in movesInputLines)
            {
                Match match = regex2.Match(moveCommand);
                int howMany = int.Parse(match.Groups[1].Value);
                int from = int.Parse(match.Groups[2].Value);
                int to = int.Parse(match.Groups[3].Value);
                string value;

                if (!moveMultipleAtOnce)
                {
                    for (int i = 0; i < howMany; i++)
                    {
                        value = stacks[from - 1].Pop();
                        stacks[to - 1].Push(value);
                    }
                }
                else
                {
                    Stack<string> intermediateStorage = new();

                    for (int i = 0; i < howMany; i++)
                    {
                        value = stacks[from - 1].Pop();
                        intermediateStorage.Push(value);
                    }

                    for (int i = 0; i < howMany; i++)
                    {
                        value = intermediateStorage.Pop();
                        stacks[to - 1].Push(value);
                    }
                }
            }


            // Print top elements
            StringBuilder sb = new();

            foreach (var stack in stacks)
            {
                sb.Append(stack.Peek());
            }

            return sb.ToString();
        }
    }
}
