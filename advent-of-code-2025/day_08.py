import math

def part1(points):
	print(points)
	direct_connections = {}
	circuits = []

	for point in points:
		new_circuit = set()
		new_circuit.add(point)
		circuits.append(new_circuit)
		direct_connections[point] = set()

	connections_made = 0

	# while connections_made < 10:
	for con_counter in range(1000):
		print(con_counter)
		min_distance = calculate_distance((0,0,0), (999999,999999,999999))
		point_x, point_y = None, None

		for x in range(len(points)):
			for y in range(len(points)):
				if x == y:
					continue
				
				if points[y] in direct_connections[points[x]]:
					continue

				if points[x] in direct_connections[points[y]]:
					continue

				# if points[x] in connected_points and points[y] in connected_points:
				# 	continue

				dist = calculate_distance(points[x], points[y])
				if dist < min_distance:
					min_distance = dist
					point_x = points[x]
					point_y = points[y]

		# print("Connecting dist", min_distance, "between points ", point_x, point_y, end=" ")
		direct_connections[point_x].add(point_y)
		direct_connections[point_y].add(point_x)

		point_x_circuit = None
		point_y_circuit = None

		for circuit in circuits:
			if point_x in circuit:
				point_x_circuit = circuit
			if point_y in circuit:
				point_y_circuit = circuit

		if point_x_circuit == point_y_circuit:
			# print("In the same circuit, skipping")
			continue

		if point_x_circuit is not None and point_y_circuit is not None:
			# print("Merging circuits", point_x_circuit, "     +     ", point_y_circuit)
			new_circuit = set()
			for x in point_x_circuit:
				new_circuit.add(x)
			for y in point_y_circuit:
				new_circuit.add(y)
			circuits.remove(point_x_circuit)
			circuits.remove(point_y_circuit)
			circuits.append(new_circuit)
		elif point_x_circuit is None:
			# print("Adding x to y circuit")
			point_y_circuit.add(point_x)
		elif point_y_circuit is None:
			# print("Adding y to x circuit")
			point_x_circuit.add(point_y)
		else:
			print("This should not happen")

		# connections_made += 1

	circuit_sizes = []

	for circuit in circuits:
		# print(circuit)
		circuit_sizes.append(len(circuit))

		circuit_sizes.sort(reverse=True)
	print(circuit_sizes)

	# for circuit in circuits:
	# 	print(circuit)

	# for cc in connected_points:
	# 	print(cc)

	print("Part 1: ", circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2])

def part2(data):
	result = 0
	print("Part 2: ", result)

def calculate_distance(box1, box2):
	a = pow(box1[0] - box2[0], 2)
	b = pow(box1[1] - box2[1], 2)
	c = pow(box1[2] - box2[2], 2)
	
	return math.sqrt(a + b + c)

with open("input/day_08.txt", 'r') as file:
# with open("input_test/day_08.txt", 'r') as file:
	data = file.read().splitlines()
	points = []
	for line in data:
		x = line.split(",")
		points.append((int(x[0]), int(x[1]), int(x[2])))

	part1(points)
	part2(points)
