using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_13
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "13", testInput: false);
        }

        public enum Result
        {
            False = 0,
            True = 1,
            Draw = 2
        }

        private static int Part1(IEnumerable<string> input)
        {
            List<string> lines = input.ToList();

            int ammountOfPacketPairs = (input.Count() / 3) + 1;
            int currentPacketPair = 0;
            int sumOfIndicesOfCorrectPairs = 0;

            while (currentPacketPair < ammountOfPacketPairs)
            {
                string firstPacket = lines[0 + currentPacketPair * 3];
                string secondPacket = lines[1 + currentPacketPair * 3];

                if(CompareTwoPackets(firstPacket, secondPacket) == Result.True)
                {
                    sumOfIndicesOfCorrectPairs += currentPacketPair + 1;
                }

                currentPacketPair++;
            }

            return sumOfIndicesOfCorrectPairs;
        }

        private static int Part2(IEnumerable<string> input)
        {
            string dividerPacketOne = "[[2]]";
            string dividerPacketTwo = "[[6]]";
            List<string> packets = new() { dividerPacketOne, dividerPacketTwo };

            foreach (string line in input)
            {
                if (line != "") packets.Add(line);
            }

            bool swapped = true;

            while (swapped)
            {
                swapped = false;

                for (int i = 0; i < packets.Count - 1; i++)
                {
                    if (CompareTwoPackets(packets[i], packets[i + 1]) == Result.False)
                    {
                        swapped = true;
                        (packets[i], packets[i + 1]) = (packets[i + 1], packets[i]);
                    }
                }
            }

            int decoderKey = 1;

            for (int i = 0; i < packets.Count; i++)
            {
                if (packets[i] == dividerPacketOne) decoderKey *= i + 1;
                else if (packets[i] == dividerPacketTwo) decoderKey *= i + 1;
            }

            return decoderKey;
        }

        public static Result CompareTwoPackets(string firstPacket, string secondPacket)
        {
            // Both are integers
            if(firstPacket.Contains(',') == false && secondPacket.Contains(',') == false)
            {
                if (firstPacket == string.Empty || firstPacket == "[]") return Result.True;
                else if (secondPacket == string.Empty || secondPacket == "[]") return Result.False;

                while (firstPacket.StartsWith('[') || secondPacket.StartsWith('[')) 
                {
                    if (firstPacket == string.Empty) return Result.True;
                    else if (secondPacket == string.Empty) return Result.False;

                    if(firstPacket.StartsWith('['))
                        firstPacket = firstPacket.Substring(1, firstPacket.Length - 2);
    
                    if(secondPacket.StartsWith('['))
                        secondPacket = secondPacket.Substring(1, secondPacket.Length - 2);

                    if (firstPacket == string.Empty) return Result.True;
                    else if (secondPacket == string.Empty) return Result.False;
                }

                if (int.Parse(firstPacket) == int.Parse(secondPacket)) return Result.Draw;

                return int.Parse(firstPacket) > int.Parse(secondPacket) ? Result.False : Result.True;
            }

            // One or both is list
            List<string> firstSublists = SplitIntoSublists(firstPacket);
            List<string> secondSublists = SplitIntoSublists(secondPacket);

            int firstLen = firstSublists.Count;
            int secondLen = secondSublists.Count;

            for(int i = 0; i < Math.Max(firstLen, secondLen); i++)
            {
                // If one of the sublists ran out of elements
                if (i == firstLen) return Result.True;
                else if (i == secondLen) return Result.False;

                Result res = CompareTwoPackets(firstSublists[i], secondSublists[i]);

                // If values equal, check next values
                if (res == Result.Draw)
                {
                    continue;
                }
                else
                {
                    return res;
                }
            }

            return Result.Draw;
        }

        private static List<string> SplitIntoSublists(string value)
        {
            if(value == "")
                return new List<string>();

            if (!value.Contains(','))
            {
                if (value.StartsWith('[') && value.EndsWith(']'))
                {
                    value = value.Substring(1, value.Length - 2);
                }

                return new List<string>() { value };
            }

            List<string> sublists = new();
            int unclosedBrackets = 0;
            int lastCommaIndex = 0;

            // search for first comma that splits strings containing equal number of [ and ]
            for (int i = 0; i < value.Length; i++)
            {
                if (value[i] == '[') unclosedBrackets++;
                else if (value[i] == ']') unclosedBrackets--;
                else if (value[i] == ',' && unclosedBrackets == 0)
                {
                    sublists.Add(value.Substring(lastCommaIndex, i - lastCommaIndex));
                    lastCommaIndex = i + 1;
                }
            }

            if (unclosedBrackets == 0 && lastCommaIndex == 0)
            {
                sublists.Add(value.Substring(1, value.Length - 2));
            }
            else
            {
                sublists.Add(value.Substring(lastCommaIndex, value.Length - lastCommaIndex));
            }

            return sublists;
        }
    }
}
