const std = @import("std");

pub fn binary_search(list: [6]i32, n: i32) usize {
    var low: usize = 0;
    var high: usize = list.len;
    var ret: usize = 0;
    while (low <= high) {
        var guess: usize = (low + high) / 2;
        if (list[guess] == n) {
            ret = guess;
            break;
        } else if (list[guess] < n) {
            low = guess + 1;
        } else {
            high = guess - 1;
        }
    }
    return ret;
}

pub fn main() void {
    const hola = binary_search([6]i32{ 1, 2, 3, 4, 5, 6 }, 5);
    std.debug.print("{d}\n", .{hola});
}
