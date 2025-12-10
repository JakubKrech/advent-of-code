def part1(data):
	required_button_presses = 0

	for line in data:
		indicator_lights, buttons = parse_line_part1(line)
		print("lights:", indicator_lights, "buttons:", buttons)
		break


	print("Part 1: ", required_button_presses)

def part2(data):
	result = 0
	print("Part 2: ", result)

def parse_line_part1(line):
	indicator_lights = []
	buttons = []
	parts = line.split(" ")
	for light_id in range(1, len(parts[0]) - 1):
		if parts[0][light_id] == ".":
			indicator_lights.append(0)
		else:
			indicator_lights.append(1)

	for part_id in range(1, len(parts) - 1):
		new_button = [0] * len(indicator_lights)
		numbers = parts[part_id][1:len(parts[part_id]) - 1].split(",")
		for n in numbers:
			new_button[int(n)] = 1	
		buttons.append(new_button)

	return indicator_lights, buttons

# with open("input/day_10.txt", 'r') as file:
with open("input_test/day_10.txt", 'r') as file:
	data = file.read().splitlines()
	part1(data)
	part2(data)