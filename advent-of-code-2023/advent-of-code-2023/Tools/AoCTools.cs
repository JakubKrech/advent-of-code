namespace AdventOfCode2022.Tools
{
    internal class AoCTools<T, P>
    {
        public static void RunMeasureTimeAndLog(Func<IEnumerable<string>, T> part1, Func<IEnumerable<string>, P> part2, string dayId, bool testInput)
        {
            var input = testInput ? InputData.ReadTestInput($"Day_{dayId}.txt") : InputData.ReadInput($"Day_{dayId}.txt");

            var watch = System.Diagnostics.Stopwatch.StartNew();
            var p1 = part1(input);
            watch.Stop(); var part1ElapsedMs = watch.ElapsedMilliseconds; watch.Start();
            var p2 = part2(input);
            var part2ElapsedMs = watch.ElapsedMilliseconds;

            Console.WriteLine($"-~= Day {dayId} =~-  {part1ElapsedMs + part2ElapsedMs}ms ({part1ElapsedMs}ms | {part2ElapsedMs}ms)");
            Console.WriteLine($"Part 1: {p1}");
            Console.WriteLine($"Part 2: {p2}\n");
        }
    }

    internal class AoCTools<T>
    {
        public static void RunMeasureTimeAndLog(Func<IEnumerable<string>, T> part1, Func<IEnumerable<string>, T> part2, string dayId, bool testInput)
        {
            AoCTools<T, T>.RunMeasureTimeAndLog(part1, part2, dayId, testInput);
        }
    }

    internal class AoCTools
    {
        public static void RunMeasureTimeAndLog(Func<IEnumerable<string>, int> part1, Func<IEnumerable<string>, int> part2, string dayId, bool testInput)
        {
            AoCTools<int, int>.RunMeasureTimeAndLog(part1, part2, dayId, testInput);
        }
    }
}