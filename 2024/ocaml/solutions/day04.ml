type match_counts = {
  horizontal_forward : int;
  horizontal_backward : int;
  vertical_forward : int;
  vertical_backward : int;
  diagonal_ltr_forward : int;
  diagonal_ltr_backward : int;
  diagonal_rtl_forward : int;
  diagonal_rtl_backward : int;
}

type match_state = {
  vertical_forward_counter : int array;
  vertical_backward_counter : int array;
  diagonal_ltr_forward_counter : int array;
  diagonal_ltr_backward_counter : int array;
  diagonal_rtl_forward_counter : int array;
  diagonal_rtl_backward_counter : int array;
}

type full_state = { counts : match_counts; states : match_state }

let find_matches line pattern =
  let regexp = Re.compile (Re.str pattern) in
  Re.all regexp line |> List.length

let find_vertical_matches vertical_counter line pattern =
  let len = String.length line in
  let rec aux i count new_counter =
    if i >= len then (count, new_counter)
    else
      let find_letter = String.get pattern new_counter.(i) in
      let new_count =
        if String.get line i = find_letter then
          let new_val = new_counter.(i) + 1 in
          if new_val = 4 then (count + 1, 0) else (count, new_val)
        else if String.get line i = String.get pattern 0 then (count, 1)
        else (count, 0)
      in
      let updated_counter = Array.copy new_counter in
      updated_counter.(i) <- snd new_count;
      aux (i + 1) (fst new_count) updated_counter
  in

  aux 0 0 (Array.copy vertical_counter)

let find_diagonal_matches_left_to_right previous_line_diagonal_counter line
    pattern =
  let len = String.length line in
  let new_counter = Array.make len 0 in
  let rec loop i =
    if i >= len then new_counter
    else (
      (*find continuing diagonals*)
      if i > 0 then (
        let find_letter =
          String.get pattern previous_line_diagonal_counter.(i - 1)
        in
        if String.get line i = find_letter then
          new_counter.(i) <- previous_line_diagonal_counter.(i - 1) + 1)
      else ();

      (* find starting letter *)
      if
        previous_line_diagonal_counter.(i) < String.length pattern
        && len - String.length pattern >= i
      then (
        let find_letter = String.get pattern 0 in
        if String.get line i = find_letter then new_counter.(i) <- 1)
      else ();

      loop (i + 1))
  in

  loop 0

let find_diagonal_matches_right_to_left previous_line_diagonal_counter line
    pattern =
  let len = String.length line in
  let new_counter = Array.make len 0 in
  let rec loop i =
    if i >= len then new_counter
    else (
      (*find continuing diagonals*)
      if i < len - 1 then (
        let find_letter =
          String.get pattern previous_line_diagonal_counter.(i + 1)
        in
        if String.get line i = find_letter then
          new_counter.(i) <- previous_line_diagonal_counter.(i + 1) + 1)
      else ();

      (* find starting letter *)
      if
        previous_line_diagonal_counter.(i) < String.length pattern
        && i >= String.length pattern - 1
      then (
        let find_letter = String.get pattern 0 in
        if String.get line i = find_letter then new_counter.(i) <- 1)
      else ();

      loop (i + 1))
  in

  loop 0

let count_and_reset_fours arr =
  let new_arr = Array.copy arr in
  let count =
    Array.fold_left
      (fun (count, i) x ->
        if x >= 4 then (
          new_arr.(i) <- 0;
          (count + 1, i + 1))
        else (count, i + 1))
      (0, 0) arr
    |> fst
  in
  (count, new_arr)

