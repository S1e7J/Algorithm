def selection_sort(arr):
    for i in range(len(arr)):
        min_id = i;
        for j in range(i+1,len(arr)):
            if arr[min_id] > arr[j]:
                min_id = j;
        temp = arr[min_id];
        arr[min_id] = arr[i];
        arr[i] = temp;
    return arr;

print(selection_sort([1,23,13,514,123,34,1,231,2]))
