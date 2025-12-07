def part1(data):
	split_count = 0
	array_2d, starting_beam = parse_input_to_2d_array(data)
	beams_to_process = []
	
	beams_to_process.append(starting_beam)

	while len(beams_to_process) > 0:
		beam = beams_to_process.pop()
		row, col = beam[0], beam[1]

		if array_2d[row][col] == '.':
			array_2d[row][col] = '|'

			if row + 1 < len(array_2d):
				if array_2d[row + 1][col] == '.':
					beams_to_process.append((row + 1, col))
				elif array_2d[row + 1][col] == '^':
					split_count += 1
					beams_to_process.append((row + 1, col - 1))
					beams_to_process.append((row + 1, col + 1))

	# for row in array_2d:
	# 	for c in row:
	# 		print(c, end="")
	# 	print()

	print("Part 1: ", split_count)

def part2(data):
	array_2d, starting_beam = parse_input_to_2d_array(data)
	cache = {}

	result = traverse(array_2d, cache, starting_beam[0], starting_beam[1], 1)

	print("Part 2: ", result)

def traverse(array_2d, cache, row, col, realities_counter):
	if row + 1 >= len(array_2d):
		return realities_counter
	
	if array_2d[row + 1][col] == '.':
		return traverse(array_2d, cache, row + 1, col, realities_counter)
	elif array_2d[row + 1][col] == '^':
		if (row + 1, col - 1) in cache:
			left_realities = cache[(row + 1, col - 1)]
		else:
			left_realities = cache[(row + 1, col - 1)] = traverse(array_2d, cache, row + 1, col - 1, 1)

		if (row + 1, col + 1) in cache:
			right_realities = cache[(row + 1, col + 1)]
		else:
			right_realities = cache[(row + 1, col + 1)] = traverse(array_2d, cache, row + 1, col + 1, 1)

		return left_realities + right_realities

def parse_input_to_2d_array(data):
	array_2d = []
	starting_beam = (-1, -1)
	
	for line in data:
		array_1d = []
		for c in line:
			array_1d.append(c)
		array_2d.append(array_1d)

	for x in range(len(array_2d[0])):
		if array_2d[0][x] == 'S':
			starting_beam = (1, x)
			break
	
	return array_2d, starting_beam

with open("input/day_07.txt", 'r') as file:
# with open("input_test/day_07.txt", 'r') as file:
	data = file.read().splitlines()
	part1(data)
	part2(data)