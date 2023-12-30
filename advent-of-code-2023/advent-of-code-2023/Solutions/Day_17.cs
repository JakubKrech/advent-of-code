using AdventOfCode2022.Tools;

using Node = System.Tuple<int, int>;

namespace advent_of_code_2023.Solutions
{
    internal static class Day_17
    {
        public static void Solve()
        {
            AoCTools.RunMeasureTimeAndLog(Part1, Part2, "17", testInput: false);
        }

        private static int Part1(IEnumerable<string> input)
        {
            List<List<int>> board = Tools.ParseInput.Into2dIntList(input);

            Dictionary<Node, List<Tuple<Node, char, int>>> graph = GetReworkedGraph(board, minShift: 1, maxShift: 3);

            return RunDjikstraAlgorithmThroughTheGraph(graph, board);
        }

        private static int Part2(IEnumerable<string> input)
        {
            List<List<int>> board = Tools.ParseInput.Into2dIntList(input);

            Dictionary<Node, List<Tuple<Node, char, int>>> graph = GetReworkedGraph(board, minShift: 4, maxShift: 10);

            return RunDjikstraAlgorithmThroughTheGraph(graph, board);
        }

        private static Dictionary<Node, List<Tuple<Node, char, int>>> GetReworkedGraph(List<List<int>> board, int minShift, int maxShift)
        {
            Queue<Node> constructGraphQueue = new();
            constructGraphQueue.Enqueue(new Node(0, 0));

            Dictionary<Node, List<Tuple<Node, char, int>>> graph = new()
            {
                [new Node(0, 0)] = new List<Tuple<Node, char, int>>()
            };

            while (constructGraphQueue.Count > 0)
            {
                var currNode = constructGraphQueue.Dequeue();
                int row = currNode.Item1;
                int col = currNode.Item2;

                List<Tuple<int, int, char>> directions = new()
                {
                    new Tuple<int, int, char>(1, 0, 'v'),  // Go down
                    new Tuple<int, int, char>(-1, 0, '^'), // Go up
                    new Tuple<int, int, char>(0, 1, '>'),  // Go right
                    new Tuple<int, int, char>(0, -1, '<')  // Go left
                };

                foreach (var direction in directions)
                {
                    int row_shift = direction.Item1;
                    int col_shift = direction.Item2;
                    char directionChar = direction.Item3;
                    int pathCost = 0;

                    for (int shift = 1; shift <= maxShift; shift++)
                    {
                        int newRow = row;
                        int newCol = col;
                        bool canMove = true;

                        if (col_shift == 1 && col + shift < board[0].Count) // GO RIGHT
                        {
                            newCol = col + shift;
                        }
                        else if (col_shift == -1 && col - shift >= 0) // GO LEFT
                        {
                            newCol = col - shift;
                        }
                        else if (row_shift == -1 && row - shift >= 0) // GO UP
                        {
                            newRow = row - shift;
                        }
                        else if (row_shift == 1 && row + shift < board.Count) // GO DOWN
                        {
                            newRow = row + shift;
                        }
                        else
                        {
                            canMove = false;
                        }

                        if (canMove)
                        {
                            pathCost += board[newRow][newCol];
                            if (shift < minShift) continue; // Skip - just needed to calculate the initial pathCost

                            Node newNode = new(newRow, newCol);

                            // Add new entry for new dict key
                            if (!graph.ContainsKey(newNode))
                            {
                                graph[newNode] = new List<Tuple<Node, char, int>>();

                                // Add to queue for graph construction processing
                                constructGraphQueue.Enqueue(newNode);
                            }

                            graph[currNode].Add(new Tuple<Node, char, int>(newNode, directionChar, pathCost));
                        }
                    }
                }
            }

            return graph;
        }

        private static int RunDjikstraAlgorithmThroughTheGraph(Dictionary<Node, List<Tuple<Node, char, int>>> graph, List<List<int>> board)
        {
            int minPathLength = 0;

            // Use a combo of PriorityQueue and HashSet to get an effect of sorted container without duplicates
            PriorityQueue<Tuple<Node, int, char>, int> prioQueue = new();
            HashSet<Tuple<Node, int, char>> hashQueue = new();

            var initialQueueItem = new Tuple<Node, int, char>(new Node(0, 0), 0, '-');

            prioQueue.Enqueue(initialQueueItem, priority: 0);
            hashQueue.Add(initialQueueItem);

            while (hashQueue.Count > 0)
            {
                var queueItem = prioQueue.Dequeue();
                hashQueue.Remove(queueItem);

                Node currentNode = queueItem.Item1;
                int currentCost = queueItem.Item2;
                char previousDirection = queueItem.Item3;

                if (currentNode.Item1 == board.Count - 1 && currentNode.Item2 == board[0].Count - 1)
                {
                    minPathLength = currentCost;
                    break;
                }

                foreach (var destination in graph[currentNode])
                {
                    Node destinationNode = destination.Item1;
                    char destinationDirection = destination.Item2;
                    int destinationCost = destination.Item3;

                    if (previousDirection == destinationDirection || previousDirection == GetOppositeDirectionChar(destinationDirection)) continue;

                    var queueItemToAdd = new Tuple<Node, int, char>(destinationNode, currentCost + destinationCost, destinationDirection);
                    if (!hashQueue.Contains(queueItemToAdd))
                    {
                        hashQueue.Add(queueItemToAdd);
                        prioQueue.Enqueue(queueItemToAdd, currentCost + destinationCost);
                    }
                }
            }

            return minPathLength;
        }

        private static char GetOppositeDirectionChar(char c)
        {
            return c switch
            {
                '>' => '<',
                '<' => '>',
                '^' => 'v',
                'v' => '^',
                _ => 'x',
            };
        }
    }
}
