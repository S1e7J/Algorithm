let rec binary_search lista n low high =
  let guess = (low + high)/2 in
  if (lista[guess] == n) then guess
  if (lista[guess] < n) then binary_search lista n (guess - 1) high
  else binary_search lista n (guess - 1) high
