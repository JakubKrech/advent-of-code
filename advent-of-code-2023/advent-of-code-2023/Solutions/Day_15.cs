using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_15
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "15", testInput: true);
        }

        private static int Part1(IEnumerable<string> input)
        {
            int total = 0;

            List<string> parsed = input.First().Split(',').ToList();

            foreach(string step in parsed)
            {
                total += HASH(step);
            }

            return total;
        }

        private static int Part2(IEnumerable<string> input)
        {
            Dictionary<int, List<Tuple<string, string>>> boxes = new();

            string inputString = input.First();
            List<string> parsed = inputString.Split(',').ToList();

            foreach (string step in parsed)
            {
                bool operIsEqualSign = step.Contains('=');
                var data = step.Split('-', '=');

                string label = data[0];
                var hashValue = HASH(label);

                Tuple<string, string> dataTuple = new Tuple<string, string>(label, data[1]);

                if (!operIsEqualSign)
                {
                    if (boxes.ContainsKey(hashValue) && boxes[hashValue].Any(x => x.Item1 == dataTuple.Item1))
                    {
                        var index = boxes[hashValue].FindIndex(x => x.Item1 == label);
                        boxes[hashValue].RemoveAt(index);

                        if (boxes[hashValue].Count == 0) boxes.Remove(hashValue);
                    }
                }
                else if (operIsEqualSign)
                {
                    if (!boxes.ContainsKey(hashValue))
                    {
                        boxes[hashValue] = new List<Tuple<string, string>>() { dataTuple };
                    }
                    else if (boxes[hashValue].Any(x => x.Item1 == dataTuple.Item1))
                    {
                        var index = boxes[hashValue].FindIndex(x => x.Item1 == dataTuple.Item1);
                        boxes[hashValue][index] = dataTuple;
                    }
                    else
                    {
                        boxes[hashValue].Add(dataTuple);
                    }
                }
            }

            int totalFocusingPower = 0;

            foreach(var box in boxes)
            {
                for(int i = 0; i < box.Value.Count; i++)
                {
                    totalFocusingPower += (box.Key + 1) * (i + 1) * int.Parse(box.Value[i].Item2);
                }
            }

            return totalFocusingPower;
        }

        private static int HASH(string s)
        {
            int currentValue = 0;

            foreach (char c in s)
            {
                int asciiVal = (int)c;
                currentValue += asciiVal;
                currentValue *= 17;
                currentValue %= 256;
            }

            return currentValue;
        }
    }
}
