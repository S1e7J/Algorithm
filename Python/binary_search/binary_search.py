def binary_search(list, n):
    low = 0;
    high = len(list);
    ret = 0;
    while low <= high:
        guess = int((low + high)/2);
        if list[guess] == n:
            ret = guess;
            break;
        elif list[guess] > n:
            high = guess - 1;
        else:
            low = guess + 1;
    return ret;

print(binary_search([1,2,3,4,5,6],5))
