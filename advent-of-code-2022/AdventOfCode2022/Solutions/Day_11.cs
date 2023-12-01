using AdventOfCode2022.Tools;
using System.Reflection;
using System.Text.RegularExpressions;
using static AdventOfCode2022.Solutions.Day_11;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_11
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "11", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            List<Monkey> monkeys = ParseInputIntoMonkeyList(input);
            int currentRound = 0;

            while(currentRound < 20)
            {
                currentRound++;

                foreach (Monkey monkey in monkeys) 
                {
                    while (monkey.items.Count> 0)
                    {
                        monkey.inspectionsNumber++;

                        int worryLevel = monkey.items.Dequeue();
                        int modifyingValue = monkey.operationValue == "old" ? worryLevel : int.Parse(monkey.operationValue);

                        if(monkey.operationType == '+')
                        {
                            worryLevel += modifyingValue;
                        }
                        else if (monkey.operationType == '*')
                        {
                            worryLevel *= modifyingValue;
                        }

                        worryLevel /= 3;

                        bool divisible = worryLevel % monkey.divisibleBy == 0 ? true : false;
                        int newMonkey = divisible ? monkey.idIfTrue : monkey.idIfFalse;

                        monkeys[newMonkey].items.Enqueue(worryLevel);
                    }
                }
            }

            List<int> inspections = new();
            foreach(Monkey monkey in monkeys)
            {
                inspections.Add(monkey.inspectionsNumber);
            }
            inspections.Sort();

            return inspections[inspections.Count - 1] * inspections[inspections.Count - 2];
        }

        private static int Part2(IEnumerable<string> input)
        {
            return -1;

            int numberOfMonkeys = (input.Count() / 7) + 1;
            List<Monkey> monkeys = ParseInputIntoMonkeyList(input);

            int currentRound = 0;

            PrintMonkeyItems(monkeys, currentRound);

            while (currentRound < 10000)
            {
                currentRound++;

                foreach (Monkey monkey in monkeys)
                {
                    // Console.WriteLine($"Monkey {monkey.id}:");

                    while (monkey.items.Count > 0)
                    {
                        monkey.inspectionsNumber++;

                        int worryLevel = monkey.items.Dequeue();
                        // Console.WriteLine($"  Monkey inspects an item with a worry level of {worryLevel}.");
                        int modifyingValue = monkey.operationValue == "old" ? worryLevel : int.Parse(monkey.operationValue);

                        if (monkey.operationType == '+')
                        {
                            worryLevel += modifyingValue;
                            // Console.WriteLine($"    Worry level increases by {modifyingValue} to {worryLevel}.");
                        }
                        else if (monkey.operationType == '*')
                        {
                            worryLevel *= modifyingValue;
                            // Console.WriteLine($"    Worry level is multiplied by {modifyingValue} to {worryLevel}.");
                        }

                        bool divisible = worryLevel % monkey.divisibleBy == 0 ? true : false;
                        //if (divisible) // Console.WriteLine($"    Current worry level is divisible by {monkey.divisibleBy}.");
                        //else // Console.WriteLine($"    Current worry level is not divisible by {monkey.divisibleBy}.");

                        int newMonkey = divisible ? monkey.idIfTrue : monkey.idIfFalse;
                        // Console.WriteLine($"    Item with worry level {worryLevel} is thrown to monkey {newMonkey}.");

                        monkeys[newMonkey].items.Enqueue(worryLevel);
                    }
                }

                PrintMonkeyItems(monkeys, currentRound);
            }

            List<int> inspections = new();
            foreach (Monkey monkey in monkeys)
            {
                // Console.WriteLine($"Monkey {monkey.id} inspected items {monkey.inspectionsNumber} times.");
                inspections.Add(monkey.inspectionsNumber);
            }
            inspections.Sort();

            return inspections[inspections.Count - 1] * inspections[inspections.Count - 2];
        }

        public class Monkey
        {
            public int id;
            public Queue<int> items;
            public char operationType;
            public string operationValue;

            public int divisibleBy;

            public int idIfTrue;
            public int idIfFalse;

            public int inspectionsNumber = 0;

            public Monkey(int id, Queue<int> items, char op, string opV, int divBy, int ifTrue, int ifFalse)
            {
                this.id = id;
                this.items = items;
                this.operationType = op;
                this.operationValue = opV;
                this.divisibleBy = divBy;
                this.idIfTrue = ifTrue;
                this.idIfFalse = ifFalse;

                //Console.Write($"Adding new monkey ({id}): items[ ");
                //foreach (int item in this.items) Console.Write($"{item} ");
                // Console.WriteLine($"], {operationType} {operationValue}, / {divisibleBy}, true ? {ifTrue} : {ifFalse}");
            }
        }

        public static List<Monkey> ParseInputIntoMonkeyList(IEnumerable<string> input)
        {
            int numberOfMonkeys = (input.Count() / 7) + 1;
            List<Monkey> monkeys = new();

            Regex regex0 = new(@"Monkey (\d+):");
            Regex regex1 = new(@"\b\d+\b");
            Regex regex2 = new(@"Operation: new = old ([\+\-\*\/]) (\d+|old)");
            Regex regex3 = new(@"Test: divisible by (\d+)");
            Regex regex4 = new(@"If true: throw to monkey (\d+)");
            Regex regex5 = new(@"If false: throw to monkey (\d+)");

            for (int i = 0; i < numberOfMonkeys; i++)
            {
                Queue<int> startingItems = new();

                int id = int.Parse(regex0.Match(input.ElementAt(i * 7 + 0)).Groups[1].ToString());

                foreach (Match match in regex1.Matches(input.ElementAt(i * 7 + 1)))
                {
                    startingItems.Enqueue(int.Parse(match.Value));
                }

                char operation = char.Parse(regex2.Match(input.ElementAt(i * 7 + 2)).Groups[1].ToString());
                string opVal = regex2.Match(input.ElementAt(i * 7 + 2)).Groups[2].ToString();

                int divisibleBy = int.Parse(regex3.Match(input.ElementAt(i * 7 + 3)).Groups[1].ToString());

                int ifTrue = int.Parse(regex4.Match(input.ElementAt(i * 7 + 4)).Groups[1].ToString());
                int ifFalse = int.Parse(regex5.Match(input.ElementAt(i * 7 + 5)).Groups[1].ToString());

                Monkey mon = new Monkey(id, startingItems, operation, opVal, divisibleBy, ifTrue, ifFalse);

                monkeys.Add(mon);
            }

            return monkeys;
        }

        public static void PrintMonkeyItems(List<Monkey> monkeys, int round)
        {
            Console.WriteLine($"\nAfter round {round}:");
            foreach(Monkey monkey in monkeys)
            {
                Console.Write($"Monkey {monkey.id}: ");
                foreach(var item in monkey.items) Console.Write($"{item} ");
                Console.WriteLine();
            }
            Console.WriteLine();
        }
    }
}
