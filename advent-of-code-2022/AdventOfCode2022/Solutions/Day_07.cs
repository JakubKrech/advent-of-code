using AdventOfCode2022.Tools;

namespace AdventOfCode2022.Solutions
{
    internal static class Day_07
    {
        public static void Solve()
        {
            // var input = InputData.ReadTestInput("Day_07.txt");
            var input = InputData.ReadInput("Day_07.txt");

            var (part1, part2) = Solution(input);

            Console.WriteLine("=== Day 07 ===");
            Console.WriteLine($"Part 1: {part1}");
            Console.WriteLine($"Part 2: {part2}\n");
        }

        private static (int, int) Solution(IEnumerable<string> input)
        {
            Folder root = new(@"\", p: null);
            Folder current = root;

            // Build folder tree
            foreach (var line in input)
            {

                if (line.StartsWith("$"))
                {
                    string restOfLine = line[2..];

                    if (restOfLine.StartsWith("ls"))
                    {
                        // Can be ignored
                    }
                    else // line starts with cd
                    {
                        // Change current folder to target of cd command
                        string targetFolder = restOfLine.Split()[1];
                        
                        if(targetFolder == @"/")
                        {
                            current = root;
                        }
                        else if (targetFolder == @"..")
                        {
                            current = (Folder)current!.parent!;
                        }
                        else
                        {
                            current = (Folder)current!.elements.Find(x => x.name == targetFolder)!;
                        }
                    }
                }
                else if (line.StartsWith("dir"))
                {
                    current!.elements.Add(new Folder(line.Split()[1], current));
                }
                else // Line starts with number which is size of file
                {
                    current!.elements.Add(
                        new File(
                            n: line.Split()[1], 
                            s: int.Parse(line.Split()[0])
                            )
                        );
                }
            }

            // Go recursively through the folder tree and calculate folder sizes
            List<int> folderSizes = new();
            CalculateFolderSize(root, folderSizes);
            int sumOfAtMost100000 = 0;

            foreach (int folderSize in folderSizes)
            {
                if (folderSize <= 100000) sumOfAtMost100000 += folderSize;
            }

            int totalSizeOfSmallestDirectoryThatNeedsToBeDeleted = Part2(folderSizes);

            return (sumOfAtMost100000, totalSizeOfSmallestDirectoryThatNeedsToBeDeleted);
        }

        private static int Part2(List<int> folderSizes)
        {
            const int totalDiskSpace = 70000000;
            const int requiredDiskSpace = 30000000;
            int rootFolderSize = folderSizes.Last();

            int requiredSpace = requiredDiskSpace - (totalDiskSpace - rootFolderSize);
            folderSizes.Sort();

            foreach (int folderSize in folderSizes)
            {
                if (folderSize > requiredSpace) return folderSize;
            }

            return -1;
        }

        class Element 
        { 
            public string name = "";
        }

        class File : Element
        {
            public int size;

            public File(string n, int s)
            {
                name = n;
                size = s;
            }
        }

        class Folder : Element
        {
            public List<Element> elements = new();
            public Element? parent;

            public Folder(string n, Element? p)
            {
                name = n;
                parent = p;
            }
        }

        private static int CalculateFolderSize(Folder f, List<int> folderSizes)
        {
            int totalSize = 0;

            foreach(Element elem in f.elements)
            {
                if(elem is Folder folder)
                {
                    totalSize += CalculateFolderSize(folder, folderSizes);
                }
                else if (elem is File file)
                {
                    totalSize += file.size;
                }
            }

            folderSizes.Add(totalSize);

            return totalSize;
        }
    }
}
