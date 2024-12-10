use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/9

const DAY_STRING : &str = "day_09";
const USE_TEST_DATA : bool = false;

const EMPTY_DISK_SPACE : i32 = -1;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    let mut disk_map : Vec<(i32, i32)> = vec![]; // <disk_size, id_number>
    let mut compressed_len = 0;
    let mut total_empty_space = 0;

    // Read input into disk map
    let mut id_number = 0;
    let mut reading_empty_space : bool = false;
    for digit in input[0].chars() {
        let disk_size : i32 = digit.to_string().parse().expect("Failed to parse");
        if reading_empty_space {
            disk_map.push((disk_size, EMPTY_DISK_SPACE));
            total_empty_space += disk_size;
        } else {
            disk_map.push((disk_size, id_number));
            compressed_len += disk_size;
            id_number += 1;
        }
        reading_empty_space = !reading_empty_space;
    }

    let mut optimized_disk_map : Vec<(i32, i32)> = vec![];
    let mut current_moved_block_id = disk_map.len() - 1;
    let mut current_moved_block_remaining_size = disk_map[current_moved_block_id].0;

    for disk_field in &disk_map {

        if disk_field.1 != EMPTY_DISK_SPACE {
            optimized_disk_map.push((disk_field.0, disk_field.1));
            continue;
        }

        let mut remaining_space = disk_field.0;

        while remaining_space > 0 && total_empty_space > 0 {

            // Cut at the end
            if remaining_space > total_empty_space {
                total_empty_space = remaining_space;
            }

            // If current moved block was partially moved before, move it again
            if current_moved_block_remaining_size > remaining_space { // block wont fit in current space
                current_moved_block_remaining_size -= remaining_space;
                total_empty_space -= remaining_space;
                optimized_disk_map.push((remaining_space, disk_map[current_moved_block_id].1));
                remaining_space = 0;
            } else { // block will fit
                remaining_space -= current_moved_block_remaining_size;
                total_empty_space -= current_moved_block_remaining_size;
                optimized_disk_map.push((current_moved_block_remaining_size, disk_map[current_moved_block_id].1));
                
                current_moved_block_id -= 2;
                current_moved_block_remaining_size = disk_map[current_moved_block_id].0;
            }
        }
    }

    let mut result : i64 = 0;
    let mut compressed_index = 0;

    for x in optimized_disk_map {
        for _ in 0..x.0 {
            if compressed_index > compressed_len - 1 {
                break;
            }

            let xx : i64 = (compressed_index * x.1).try_into().expect("Failed to parse");
            result += xx;
            compressed_index += 1;
        }
    }

    return result.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    let mut disk_map : Vec<(i32, i32)> = vec![]; // <disk_size, id_number>

    // Read input into disk map
    let mut id_number = 0;
    let mut reading_empty_space : bool = false;
    for digit in input[0].chars() {
        let disk_size : i32 = digit.to_string().parse().expect("Failed to parse");
        if reading_empty_space {
            disk_map.push((disk_size, EMPTY_DISK_SPACE));
        } else {
            disk_map.push((disk_size, id_number));
            id_number += 1;
        }
        reading_empty_space = !reading_empty_space;
    }

    let mut current_moved_block_id = disk_map.len() - 1;

    while current_moved_block_id > 0 {

        // We move only data, skip empty space
        if disk_map[current_moved_block_id].1 == EMPTY_DISK_SPACE {
            current_moved_block_id -= 1;
            continue;
        }

        for i in 1..disk_map.len() {

            // We can insert only into empty spaces
            if disk_map[i].1 != EMPTY_DISK_SPACE {
                continue;
            }

            if i >= current_moved_block_id {
                break;
            }

            if disk_map[i].0 >= disk_map[current_moved_block_id].0 {
                let remaining_space = disk_map[i].0 - disk_map[current_moved_block_id].0;

                if remaining_space > 0 {
                    disk_map[i].0 = remaining_space;
                    disk_map.insert(i, (disk_map[current_moved_block_id].0, disk_map[current_moved_block_id].1));
                    current_moved_block_id += 1;

                    // Set moved block to empty space
                    disk_map[current_moved_block_id].1 = EMPTY_DISK_SPACE;
                } else {
                    disk_map[i].1 = disk_map[current_moved_block_id].1.clone();
                    
                    // Replace value in original block and set moved block to empty space
                    disk_map[current_moved_block_id].1 = EMPTY_DISK_SPACE;
                }

                current_moved_block_id -= 1;
                break;
            }
        }

        // No free space to move the block, skip it
        current_moved_block_id -= 1;
    }

    let mut result : i64 = 0;
    let mut compressed_index = 0;

    for x in disk_map {

        if x.1 == EMPTY_DISK_SPACE {
            compressed_index += x.0;
            continue;
        }

        for _ in 0..x.0 {
            let xx : i64 = (compressed_index * x.1).try_into().expect("Failed to parse");
            result += xx;

            compressed_index += 1;
        }
    }

    return result.to_string();
}
