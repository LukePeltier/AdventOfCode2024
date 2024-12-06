module DayMap = Map.Make (Int)

module type Day = sig
  val solve : unit -> unit
  val solve_bonus : unit -> unit
end

let days =
  [
    (1, (module Solutions.Day01 : Day));
    (2, (module Solutions.Day02 : Day));
    (3, (module Solutions.Day03 : Day));
    (4, (module Solutions.Day04 : Day));
  ]
  |> List.to_seq |> DayMap.of_seq

let usage_msg = "aoc [-bonus] -d <day>"
let bonus = ref false
let day_num = ref 0

let anon_fun arg =
  Printf.fprintf stderr "Error: Unexpected argument '%s'\n" arg;
  exit 1

let () =
  let speclist =
    [
      ("-bonus", Arg.Set bonus, "Check solve_bonus function");
      ("-d", Arg.Set_int day_num, "Set the day number to check");
    ]
  in
  Arg.parse speclist anon_fun usage_msg;
  match DayMap.find_opt !day_num days with
  | Some day_module ->
      let module D = (val day_module : Day) in
      if !bonus then D.solve_bonus () else D.solve ()
  | None -> Printf.printf "Day %d not implemented yet.\n" !day_num
