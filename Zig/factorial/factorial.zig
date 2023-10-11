const std = @import("std");

pub fn factorial(obj: i32) i32 {
    if (obj == 0) {
        return 1;
    } else {
        return obj * factorial(obj - 1);
    }
}

pub fn main() void {
    const hola = factorial(5);
    std.debug.print("{d}\n", .{hola});
}
