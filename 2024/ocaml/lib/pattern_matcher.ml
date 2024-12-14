let count_horizontal_matches pattern line =
  let regexp = Re.compile (Re.str pattern) in
  Re.all regexp line |> List.length

(* Pure function for vertical matches *)
let count_vertical_matches pattern line prev_counts =
  String.to_seqi line
  |> Seq.fold_left
       (fun (counts, matches) (i, c) ->
         let current_count =
           if
             c
             = String.get pattern
                 (Option.value (List.nth_opt counts i) ~default:0)
           then List.nth counts i + 1
           else 0
         in
         let matches, final_count =
           if current_count = 4 then (matches + 1, 0)
           else (matches, current_count)
         in
         ( List.mapi (fun j v -> if j = i then final_count else v) counts,
           matches ))
       (prev_counts, 0)

let count_diagonal_matches pattern line prev_counts =
  let pattern_length = String.length pattern in
  String.to_seqi line
  |> Seq.fold_left
       (fun (counts, matches) (i, c) ->
         (* Check for continuation of previous diagonal pattern *)
         let continued_count =
           if i > 0 then
             let prev_pos_count =
               Option.value (List.nth_opt prev_counts (i - 1)) ~default:0
             in
             if
               prev_pos_count < pattern_length
               && c = String.get pattern prev_pos_count
             then prev_pos_count + 1
             else 0
           else 0
         in

         (* Check for new pattern start *)
         let start_count = if c = String.get pattern 0 then 1 else 0 in

         (* Take the most promising count *)
         let current_count = max continued_count start_count in

         (* Check if we completed a pattern *)
         let matches, final_count =
           if current_count = pattern_length then (matches + 1, 0)
             (* Reset after finding complete pattern *)
           else (matches, current_count)
         in

         ( List.mapi (fun j v -> if j = i then final_count else v) counts,
           matches ))
       (prev_counts, 0)

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
