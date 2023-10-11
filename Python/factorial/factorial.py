def factorial(obj):
    if obj == 0:
        return 1;
    else:
        return obj * factorial(obj-1);

print(factorial(5))
