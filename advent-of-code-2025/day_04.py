def part1(data):
	result = 0

	rows = len(data)
	cols = len(data[0])

	for row in range(rows):
		for col in range(cols):
			
			if data[row][col] != '@':
				# print(".", end="")
				continue
			
			adjacent = 0

			# Check top
			adjacent += row - 1 >= 0 and data[row - 1][col] == '@'

			# Check down
			adjacent += row + 1 < rows and data[row + 1][col] == '@'

			# Check left
			adjacent += col - 1 >= 0 and data[row][col - 1] == '@'

			# Check right
			adjacent += col + 1 < rows and data[row][col + 1] == '@'

			# Check top-left
			adjacent += row - 1 >= 0 and col - 1 >= 0 and data[row - 1][col - 1] == '@'

			# Check top-right
			adjacent += row - 1 >= 0 and col + 1 < rows and data[row - 1][col + 1] == '@'

			# Check down-left
			adjacent += row + 1 < rows and col - 1 >= 0 and data[row + 1][col - 1] == '@'

			# Check down-right
			adjacent += row + 1 < rows and col + 1 < rows and data[row + 1][col + 1] == '@'

			if adjacent < 4:
				result += 1
		# 	print(adjacent, end="")
		# print()

	print("Part 1: ", result)

def part2(data):
	result = 0

	rows = len(data)
	cols = len(data[0])

	removed_a_roll = True

	iteration = 0

	while removed_a_roll:
		iteration += 1
		removed_a_roll = False

		for row in range(rows):
			for col in range(cols):
				
				if data[row][col] == '.':
					# print(".", end="")
					continue
				
				adjacent = 0

				# Check top
				adjacent += row - 1 >= 0 and data[row - 1][col] != '.'

				# Check down
				adjacent += row + 1 < rows and data[row + 1][col] != '.'

				# Check left
				adjacent += col - 1 >= 0 and data[row][col - 1] != '.'

				# Check right
				adjacent += col + 1 < rows and data[row][col + 1] != '.'

				# Check top-left
				adjacent += row - 1 >= 0 and col - 1 >= 0 and data[row - 1][col - 1] != '.'

				# Check top-right
				adjacent += row - 1 >= 0 and col + 1 < rows and data[row - 1][col + 1] != '.'

				# Check down-left
				adjacent += row + 1 < rows and col - 1 >= 0 and data[row + 1][col - 1] != '.'

				# Check down-right
				adjacent += row + 1 < rows and col + 1 < rows and data[row + 1][col + 1] != '.'

				if adjacent < 4:
					new_row = list(data[row])
					new_row[col] = 'X'
					data[row] = new_row
			# 	print(adjacent, end="")
			# print()

		removed = 0

		for row in range(rows):
			for col in range(cols):
				if data[row][col] == "X":
					new_row = list(data[row])
					new_row[col] = '.'
					data[row] = new_row
					removed += 1

		if removed > 0:
			removed_a_roll = True
			# print("Iteration #", iteration, "Removed: ", removed)
			result += removed


	print("Part 2: ", result)

with open("input/day_04.txt", 'r') as file:
# with open("input_test/day_04.txt", 'r') as file:
	data = file.read().splitlines()
	part1(data)
	part2(data)