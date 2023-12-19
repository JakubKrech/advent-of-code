using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_19
    {
        public static void Solve()
        {
            AoCTools<Int64>.RunMeasureTimeAndLog(Part1, Part2, "19", testInput: false);
        }

        private static Dictionary<string, List<string>> workflows = new();

        private static Int64 Part1(IEnumerable<string> input)
        {
            // x, m, a, s
            List<List<int>> objects = new();

            int totalAcceptedRatingNumbers = 0;

            // Read workflows and objects from input
            bool readingWorkflows = true;
            foreach(string line in input.ToList())
            {
                if (line == string.Empty)
                {
                    readingWorkflows = false;
                    continue;
                }

                if(readingWorkflows)
                {
                    var data = line.Substring(0, line.Length - 1).Split('{');
                    workflows[data[0]] = data[1].Split(",").ToList();
                }
                else
                {
                    objects.Add(line.Substring(1, line.Length - 2).Split(',').Select(x => int.Parse(x.Split('=')[1])).ToList());
                }
            }

            // Process objects
            foreach(var obj in objects)
            {
                string currentWorkflow = "in";

                int x = obj[0];
                int m = obj[1];
                int a = obj[2];
                int s = obj[3];

                while (currentWorkflow != "A" && currentWorkflow != "R")
                {
                    var workflowRules = workflows[currentWorkflow];

                    foreach(var rule in workflowRules)
                    {
                        if(rule.Contains(':'))
                        {
                            // rating, value, nextWorkflow
                            var ruleParts = rule.Split('<', '>', ':');
                            int value = int.Parse(ruleParts[1]);
                            string nextWorkflow = currentWorkflow;

                            if(rule.Contains('>'))
                            {
                                switch(ruleParts[0])
                                {
                                    case "x":
                                        if (x > value) nextWorkflow = ruleParts[2];
                                        break;
                                    case "m":
                                        if (m > value) nextWorkflow = ruleParts[2];
                                        break;
                                    case "a":
                                        if (a > value) nextWorkflow = ruleParts[2];
                                        break;
                                    case "s":
                                        if (s > value) nextWorkflow = ruleParts[2];
                                        break;
                                }
                            }
                            else // rule.Contains('<')
                            {
                                switch (ruleParts[0])
                                {
                                    case "x":
                                        if (x < value) nextWorkflow = ruleParts[2];
                                        break;
                                    case "m":
                                        if (m < value) nextWorkflow = ruleParts[2];
                                        break;
                                    case "a":
                                        if (a < value) nextWorkflow = ruleParts[2];
                                        break;
                                    case "s":
                                        if (s < value) nextWorkflow = ruleParts[2];
                                        break;
                                }
                            }

                            if(currentWorkflow != nextWorkflow)
                            {
                                currentWorkflow = nextWorkflow;
                                break;
                            }
                        }
                        else
                        {
                            // Just move to default next workflow
                            currentWorkflow = rule;
                        }
                    }
                }

                if(currentWorkflow == "A") // Element is accepted
                {
                    totalAcceptedRatingNumbers += x + m + a + s;
                }
            }

            return totalAcceptedRatingNumbers;
        }

        private static List<List<Tuple<int, int>>> succesfullRuns = new();

        private static Int64 Part2(IEnumerable<string> input)
        {
            RecurrentWalk("in", new Tuple<int, int>(1, 4000), new Tuple<int, int>(1, 4000), new Tuple<int, int>(1, 4000), new Tuple<int, int>(1, 4000));

            Int64 totalResult = 0;

            foreach(var success in succesfullRuns)
            {
                Int64 successResult = 1;

                var x = success[0];
                var m = success[1];
                var a = success[2];
                var s = success[3];

                successResult *= (x.Item2 - x.Item1 + 1);
                successResult *= (m.Item2 - m.Item1 + 1);
                successResult *= (a.Item2 - a.Item1 + 1);
                successResult *= (s.Item2 - s.Item1 + 1);

                totalResult += successResult;
            }

            return totalResult;
        }

        private static void RecurrentWalk(string workflowHistory, Tuple<int, int> x, Tuple<int, int> m, Tuple<int, int> a, Tuple<int, int> s)
        {
            var currentWorkflow = workflowHistory.Split(":").Last();
            List<Tuple<int, int>> resultBounds = new();

            Tuple<int, int> new_x = new Tuple<int, int>(x.Item1, x.Item2);
            Tuple<int, int> new_m = new Tuple<int, int>(m.Item1, m.Item2);
            Tuple<int, int> new_a = new Tuple<int, int>(a.Item1, a.Item2);
            Tuple<int, int> new_s = new Tuple<int, int>(s.Item1, s.Item2);

            if (currentWorkflow == "A")
            {
                resultBounds.Add(new_x);
                resultBounds.Add(new_m);
                resultBounds.Add(new_a);
                resultBounds.Add(new_s);

                succesfullRuns.Add(resultBounds);
                return;
            }

            var workflowRules = workflows[currentWorkflow];

            for (int i = 0; i < workflowRules.Count - 1; i++)
            {
                var rule = workflowRules[i];

                Tuple<int, int> recurr_x = new Tuple<int, int>(new_x.Item1, new_x.Item2);
                Tuple<int, int> recurr_m = new Tuple<int, int>(new_m.Item1, new_m.Item2);
                Tuple<int, int> recurr_a = new Tuple<int, int>(new_a.Item1, new_a.Item2);
                Tuple<int, int> recurr_s = new Tuple<int, int>(new_s.Item1, new_s.Item2);

                // rating, value, nextWorkflow
                var ruleParts = rule.Split('<', '>', ':');
                int value = int.Parse(ruleParts[1]);

                if (rule.Contains('>'))
                {
                    // Update recurr values for the recurrent call
                    if (ruleParts[0] == "x") recurr_x = new Tuple<int, int>(int.Max(recurr_x.Item1, value + 1), recurr_x.Item2);
                    else if (ruleParts[0] == "m") recurr_m = new Tuple<int, int>(int.Max(recurr_m.Item1, value + 1), recurr_m.Item2);
                    else if (ruleParts[0] == "a") recurr_a = new Tuple<int, int>(int.Max(recurr_a.Item1, value + 1), recurr_a.Item2);
                    else if (ruleParts[0] == "s") recurr_s = new Tuple<int, int>(int.Max(recurr_s.Item1, value + 1), recurr_s.Item2);

                    // Update values for next rule analysis
                    if (ruleParts[0] == "x") new_x = new Tuple<int, int>(new_x.Item1, int.Min(new_x.Item2, value));
                    else if (ruleParts[0] == "m") new_m = new Tuple<int, int>(new_m.Item1, int.Min(new_m.Item2, value));
                    else if (ruleParts[0] == "a") new_a = new Tuple<int, int>(new_a.Item1, int.Min(new_a.Item2, value));
                    else if (ruleParts[0] == "s") new_s = new Tuple<int, int>(new_s.Item1, int.Min(new_s.Item2, value));
                }
                else // rule.Contains('<')
                {
                    // Update recurr values for the recurrent call
                    if (ruleParts[0] == "x") recurr_x = new Tuple<int, int>(recurr_x.Item1, int.Min(recurr_x.Item2, value - 1));
                    else if (ruleParts[0] == "m") recurr_m = new Tuple<int, int>(recurr_m.Item1, int.Min(recurr_m.Item2, value - 1));
                    else if (ruleParts[0] == "a") recurr_a = new Tuple<int, int>(recurr_a.Item1, int.Min(recurr_a.Item2, value - 1));
                    else if (ruleParts[0] == "s") recurr_s = new Tuple<int, int>(recurr_s.Item1, int.Min(recurr_s.Item2, value - 1));

                    // Update values for next rule analysis
                    if (ruleParts[0] == "x") new_x = new Tuple<int, int>(int.Max(new_x.Item1, value), new_x.Item2);
                    else if (ruleParts[0] == "m") new_m = new Tuple<int, int>(int.Max(new_m.Item1, value), new_m.Item2);
                    else if (ruleParts[0] == "a") new_a = new Tuple<int, int>(int.Max(new_a.Item1, value), new_a.Item2);
                    else if (ruleParts[0] == "s") new_s = new Tuple<int, int>(int.Max(new_s.Item1, value), new_s.Item2);
                }

                if (ruleParts[2] == "R") continue;

                RecurrentWalk(workflowHistory + ":" + ruleParts[2], recurr_x, recurr_m, recurr_a, recurr_s);
            }

            var lastRule = workflowRules[workflowRules.Count - 1];

            if (lastRule != "R") RecurrentWalk(workflowHistory + ":" + lastRule, new_x, new_m, new_a, new_s);
        }
    }
}
