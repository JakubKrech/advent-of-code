from collections import defaultdict

def part1(data):
	id_ranges = data.split(",")
	result = 0
	
	for id_range in id_ranges:
		r = id_range.split("-")
		first = int(r[0])
		last = int(r[1])

		for id in range(first, last + 1):
			if not validate_id(str(id)):
				result += id

	print("Part 1: ", result)

def validate_id(id):
	# print("validating: ", id)

	if len(id) % 2 == 1:
		return True
	
	# print("A: ", id[0:len(id) // 2])
	# print("B: ", id[len(id) // 2:])
	
	if id[0:len(id) // 2] == id[len(id) // 2:]:
		return False

	return True

def part2(data):
	id_ranges = data.split(",")
	result = 0
	
	for id_range in id_ranges:
		# print(id_range)
		r = id_range.split("-")
		first = int(r[0])
		last = int(r[1])

		for id in range(first, last + 1):
			if not validate_id_2(str(id)):
				result += id

	print("Part 2: ", result)

def validate_id_2(id):
	# print("validating: ", id)

	sequences = defaultdict(int)

	curr_len = 1
	max_len = len(id)
	curr_pos = 0

	while curr_len < max_len:

		if max_len % curr_len != 0:
			curr_len += 1
			curr_pos = 0
			sequences.clear()
			continue

		while curr_pos < max_len and len(sequences) <= 1:
			# print(id, curr_len, max_len, curr_pos, id[curr_pos:curr_pos + curr_len])
			sequences[id[curr_pos:curr_pos + curr_len]] += 1
			curr_pos += curr_len

		# print(sequences)

		if len(sequences) == 1:
			# print(" ----------------------------- Invalid: ", id)
			return False

		curr_len += 1
		curr_pos = 0
		sequences.clear()

	# print("A: ", id[0:len(id) // 2])
	# print("B: ", id[len(id) // 2:])
	
	return True

with open("input/day_02.txt", 'r') as file:
# with open("input_test/day_02.txt", 'r') as file:
	data = file.read().splitlines()
	part1(data[0])
	part2(data[0])