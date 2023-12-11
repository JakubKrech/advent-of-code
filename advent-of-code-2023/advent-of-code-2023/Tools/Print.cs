namespace advent_of_code_2023.Tools
{
    using System.Text;

    internal static class Print
    {
        public static void Print2dList(List<List<char>> data)
        {
            StringBuilder myString = new();
            foreach (var line in data)
            {
                foreach (char c in line)
                {
                    myString.Append(c);
                }
                myString.Append('\n');
            }
            Console.WriteLine(myString.ToString());
        }
    }
}
