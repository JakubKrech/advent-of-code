def part1(data):
	point_to_zero_counter = 0
	current_position = 50

	for line in data:
		direction = line[0]
		steps = int(line[1:])
		steps = steps % 100

		if direction == "L":
			current_position -= steps
			if current_position < 0:
				current_position += 100
		else:
			current_position += steps
			if current_position > 99:
				current_position -= 100

		if current_position == 0:
				point_to_zero_counter += 1

	print("Part 1: ", point_to_zero_counter)

def part2(data):
	point_to_zero_counter = 0
	current_position = 50
	at_zero = False

	for line in data:
		direction = line[0]
		steps = int(line[1:])

		point_to_zero_counter += steps // 100
		steps = steps % 100

		if direction == "L":
			current_position -= steps

			if not at_zero and current_position < 0:
				point_to_zero_counter += 1
			if current_position < 0:
				current_position += 100
		else:
			current_position += steps
			if current_position > 100:
				point_to_zero_counter += 1

			if current_position > 99:
				current_position -= 100

		if current_position == 0:
			point_to_zero_counter += 1
			at_zero = True
		else:
			at_zero = False

	print("Part 2: ", point_to_zero_counter)

with open("input/day_01.txt", 'r') as file:
	data = file.read().splitlines()
	part1(data)
	part2(data)