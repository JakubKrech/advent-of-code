param([String]$day="25") 

$day_string = "day_" + $day

Write-Output $day_string;

# Generate test input file
$input_test_file_path = "input_test\" + $day_string + ".txt"
Write-Output $input_test_file_path
New-Item $input_test_file_path -type file

# Generate input file
$input_file_path = "input\" + $day_string + ".txt"
Write-Output $input_file_path
New-Item $input_file_path -type file

# Copy day_xx.rs and rename it
$new_solution_file_path = "src\" + $day_string + ".rs"
Write-Output $new_solution_file_path
Copy-Item "src\day_XX.rs" $new_solution_file_path
