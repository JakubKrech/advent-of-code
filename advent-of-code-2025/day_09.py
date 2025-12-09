def part1(points):
	max_area = 0

	for x in range(len(points)):
		for y in range(len(points)):
			if x >= y:
				continue

			area = abs(points[x][0] - points[y][0] + 1) * abs(points[x][1] - points[y][1] + 1)
			if area > max_area:
				max_area = area

	print("Part 1: ", max_area)

use_test_input = True
# use_test_input = False
if use_test_input:
	file_path = "input_test/day_09.txt"
else:
	file_path = "input/day_09.txt"

with open(file_path, 'r') as file:
	data = file.read().splitlines()
	points = []

	for line in data:
		x = line.split(",")
		if use_test_input:
			points.append((int(x[1]), int(x[0])))
		else:
			points.append((int(x[0]), int(x[1])))
	
	part1(points)
	# part2(points)