function binary_search(list,n) {
  let low = 0;
  let high = list.length;
  let ret = -1;
  while (low <= high) {
    let guess = Math.round((high + low)/2)
    if (list[guess] == n) {
      ret = guess;
      break;
    } else if (list[guess] < n){
      low = guess + 1;
    } else {
      high = guess - 1;
    }
  }
  return ret;
}

console.log(binary_search([1,2,3,4,5,6],5))
