namespace AdventOfCode2022.Tools
{
    public static class InputData
    {
        private static readonly string basePath = Directory.GetParent(Environment.CurrentDirectory).Parent.Parent.FullName;

        public static IEnumerable<string> ReadInput(string fileName)
        {
            return File.ReadLines(Path.Combine(basePath, $"Input\\{fileName}"));
        }

        public static IEnumerable<string> ReadTestInput(string fileName)
        {
            return File.ReadLines(Path.Combine(basePath, $"TestInput\\{fileName}"));
        }
    }
}
