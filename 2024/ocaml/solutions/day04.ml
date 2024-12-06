let find_matches line pattern =
  let regexp = Re.compile (Re.str pattern) in
  let matches = Re.all regexp line in
  List.length matches

let find_vertical_matches vertical_counter line pattern =
  let count = ref 0 in
  let len = String.length line in
  let rec loop i =
    if i >= len then !count
    else
      let find_letter = String.get pattern vertical_counter.(i) in
      if String.get line i = find_letter then
        vertical_counter.(i) <- vertical_counter.(i) + 1
      else vertical_counter.(i) <- 0;

      if vertical_counter.(i) = 4 then (
        count := !count + 1;
        vertical_counter.(i) <- 0);
      loop (i + 1)
  in

  loop 0

let find_diagonal_matches_left_to_right previous_line_diagonal_counter line
    pattern =
  let len = String.length line in
  let current_line_diagonal_counter = Array.make len 0 in
  let rec loop i =
    if i >= len then current_line_diagonal_counter
    else (
      (*find continuing diagonals*)
      (if i > 0 then
         let find_letter =
           String.get pattern previous_line_diagonal_counter.(i - 1)
         in
         if String.get line i = find_letter then
           current_line_diagonal_counter.(i) <-
             previous_line_diagonal_counter.(i - 1) + 1);

      (* find starting letter *)
      (if
         previous_line_diagonal_counter.(i) < String.length pattern
         && len - String.length pattern >= i
       then
         let find_letter = String.get pattern 0 in
         if String.get line i = find_letter then
           current_line_diagonal_counter.(i) <- 1);

      loop (i + 1))
  in

  loop 0

let count_and_reset_fours arr =
  let count = ref 0 in
  Array.mapi
    (fun i x ->
      if x = 4 then (
        incr count;
        arr.(i) <- 0;
        0)
      else x)
    arr
  |> ignore;
  !count

let count_xmas filename : int =
  let ic = open_in filename in
  let horizontal_forward_matches = ref 0 in
  let horizontal_backward_matches = ref 0 in
  let vertical_forward_matches = ref 0 in
  let vertical_backward_matches = ref 0 in
  let diagonal_left_to_right_forward_matches = ref 0 in
  let diagonal_left_to_right_backward_matches = ref 0 in
  try
    let first_line = input_line ic in
    let line_width = String.length first_line in
    let vertical_forward_counter = Array.make line_width 0 in
    let vertical_backward_counter = Array.make line_width 0 in
    let diagonal_left_to_right_forward_counter = Array.make line_width 0 in
    let diagonal_left_to_right_backward_counter = Array.make line_width 0 in
    horizontal_forward_matches :=
      !horizontal_forward_matches + find_matches first_line "XMAS";
    horizontal_backward_matches :=
      !horizontal_backward_matches + find_matches first_line "SAMX";

    String.iteri
      (fun i c ->
        match c with
        | 'X' ->
            vertical_forward_counter.(i) <- vertical_forward_counter.(i) + 1;
            if line_width - 4 >= i then
              diagonal_left_to_right_forward_counter.(i) <- 1
        | 'S' ->
            vertical_backward_counter.(i) <- vertical_backward_counter.(i) + 1;
            if line_width - 4 >= i then
              diagonal_left_to_right_backward_counter.(i) <- 1
        | _ -> ())
      first_line;

    let rec process_lines () =
      try
        let line = input_line ic in
        horizontal_forward_matches :=
          !horizontal_forward_matches + find_matches line "XMAS";

        horizontal_backward_matches :=
          !horizontal_backward_matches + find_matches line "SAMX";

        vertical_forward_matches :=
          !vertical_forward_matches
          + find_vertical_matches vertical_forward_counter line "XMAS";

        vertical_backward_matches :=
          !vertical_backward_matches
          + find_vertical_matches vertical_backward_counter line "SAMX";

        (* First find new diagonal matches *)
        let new_diagonal_counter =
          find_diagonal_matches_left_to_right
            diagonal_left_to_right_forward_counter line "XMAS"
        in

        (* Copy values to existing counter *)
        Array.iteri
          (fun i _ ->
            diagonal_left_to_right_forward_counter.(i) <-
              new_diagonal_counter.(i))
          new_diagonal_counter;

        (* Then count and reset any fours *)
        diagonal_left_to_right_forward_matches :=
          !diagonal_left_to_right_forward_matches
          + count_and_reset_fours diagonal_left_to_right_forward_counter;

        (* First find new diagonal matches *)
        let new_diagonal_counter_back =
          find_diagonal_matches_left_to_right
            diagonal_left_to_right_backward_counter line "SAMX"
        in

        (* Copy values to existing counter *)
        Array.iteri
          (fun i _ ->
            diagonal_left_to_right_backward_counter.(i) <-
              new_diagonal_counter_back.(i))
          new_diagonal_counter_back;

        (* Then count and reset any fours *)
        diagonal_left_to_right_backward_matches :=
          !diagonal_left_to_right_backward_matches
          + count_and_reset_fours diagonal_left_to_right_backward_counter;

        process_lines ()
      with End_of_file ->
        close_in ic;

        Printf.printf "horizontal_forward_matches: %d\n"
          !horizontal_forward_matches;

        Printf.printf "horizontal_backward_matches: %d\n"
          !horizontal_backward_matches;

        Printf.printf "vertical_forward_matches: %d\n" !vertical_forward_matches;

        Printf.printf "vertical_backward_matches: %d\n"
          !vertical_backward_matches;

        Printf.printf "diagonal_left_to_right_forward_matches: %d\n"
          !diagonal_left_to_right_forward_matches;

        Printf.printf "diagonal_left_to_right_backward_matches: %d\n"
          !diagonal_left_to_right_backward_matches;

        let total =
          !horizontal_forward_matches
          + !horizontal_backward_matches
          + !vertical_forward_matches + !vertical_backward_matches
          + !diagonal_left_to_right_forward_matches
          + !diagonal_left_to_right_backward_matches
        in
        total
    in
    process_lines ()
  with e ->
    close_in_noerr ic;
    raise e

let solve () =
  let result = count_xmas "inputs/day04.test" in
  Printf.printf "XMAS count: %d\n" result

let solve_bonus () = print_endline "Not yet implemented"
