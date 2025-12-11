from collections import deque

def part1(data):
	required_button_presses = 0
	line_id = 0

	for line in data:
		indicator_lights, buttons, _ = parse_line(line)

		starting_lights = [-1] * len(indicator_lights)
		queue = deque()
		queue.append((starting_lights, 0))

		max_presses = 8
		least_presses = 1000000000

		while len(queue) > 0:
			instructions = queue.popleft()
			current_lights = instructions[0]
			current_presses = instructions[1]

			# print(instructions, len(queue))

			if current_lights == indicator_lights:
				if current_presses < least_presses:
					least_presses = current_presses
					# print("New least presses:", least_presses)
					continue

			if current_presses + 1 > max_presses or current_presses + 1 > least_presses:
				continue

			for button in buttons:
				new_lights = []
				for light_id in range(len(current_lights)):
					new_lights.append(current_lights[light_id] * button[light_id])
				queue.append((new_lights, current_presses + 1))

		required_button_presses += least_presses
		print("#", line_id, "Final least presses for light setup:", least_presses)
		line_id += 1

	print("Part 1: ", required_button_presses)

def part2(data):
	required_button_presses = 0
	line_id = 0

	for line in data:
		_, buttons, joltage_levels = parse_line(line)
		print(buttons, joltage_levels)

		starting_joltage = [0] * len(joltage_levels)
		queue = deque()

		queue.append((starting_joltage, 0))

		max_presses = 100
		least_presses = 1000000000

		while len(queue) > 0:
			instructions = queue.popleft()
			current_joltage = instructions[0]
			current_presses = instructions[1]

			# print("X", current_joltage, current_presses)

			if current_joltage == joltage_levels:
				if current_presses < least_presses:
					least_presses = current_presses
					print("New least presses:", least_presses)
					continue

			if current_presses + 1 > max_presses or current_presses + 1 > least_presses:
				continue

			for button in buttons:
				new_joltage = []
				for joltage_id in range(len(current_joltage)):
					if button[joltage_id] == -1:
						new_joltage.append(current_joltage[joltage_id] + 1)
					else:
						new_joltage.append(current_joltage[joltage_id])

				if new_joltage[joltage_id] > joltage_levels[joltage_id]:
					continue

				queue.append((new_joltage, current_presses + 1))

		required_button_presses += least_presses
		print("#", line_id, "Final least presses for joltage setup:", least_presses)
		line_id += 1

	print("Part 2: ", required_button_presses)

def parse_line(line):
	indicator_lights = [] # use 1 for ON and -1 for OFF
	buttons = [] # use 1 for lights that don't change status and -1 for those that will get flipped after clicking button
	joltage_levels = []
	parts = line.split(" ")
	for light_id in range(1, len(parts[0]) - 1):
		if parts[0][light_id] == ".":
			indicator_lights.append(-1)
		else:
			indicator_lights.append(1)

	for part_id in range(1, len(parts) - 1):
		new_button = [1] * len(indicator_lights)
		numbers = parts[part_id][1:len(parts[part_id]) - 1].split(",")
		for n in numbers:
			new_button[int(n)] = -1	
		buttons.append(new_button)

	joltage_numbers = parts[-1][1:len(parts[-1]) - 1].split(",")
	joltage_levels = []
	for jn in joltage_numbers:
		joltage_levels.append(int(jn))

	return indicator_lights, buttons, joltage_levels

# with open("input/day_10.txt", 'r') as file:
with open("input_test/day_10.txt", 'r') as file:
	data = file.read().splitlines()
	part1(data)
	# part2(data)