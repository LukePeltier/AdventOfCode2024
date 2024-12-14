type direction = Forward | Backward
type pattern_type = Horizontal | Vertical | Diagonal

type match_counter = {
  mutable horizontal_forward : int;
  mutable horizontal_backward : int;
  mutable vertical_forward : int;
  mutable vertical_backward : int;
  mutable diagonal_forward : int;
  mutable diagonal_backward : int;
}

type pattern_state = { vertical_counts : int list; diagonal_counts : int list }
type pattern_counts = { horizontal : int; vertical : int; diagonal : int }
