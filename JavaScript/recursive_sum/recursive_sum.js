function recursive_sum(arr) {
  if (arr.length == 1) {
    return arr[0];
  } else {
    return arr[0] + recursive_sum(arr.slice(1,arr.length));
  }
}

console.log(recursive_sum([1,2,3,4,5]))

