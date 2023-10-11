let rec factorial obj =
  if obj == 0 then 1 else obj * factorial (obj - 1);;

let hola = factorial 5;;

let () = Printf.printf "%d" hola;;
