def part1(data):
	result = 0
	nums_and_operations = []

	for line in data:
		nums_and_operations.append(line.split())

	for col in range(len(nums_and_operations[0])):
		if nums_and_operations[-1][col] == '*':
			subresult = 1
			for row in range(len(nums_and_operations) - 1):
				subresult *= int(nums_and_operations[row][col])
			result += subresult
		else:
			for row in range(len(nums_and_operations) - 1):
				result += int(nums_and_operations[row][col])

	print("Part 1: ", result)

def part2(data):
	result = 0

	operation_row_index = len(data) - 1
	current_col_index = len(data[0]) - 1

	numbers = []

	while current_col_index >= 0:
		
		current_number = 0

		for row in range(len(data) - 1):
			current_digit = data[row][current_col_index]
			if current_digit.isdigit():
				current_number *= 10
				current_number += int(current_digit)
		numbers.append(current_number)

		if data[operation_row_index][current_col_index] in ['+', '*']:
			subresult = 0

			match data[operation_row_index][current_col_index]:
				case '+':
					subresult += sum(numbers)
				case '*':
					subresult = 1
					for n in numbers:
						subresult *= n
			
			result += subresult
			numbers.clear()
			current_col_index -= 2
		else:
			current_col_index -= 1
	
	print("Part 2: ", result)

with open("input/day_06.txt", 'r') as file:
# with open("input_test/day_06.txt", 'r') as file:
	data = file.read().splitlines()
	part1(data)
	part2(data)