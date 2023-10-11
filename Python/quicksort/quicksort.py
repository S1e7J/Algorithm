def quicksort(arr):
    if len(arr) < 2:
        return arr
    else:
        pivot = arr[0]
        smalls = []
        bigs = []
        for i in range(1,len(arr)):
            if arr[i] < pivot:
                smalls.append(arr[i])
            else:
                bigs.append(arr[i])
        return quicksort(smalls) + [pivot] + quicksort(bigs)

print(quicksort([1,23,1,541312,4512,52,13,21,541,23,1231,412,12,32,2,32,1,2,3]))
