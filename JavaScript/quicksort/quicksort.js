function quicksort(arr) {
  if (arr.length < 2) {
    return arr;
  } else {
    let pivot = arr[0];
    let smalls = [];
    let bigs = [];
    for (let i = 1; i < arr.length ; i++) {
      if (arr[i] < pivot) {
        smalls.push(arr[i]);
      } else {
        bigs.push(arr[i]);
      }
    }
    return [...quicksort(smalls), pivot, ...quicksort(bigs)];
  }
}

console.log(quicksort([12,3,45,634,54,23,432,123,54,3,53,1,52,23,12,3542,43]))
