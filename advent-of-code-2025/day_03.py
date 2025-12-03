def part1(line):
	result = 0

	for line in data:
		first_index = len(line) - 2
		second_index = len(line) - 1
		current_index = len(line) - 3

		while current_index >= 0:
			if line[current_index] >= line[first_index]:
				if line[first_index] > line[second_index]:
					second_index = first_index
				first_index = current_index
			current_index -= 1

		result += int(line[first_index]) * 10 +  int(line[second_index])

		# print(line, int(line[first_index]) * 10 +  int(line[second_index]))

	print("Part 1: ", result)

def part2(data):
	result = 0

	for line in data:
		battery_count = 12
		indexes = [-1] * battery_count

		for battery_index in range(battery_count):
			first_index = 0
			if battery_index > 0:
				first_index = indexes[battery_index - 1] + 1

			last_index = len(line) - battery_count + battery_index
			# print("Battery", battery_index, "can be placed in index", first_index, "-", last_index, "in line", line)

			max_index = first_index

			for i in range(first_index, last_index + 1):
				if line[i] > line[max_index]:
					max_index = i
			
			indexes[battery_index] = max_index
			# print(indexes, "", end="")

			# for index in indexes:
			# 	if index == -1:
			# 		continue
			# 	print(line[index], end="")
			# print("\n")

		for battery_index in range(battery_count):
			result += 10**(battery_count - battery_index - 1) * int(line[indexes[battery_index]])

	print("Part 2: ", result)


with open("input/day_03.txt", 'r') as file:
# with open("input_test/day_03.txt", 'r') as file:
	data = file.read().splitlines()

	part1(data)
	part2(data)