let process_line line state =
  let {
    counts =
      {
        horizontal_forward;
        horizontal_backward;
        vertical_forward;
        vertical_backward;
        diagonal_ltr_forward;
        diagonal_ltr_backward;
        diagonal_rtl_forward;
        diagonal_rtl_backward;
      };
    _;
  } =
    state
  in

  let new_horizontal_forward = horizontal_forward + find_matches line "XMAS" in
  let new_horizontal_backward =
    horizontal_backward + find_matches line "SAMX"
  in

  let new_vertical_forward, new_vertical_forward_counter =
    find_vertical_matches state.states.vertical_forward_counter line "XMAS"
  in

  let new_vertical_backward, new_vertical_backward_counter =
    find_vertical_matches state.states.vertical_backward_counter line "SAMX"
  in

  let temp_diagonal_ltr_forward_counter =
    find_diagonal_matches_left_to_right
      state.states.diagonal_ltr_forward_counter line "XMAS"
  in
  let new_diagonal_ltr_forward, new_diagonal_ltr_forward_counter =
    count_and_reset_fours temp_diagonal_ltr_forward_counter
  in
  let temp_diagonal_ltr_backward_counter =
    find_diagonal_matches_left_to_right
      state.states.diagonal_ltr_backward_counter line "SAMX"
  in
  let new_diagonal_ltr_backward, new_diagonal_ltr_backward_counter =
    count_and_reset_fours temp_diagonal_ltr_backward_counter
  in

  let temp_diagonal_rtl_forward_counter =
    find_diagonal_matches_right_to_left
      state.states.diagonal_rtl_forward_counter line "XMAS"
  in
  let new_diagonal_rtl_forward, new_diagonal_rtl_forward_counter =
    count_and_reset_fours temp_diagonal_rtl_forward_counter
  in
  let temp_diagonal_rtl_backward_counter =
    find_diagonal_matches_right_to_left
      state.states.diagonal_rtl_backward_counter line "SAMX"
  in
  let new_diagonal_rtl_backward, new_diagonal_rtl_backward_counter =
    count_and_reset_fours temp_diagonal_rtl_backward_counter
  in

  {
    counts =
      {
        horizontal_forward = new_horizontal_forward;
        horizontal_backward = new_horizontal_backward;
        vertical_forward = new_vertical_forward + vertical_forward;
        vertical_backward = new_vertical_backward + vertical_backward;
        diagonal_ltr_forward = new_diagonal_ltr_forward + diagonal_ltr_forward;
        diagonal_ltr_backward =
          new_diagonal_ltr_backward + diagonal_ltr_backward;
        diagonal_rtl_forward = new_diagonal_rtl_forward + diagonal_rtl_forward;
        diagonal_rtl_backward =
          new_diagonal_rtl_backward + diagonal_rtl_backward;
      };
    states =
      {
        vertical_forward_counter = new_vertical_forward_counter;
        vertical_backward_counter = new_vertical_backward_counter;
        diagonal_ltr_forward_counter = new_diagonal_ltr_forward_counter;
        diagonal_ltr_backward_counter = new_diagonal_ltr_backward_counter;
        diagonal_rtl_forward_counter = new_diagonal_rtl_forward_counter;
        diagonal_rtl_backward_counter = new_diagonal_rtl_backward_counter;
      };
  }

let count_xmas filename : int =
  let lines = In_channel.with_open_text filename In_channel.input_lines in
  let initial_state =
    {
      counts =
        {
          horizontal_forward = 0;
          horizontal_backward = 0;
          vertical_forward = 0;
          vertical_backward = 0;
          diagonal_ltr_forward = 0;
          diagonal_ltr_backward = 0;
          diagonal_rtl_forward = 0;
          diagonal_rtl_backward = 0;
        };
      states =
        {
          vertical_forward_counter =
            Array.make (String.length (List.hd lines)) 0;
          vertical_backward_counter =
            Array.make (String.length (List.hd lines)) 0;
          diagonal_ltr_forward_counter =
            Array.make (String.length (List.hd lines)) 0;
          diagonal_ltr_backward_counter =
            Array.make (String.length (List.hd lines)) 0;
          diagonal_rtl_forward_counter =
            Array.make (String.length (List.hd lines)) 0;
          diagonal_rtl_backward_counter =
            Array.make (String.length (List.hd lines)) 0;
        };
    }
  in
  let final_state =
    List.fold_left
      (fun state line -> process_line line state)
      initial_state lines
  in

  Printf.printf "horizontal_forward_matches: %d\n"
    final_state.counts.horizontal_forward;

  Printf.printf "horizontal_backward_matches: %d\n"
    final_state.counts.horizontal_backward;

  Printf.printf "vertical_forward_matches: %d\n"
    final_state.counts.vertical_forward;

  Printf.printf "vertical_backward_matches: %d\n"
    final_state.counts.vertical_backward;

  Printf.printf "diagonal_left_to_right_forward_matches: %d\n"
    final_state.counts.diagonal_ltr_forward;

  Printf.printf "diagonal_left_to_right_backward_matches: %d\n"
    final_state.counts.diagonal_ltr_backward;

  Printf.printf "diagonal_right_to_left_forward_matches: %d\n"
    final_state.counts.diagonal_rtl_forward;

  Printf.printf "diagonal_right_to_left_backward_matches: %d\n"
    final_state.counts.diagonal_rtl_backward;

  final_state.counts.horizontal_forward + final_state.counts.horizontal_backward
  + final_state.counts.vertical_forward + final_state.counts.vertical_backward
  + final_state.counts.diagonal_ltr_forward
  + final_state.counts.diagonal_ltr_backward
  + final_state.counts.diagonal_rtl_forward
  + final_state.counts.diagonal_rtl_backward

let solve () =
  let result = count_xmas "inputs/day04.input" in
  Printf.printf "XMAS count: %d\n" result

let solve_bonus () = print_endline "Not yet implemented"
