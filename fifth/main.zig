const std = @import("std");

const allocator = std.heap.page_allocator;
const print = std.debug.print;

const Range = struct {
    min: usize,
    max: usize
};

fn is_invalid_digit(val: u8) bool {
    return val < '0' or '9' < val;
}

fn read_entire_file(file_path: []const u8) ![]u8 {
    var file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();

    const stat = try file.stat();
    const buffer = try allocator.alloc(u8, stat.size);
    _ = try file.readAll(buffer); 
    
    return buffer;
}

fn get_number(buffer: []const u8, offset: *usize) !usize {
    var idx = offset.*;
    while (idx < buffer.len) {
        if (is_invalid_digit(buffer[idx])) {
            break;
        }
        idx += 1;
    }
    const slice = buffer[offset.*..idx];
    if (slice.len > 0) {
        offset.* = idx+1;
        return try std.fmt.parseInt(usize, slice, 10);
    } else {
        return 0;
    }
}

fn get_ranges(iter: *std.mem.SplitIterator(u8, std.mem.DelimiterType.sequence)) !std.ArrayList(Range) {
    var list = try std.ArrayList(Range).initCapacity(allocator, 0);
    while (iter.next()) |line| {
        if (std.mem.eql(u8, line, "")) {
            break;
        }
        var offset: usize = 0;
        const min = try get_number(line, &offset);
        const max = try get_number(line, &offset);
        try list.append(allocator, .{.min = min, .max = max});
    }
    return list;
}

pub fn filter_ranges(ranges: std.ArrayList(Range)) !std.ArrayList(Range) {
    var list = try std.ArrayList(Range).initCapacity(allocator, 0);
    var i: usize = 0;
    while (i < ranges.items.len) {
        const range = ranges.items[i];
        var j: usize = i+1;
        var did_overlap = false;
        while (j < ranges.items.len) {
            const _range = ranges.items[j];
            if (
                (range.max >= _range.min and range.min <= _range.min) or
                (range.max >= _range.max and range.min <= _range.max)
            ) {
                const new_range: Range = .{
                    .min = @min(range.min, _range.min),
                    .max = @max(range.max, _range.max),
                };
                ranges.items[j] = new_range;
                did_overlap = true;
            }
            j += 1;
        }
        if (!did_overlap) {
            try list.append(allocator, range);
        }
        i += 1;
    }
    return list;
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

    var iter = std.mem.splitSequence(u8, input, "\n");
    const raw_ranges = try get_ranges(&iter);
    const ranges = try filter_ranges(raw_ranges);

    var sum: usize = 0;
    while (iter.next()) |line| {
        var offset: usize = 0;
        const num = try get_number(line, &offset);
        for (ranges.items) |range| {
            if (range.min <= num and num <= range.max) {
                sum += 1;
                break;
            }
        }
    }

    std.debug.print("first part: {d}\n", .{sum});

    sum = 0;
    for (ranges.items) |range| {
        sum += range.max - range.min + 1;
    }
    std.debug.print("second part: {d}\n", .{sum});
}
