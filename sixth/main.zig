const std = @import("std");
const print = std.debug.print;
const allocator = std.heap.page_allocator;

fn read_entire_file(file_path: []const u8) ![]u8 {
    var file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();

    const stat = try file.stat();
    const buffer = try allocator.alloc(u8, stat.size);
    _ = try file.readAll(buffer); 
    
    return buffer;
}

fn is_invalid_digit(val: u8) bool {
    return val < '0' or '9' < val;
}

fn get_number(buffer: []const u8, offset: *usize) !isize {
    // trim left
    while (offset.* < buffer.len and buffer[offset.*] == ' ') {
        offset.* += 1;
    }
    var idx = offset.*;
    while (idx < buffer.len) {
        if (is_invalid_digit(buffer[idx])) {
            break;
        }
        idx += 1;
    }
    const slice = buffer[offset.*..idx];
    if (slice.len > 0) {
        offset.* = idx + 1;
        return try std.fmt.parseInt(isize, slice, 10);
    } else {
        return 0;
    }
}

fn operate(x: isize, y: isize, op: u8) isize {
    return switch (op) {
        '+' => return x + y,
        '-' => return x - y,
        '*' => return x * y,
        // '/' => return x / y,
        else => unreachable
    };
}

pub fn main() !void {
    const argv = std.os.argv;
    if (argv.len < 2) {
        std.debug.print("usage: {s} <input_path>\n", .{argv[0]});
        std.process.exit(69);
    }

    const input_path: []const u8 = std.mem.span(argv[1]);
    const input = try read_entire_file(input_path);
    defer allocator.free(input);

    var lines = try std.ArrayList([]const u8).initCapacity(allocator, 0);
    defer lines.deinit(allocator);
    var iter = std.mem.splitSequence(u8, input, "\n");
    while (iter.next()) |part| {
        try lines.append(allocator, part);
    }
    const lines_count = lines.items.len - 1;

    var sum: isize = 0;
    var offset: usize = 0;
    while (offset < lines.items[0].len) {
        var l_sum: isize = 0;
        const op = lines.items[lines_count-1][offset];

        var max_loffset = offset;
        for (0.., lines.items) |idx, line| {
            if (idx == lines_count-1) break;
            var l_offset = offset;
            const num = try get_number(line, &l_offset);
            l_sum = if (idx == 0) num else operate(l_sum, num, op);
            if (l_offset > max_loffset) {
                max_loffset = l_offset;
            }
        }

        sum += l_sum;
        offset = max_loffset;
    }

    print("{d}\n", .{sum});
}
