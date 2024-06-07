# Oktu

Type safe Bend programs without typing any type!

## Example

```ml
(* write recursive functions *)
let rec fact n :=
  if n = 0 then
    1
  else
    n * fact (n - 1)

(* write generic functions *)
let id x := x

(* the entrypoint *)
let main :=
  let message = "Hello, " ++ "Oktu!" ++ " :D" in
  message
```
