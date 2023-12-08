using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_03
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "03", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            List<List<char>> data = Get2DListWithData(input);
            int sumOfPartNumbers = 0;

            int width = data[0].Count;
            int height = data.Count;

            // Detect the numbers in lines
            for (int x = 0; x < height; x++)
            {
                int beginning = -1;
                int ending = -1;

                for (int y = 0; y < width; y++)
                {
                    if (char.IsDigit(data[x][y]))
                    {
                        if (beginning == -1) beginning = y;
                        ending = y;
                    }
                    else
                    {
                        if(beginning != -1 && ending != -1)
                        {
                            // Call the neighbor checking function
                            bool isValid = IsPartNumberValid(data, x, beginning, ending);

                            if(isValid)
                            {
                                string number = string.Empty;

                                for(int z = beginning; z <= ending; z++)
                                {
                                    number += data[x][z];
                                }

                                sumOfPartNumbers += int.Parse(number);
                            }
                        }

                        beginning = -1;
                        ending = -1;
                    }
                }
            }

            return sumOfPartNumbers;
        }

        private static int Part2(IEnumerable<string> input)
        {
            List<List<char>> data = Get2DListWithData(input);
            int sumOfGearRatios = 0;

            for (int x = 0; x < data.Count; x++)
            {
                for (int y = 0; y < data[x].Count; y++)
                {
                    if (data[x][y] == '*')
                    {
                        sumOfGearRatios += GetGearRatio(Get2DListWithData(input), x, y);
                    }
                }
            }

            return sumOfGearRatios;
        }

        private static List<List<char>> Get2DListWithData(IEnumerable<string> input)
        {
            List<List<char>> data = new List<List<char>>();

            foreach (string line in input)
            {
                List<char> splitLine = new List<char>();

                foreach (char c in line)
                {
                    splitLine.Add(c);
                }

                splitLine.Add('.'); // add padding on the right side

                data.Add(splitLine);
            }

            return data;
        }

        private static bool IsPartNumberValid(List<List<char>> data, int row, int beginning, int ending)
        {
            int width = data[0].Count;
            int height = data.Count;

            // Check top row
            if(row - 1 >= 0)
            {
                for(int x = beginning; x <= ending; x++)
                {
                    if (data[row - 1][x] != '.' && !char.IsDigit(data[row - 1][x]))
                    {
                        return true;
                    }
                }
            }

            // Check bottom row
            if (row + 1 < height)
            {
                for (int x = beginning; x <= ending; x++)
                {
                    if (data[row + 1][x] != '.' && !char.IsDigit(data[row + 1][x]))
                    {
                        return true;
                    }
                }
            }

            // Check left end
            if (beginning - 1 >= 0)
            {
                if (data[row][beginning - 1] != '.' && !char.IsDigit(data[row][beginning - 1]))
                {
                    return true;
                }
            }

            // Check right end
            if (ending + 1 < width)
            {
                if (data[row][ending + 1] != '.' && !char.IsDigit(data[row][ending + 1]))
                {
                    return true;
                }
            }

            // Check left-top corner
            if (row - 1 >= 0 && beginning - 1 >= 0)
            {
                if (data[row - 1][beginning - 1] != '.' && !char.IsDigit(data[row - 1][beginning - 1]))
                {
                    return true;
                }
            }

            // Check right-top corner
            if (row - 1 >= 0 && ending + 1 < width)
            {
                if (data[row - 1][ending + 1] != '.' && !char.IsDigit(data[row - 1][ending + 1]))
                {
                    return true;
                }
            }

            // Check left-bottom corner
            if (row + 1 < height && beginning - 1 >= 0)
            {
                if (data[row + 1][beginning - 1] != '.' && !char.IsDigit(data[row + 1][beginning - 1]))
                {
                    return true;
                }
            }


            // Check right-bottom corner
            if (row + 1 < height && ending + 1 < width)
            {
                if (data[row + 1][ending + 1] != '.' && !char.IsDigit(data[row + 1][ending + 1]))
                {
                    return true;
                }
            }

            return false;
        }
    
        private static int GetGearRatio(List<List<char>> data, int gearX, int gearY)
        {
            List<int> numbers = new List<int>();
            int width = data[0].Count;
            int height = data.Count;

            // top
            if (gearX - 1 >= 0)
            {
                if (char.IsDigit(data[gearX - 1][gearY])) 
                    numbers.Add(GetTheNumberAndEraseIt(data, gearX - 1, gearY));
            }

            // bot
            if (gearX + 1 < height)
            {
                if (char.IsDigit(data[gearX + 1][gearY]))
                    numbers.Add(GetTheNumberAndEraseIt(data, gearX + 1, gearY));
            }

            // left
            if (gearY - 1 >= 0)
            {
                if (char.IsDigit(data[gearX][gearY - 1]))
                    numbers.Add(GetTheNumberAndEraseIt(data, gearX, gearY - 1));
            }

            // right
            if (gearY + 1 < width)
            {
                if (char.IsDigit(data[gearX][gearY + 1]))
                    numbers.Add(GetTheNumberAndEraseIt(data, gearX, gearY + 1));
            }

            // top-left
            if (gearX - 1 >= 0 && gearY - 1 >= 0)
            {
                if (char.IsDigit(data[gearX - 1][gearY - 1]))
                    numbers.Add(GetTheNumberAndEraseIt(data, gearX - 1, gearY - 1));
            }

            // top-right
            if (gearX - 1 >= 0 && gearY + 1 < width)
            {
                if (char.IsDigit(data[gearX - 1][gearY + 1]))
                    numbers.Add(GetTheNumberAndEraseIt(data, gearX - 1, gearY + 1));
            }

            // bot-left
            if (gearX + 1 < height && gearY - 1 >= 0)
            {
                if (char.IsDigit(data[gearX + 1][gearY - 1]))
                    numbers.Add(GetTheNumberAndEraseIt(data, gearX + 1, gearY - 1));
            }

            // bot-right
            if (gearX + 1 < height && gearY + 1 < width)
            {
                if (char.IsDigit(data[gearX + 1][gearY + 1]))
                    numbers.Add(GetTheNumberAndEraseIt(data, gearX + 1, gearY + 1));
            }

            if(numbers.Count >= 2)
            {
                return numbers[0] * numbers[1];
            }

            return 0;
        }

        private static int GetTheNumberAndEraseIt(List<List<char>> data, int x, int y)
        {
            int width = data[0].Count;
            string numberStr = string.Empty;

            // Go to the left to beginning of the number
            while(y - 1 >= 0)
            {
                if (char.IsDigit(data[x][y - 1]))
                {
                    y--;
                }
                else
                {
                    break;
                }
            }

            // Go to the right to ending of the number while adding chars to number string
            while(y + 1 < width)
            {
                if (char.IsDigit(data[x][y]))
                {
                    numberStr += data[x][y];
                    data[x][y] = '='; // Override the number so its not detected multiple times
                    y++;
                }
                else
                {
                    break;
                }
            }

            return int.Parse(numberStr);
        }
    }
}
