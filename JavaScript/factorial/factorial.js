function factorial(obj) {
  if (obj == 0) {
    return 1;
  } else {
    return obj * factorial(obj-1);
  }
}

console.log(factorial(5));
