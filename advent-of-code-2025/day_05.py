def part1(data):
	fresh_count = 0
	fresh_ranges, ingredient_ids = parse_data(data)

	for id in ingredient_ids:
		for range in fresh_ranges:
			if id >= range[0] and id <= range[1]:
				fresh_count += 1
				break

	print("Part 1: ", fresh_count)

def part2(data):
	fresh_ranges, _ = parse_data(data)
	unique_ranges = []

	current_range_id = 0
	compared_range_id = 1
	fresh_ranges.sort()

	while compared_range_id < len(fresh_ranges):
		if fresh_ranges[current_range_id][1] < fresh_ranges[compared_range_id][0]:
			unique_ranges.append(fresh_ranges[current_range_id])
			# print("Adding range", fresh_ranges[current_range_id], "made of ranges id", current_range_id, compared_range_id - 1)
			current_range_id = compared_range_id
		elif fresh_ranges[current_range_id][1] >= fresh_ranges[compared_range_id][0]:
			if fresh_ranges[current_range_id][1] < fresh_ranges[compared_range_id][1]:
				fresh_ranges[current_range_id] = (fresh_ranges[current_range_id][0], fresh_ranges[compared_range_id][1])
		else:
			print("should not happen", fresh_ranges[current_range_id], fresh_ranges[compared_range_id])

		compared_range_id += 1

	unique_ranges.append(fresh_ranges[current_range_id])
	# print("Finally adding range", fresh_ranges[current_range_id], "made of ranges id", current_range_id, compared_range_id - 1)

	result = 0

	for unique_range in unique_ranges:
		result += unique_range[1] - unique_range[0] + 1

	print("Part 2: ", result)

def parse_data(data):
	fresh_ranges = []
	ingredient_ids = []

	ranges_read = False

	for line in data:
		if line == "":
			ranges_read = True
			continue

		if not ranges_read:
			x = line.split("-")
			fresh_ranges.append((int(x[0]), int(x[1])))
		else:
			ingredient_ids.append(int(line))

	return fresh_ranges, ingredient_ids

with open("input/day_05.txt", 'r') as file:
# with open("input_test/day_05.txt", 'r') as file:
	data = file.read().splitlines()
	part1(data)
	part2(data)
