using AdventOfCode2022.Tools;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_11
    {
        public static void Solve()
        {
            AoCTools<Int64>.RunMeasureTimeAndLog(Part1, Part2, "11", testInput: false);
        }

        private static Int64 Part1(IEnumerable<string> input)
        {
            return CalculateSumOfShortestPathsBetweenGalaxies(input, universeExpansionRate: 2);
        }
        private static Int64 Part2(IEnumerable<string> input)
        {
            return CalculateSumOfShortestPathsBetweenGalaxies(input, universeExpansionRate: 1000000);
        }

        private const char GalaxyChar = '#';

        private static Int64 CalculateSumOfShortestPathsBetweenGalaxies(IEnumerable<string> input, int universeExpansionRate)
        {
            List<List<char>> universe = new();

            foreach (string s in input)
            {
                List<char> line = new();
                foreach (char c in s) line.Add(c);
                universe.Add(line);
            }

            List<int> rowExpansionIndexes = GetRowExpansionIndexes(universe);
            List<int> colExpansionIndexes = GetColExpansionIndexes(universe);

            List<Tuple<int, int>> galaxies = GetGalaxyLocations(universe);
            Int64 sumOfShortestPathsBetweenGalaxies = 0;

            for (int i = 0; i < galaxies.Count; i++)
            {
                for (int j = i + 1; j < galaxies.Count; j++)
                {
                    int x_min = int.Min(galaxies[i].Item1, galaxies[j].Item1);
                    int x_max = int.Max(galaxies[i].Item1, galaxies[j].Item1);

                    int y_min = int.Min(galaxies[i].Item2, galaxies[j].Item2);
                    int y_max = int.Max(galaxies[i].Item2, galaxies[j].Item2);

                    // Calculate in the row expansion count
                    int expandedRows = 0;
                    foreach (int expandedRowIndexes in rowExpansionIndexes)
                    {
                        if (expandedRowIndexes > x_min && expandedRowIndexes < x_max) expandedRows++;
                    }

                    // Calculate in the column expansion count
                    int expandedCols = 0;
                    foreach (int expandedColIndexes in colExpansionIndexes)
                    {
                        if (expandedColIndexes > y_min && expandedColIndexes < y_max) expandedCols++;
                    }

                    Int64 x_diff = x_max - x_min;
                    Int64 y_diff = y_max - y_min;

                    Int64 x_distance = (x_diff - expandedCols) + expandedCols * universeExpansionRate;
                    Int64 y_distance = (y_diff - expandedRows) + expandedRows * universeExpansionRate;

                    sumOfShortestPathsBetweenGalaxies += x_distance + y_distance;
                }
            }

            return sumOfShortestPathsBetweenGalaxies;
        }

        private static List<Tuple<int, int>> GetGalaxyLocations(List<List<char>> universe)
        {
            List<Tuple<int, int>> galaxyLocations = new();

            for (int row = 0; row < universe.Count; row++)
            {
                for (int col = 0; col < universe[0].Count; col++)
                {
                    if (universe[row][col] == GalaxyChar)
                    {
                        galaxyLocations.Add(new Tuple<int, int>(row, col));
                    }
                }
            }

            return galaxyLocations;
        }

        private static List<int> GetRowExpansionIndexes(List<List<char>> universe)
        {
            List<int> rowExpansionIndexes = new();

            for (int row = 0; row < universe.Count; row++)
            {
                if (!universe[row].Contains(GalaxyChar))
                {
                    rowExpansionIndexes.Add(row);
                }
            }

            return rowExpansionIndexes;
        }

        private static List<int> GetColExpansionIndexes(List<List<char>> universe)
        {
            List<int> colExpansionIndexes = new();

            for (int col = 0; col < universe[0].Count; col++)
            {
                bool columnIsEmpty = true;

                for (int row = 0; row < universe.Count; row++)
                {
                    if (universe[row][col] == GalaxyChar)
                    {
                        columnIsEmpty = false;
                        break;
                    }
                }

                if (columnIsEmpty)
                {
                    colExpansionIndexes.Add(col);
                }
            }

            return colExpansionIndexes;
        }
    }
}
