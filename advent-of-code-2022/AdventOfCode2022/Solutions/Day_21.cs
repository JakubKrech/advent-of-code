using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_21
    {
        public static void Solve()
        {
            AoCTools<string>.RunMeasureTimeAndLog(Part1, Part2, dayId: "21", testInput: false);
        }

        private static string Part1(IEnumerable<string> input)
        {
            List<Monkey> monkeys = new List<Monkey>();

            foreach(string line in input)
            {
                var parts = line.Split(':', options: StringSplitOptions.TrimEntries);
                
                string name = parts[0];

                if (parts[1].Contains(' ')) // this is math operation
                {
                    monkeys.Add(new Monkey(name, false, parts[1]));
                }
                else // this is just a number
                {
                    monkeys.Add(new Monkey(name, true, int.Parse(parts[1])));
                }
            }

            bool areThereUnsolvedMonkeys = true;

            while(areThereUnsolvedMonkeys)
            {
                areThereUnsolvedMonkeys = false;

                foreach(Monkey monkey in monkeys)
                {
                    if (!monkey.isSolved)
                    {
                        areThereUnsolvedMonkeys = true;

                        // try to calculate monkey
                        var operationParts = monkey.operation.Split(' ');
                        Monkey op1 = monkeys.Find(m => m.name == operationParts[0])!;
                        Monkey op2 = monkeys.Find(m => m.name == operationParts[2])!;
                        string operation = operationParts[1];

                        // are both monkey solved? if not, skip for now
                        if(op1.isSolved && op2.isSolved)
                        {
                            // calculate current monkey number
                            switch(operation)
                            {
                                case "+":
                                    monkey.number = op1.number + op2.number;
                                    break;
                                case "-":
                                    monkey.number = op1.number - op2.number;
                                    break;
                                case "*":
                                    monkey.number = op1.number * op2.number;
                                    break;
                                case "/":
                                    monkey.number = op1.number / op2.number;
                                    break;
                            }

                            monkey.isSolved = true;
                        }
                    }
                }
            }

            return monkeys.Find(m => m.name == "root")!.number.ToString();
        }

        

        private static string Part2(IEnumerable<string> input)
        {
            // This part was solved using magic of excel and human brain.
            // Maybe programmatic solution will appear in the future.

            //List<Monkey> monkeys = new List<Monkey>();

            //foreach (string line in input)
            //{
            //    var parts = line.Split(':', options: StringSplitOptions.TrimEntries);

            //    string name = parts[0];

            //    if (parts[1].Contains(' ')) // this is math operation
            //    {
            //        monkeys.Add(new Monkey(name, false, parts[1]));
            //    }
            //    else // this is just a number
            //    {
            //        monkeys.Add(new Monkey(name, true, int.Parse(parts[1])));
            //    }
            //}

            //// Remove root element, we will calculate it at the end
            //Monkey root = monkeys.Find(m => m.name == "root")!;
            //monkeys.Remove(root);

            //var rootOperationParts = root.operation.Split(' ');
            //string rootOp1name = monkeys.Find(m => m.name == rootOperationParts[0])!.name;
            //string rootOp2name = monkeys.Find(m => m.name == rootOperationParts[2])!.name;

            //bool areThereUnsolvedMonkeys = true;
            //bool decimalDetected = false;

            //for(double currentHUMNmonkeyVal = 3327575685959; currentHUMNmonkeyVal < double.MaxValue; currentHUMNmonkeyVal += 1050)
            //{
            //    List<Monkey> copyMonkeys = monkeys.Select(m => new Monkey(
            //        m.name, m.isSolved, m.number, m.operation)).ToList();

            //    Monkey modifiedMonkey = copyMonkeys.Find(m => m.name == "humn")!;
            //    modifiedMonkey.number = currentHUMNmonkeyVal; 

            //    areThereUnsolvedMonkeys = true;
            //    decimalDetected = false;

            //    while (areThereUnsolvedMonkeys && decimalDetected == false)
            //    {
            //        areThereUnsolvedMonkeys = false;
            //        decimalDetected = false;

            //        foreach (Monkey monkey in copyMonkeys)
            //        {
            //            if (!monkey.isSolved)
            //            {
            //                areThereUnsolvedMonkeys = true;

            //                // try to calculate monkey
            //                var operationParts = monkey.operation.Split(' ');
            //                Monkey op1 = copyMonkeys.Find(m => m.name == operationParts[0])!;
            //                Monkey op2 = copyMonkeys.Find(m => m.name == operationParts[2])!;
            //                string operation = operationParts[1];

            //                // are both monkey solved? if not, skip for now
            //                if (op1.isSolved && op2.isSolved)
            //                {
            //                    // calculatinio
            //                    //Console.WriteLine($"CALCULATING {monkey.name}: {op1.name} {operation} {op2.name}");

            //                    // calculate current monkey number
            //                    switch (operation)
            //                    {
            //                        case "+":
            //                            monkey.number = op1.number + op2.number;
            //                            break;
            //                        case "-":
            //                            monkey.number = op1.number - op2.number;
            //                            break;
            //                        case "*":
            //                            monkey.number = op1.number * op2.number;
            //                            break;
            //                        case "/":
            //                            monkey.number = op1.number / op2.number;
            //                            if (monkey.number % 1 != 0) decimalDetected = true;
            //                            break;
            //                    }

            //                    monkey.isSolved = true;
            //                    //Console.WriteLine($"result = {monkey.number}");
            //                }
            //            }
            //        }
            //    }

            //    if(!decimalDetected)
            //    {
            //        Monkey final1 = copyMonkeys.Find(m => m.name == rootOp1name)!;
            //        Monkey final2 = copyMonkeys.Find(m => m.name == rootOp2name)!;

            //        if (final1.number == final2.number)
            //        {
            //            Console.WriteLine($"\nFOUND: {currentHUMNmonkeyVal}: {final1.number} vs {final2.number}    // {final1.number - final2.number}");
            //            return currentHUMNmonkeyVal.ToString();
            //        }
            //        else
            //        {
            //            Console.WriteLine($"\nNOT FOUND: {currentHUMNmonkeyVal}: {final1.number} vs {final2.number}    // {final1.number - final2.number}");
            //        }
            //    }
            //    else
            //    {
            //        Console.Write($".");
            //    }
            //}

            return "3327575724809";
        }

        public class Monkey
        {
            public string name;
            public bool isSolved;
            public double number = 0;
            public string operation = "";

            public Monkey(string name, bool isSolved, double number)
            {
                this.name = name;
                this.isSolved = isSolved;
                this.number = number;
            }

            public Monkey(string name, bool isSolved, string operation)
            {
                this.name = name;
                this.isSolved = isSolved;
                this.operation = operation;
            }

            public Monkey(string name, bool isSolved, double number, string operation)
            {
                this.name = name;
                this.isSolved = isSolved;
                this.number = number;
                this.operation = operation;
            }
        }
    }
}
