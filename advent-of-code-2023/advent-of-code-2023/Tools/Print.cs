namespace advent_of_code_2023.Tools
{
    using System.Text;

    internal static class Print
    {
        public static void Print2dList<T>(List<List<T>> data, string separator = "", int charWidth = 3)
        {
            StringBuilder myString = new();
            foreach (var line in data)
            {
                foreach (T c in line)
                {
                    myString.AppendFormat("{0," + charWidth +"}{1}", c, separator);
                }
                myString.Append('\n');
            }
            Console.WriteLine(myString.ToString());
        }
    }
}
