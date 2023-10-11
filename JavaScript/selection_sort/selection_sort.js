function selection_sort(arr) {
 for (let i = 0; i < arr.length ; i++) {
   let min_id = i;
   for (let j  = i + 1; j  < arr.length; j ++) {
     if (arr[min_id] > arr[j]) {
       min_id = j;
     }
   }
   let temp = arr[min_id];
   arr[min_id] = arr[i];
   arr[i] = temp;
 } 
  return arr;
}

console.log(selection_sort([1,23,1,51,23,12,32,123,3]))
