open Types

let create_counter () : match_counter =
  {
    horizontal_forward = 0;
    horizontal_backward = 0;
    vertical_forward = 0;
    vertical_backward = 0;
    diagonal_forward = 0;
    diagonal_backward = 0;
  }

let update_counter counter pattern_type direction count =
  match (pattern_type, direction) with
  | Horizontal, Forward ->
      counter.horizontal_forward <- counter.horizontal_forward + count
  | Horizontal, Backward ->
      counter.horizontal_backward <- counter.horizontal_backward + count
  | Vertical, Forward ->
      counter.vertical_forward <- counter.vertical_forward + count
  | Vertical, Backward ->
      counter.vertical_backward <- counter.vertical_backward + count
  | Diagonal, Forward ->
      counter.diagonal_forward <- counter.diagonal_forward + count
  | Diagonal, Backward ->
      counter.diagonal_backward <- counter.diagonal_backward + count
