let find_matches line pattern =
  let regexp = Re.compile (Re.str pattern) in
  let matches = Re.all regexp line in
  List.length matches;;

let find_vertical_matches line pattern =


let count_xmas filename =
	let ic = open_in filename in
  let horizontal_forward_matches = ref 0 in
  let horizontal_backward_matches = ref 0 in
  let vertical_forward_matches = ref 0 in
	try
		let first_line = input_line ic in
		let line_width = String.length first_line in
		let vertical_forward_counter = Array.make line_width 0 in
    horizontal_forward_matches := !horizontal_forward_matches + (find_matches first_line "XMAS");
    horizontal_backward_matches := !horizontal_backward_matches + (find_matches first_line "SAMX");

    String.iteri (fun i c ->
if c = 'X' then vertical_forward_counter.(i) <- vertical_forward_counter.(i) + 1
    ) first_line;

    let rec process_lines () =
      try
      let line = input_line ic in
      horizontal_forward_matches := !horizontal_forward_matches + (find_matches line "XMAS");
      horizontal_backward_matches := !horizontal_forward_matches + (find_matches line "SAMX");
      process_lines ()

      with End_of_file ->
        close_in ic;
        !horizontal_forward_matches + !horizontal_backward_matches + !vertical_forward_matches
    in
    process_lines ()




	with e ->
		close_in_noerr ic;
		raise e;

;;
let solve () =
  let result = count_xmas "inputs/day04.test" in
  Printf.printf "XMAS count: %d\n" (result);

let solve_bonus () = print_endline "Not yet implemented"
