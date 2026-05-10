struct Point {
    x: f32,
    y: f32,
}

struct Params {
    input_count: u32,
    output_count: u32,
}

@group(0) @binding(0) var<storage, read> input_data: array<Point>;
@group(0) @binding(1) var<storage, read_write> output_data: array<Point>;
@group(0) @binding(2) var<uniform> params: Params;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let bucket_idx = global_id.x;
    if (bucket_idx >= params.output_count) {
        return;
    }

    // First and last points are special in LTTB
    if (bucket_idx == 0u) {
        output_data[0] = input_data[0];
        return;
    }
    if (bucket_idx == params.output_count - 1u) {
        output_data[params.output_count - 1u] = input_data[params.input_count - 1u];
        return;
    }

    // Standard LTTB:
    // Bucket 0: [0]
    // Bucket 1 to output_count - 2: split remaining input_count - 2 points into output_count - 2 buckets.
    // Bucket output_count - 1: [input_count - 1]

    let bucket_size = f32(params.input_count - 2u) / f32(params.output_count - 2u);

    let start_idx = u32(floor(f32(bucket_idx - 1u) * bucket_size)) + 1u;
    let end_idx = u32(floor(f32(bucket_idx) * bucket_size)) + 1u;

    // Average of previous bucket (Point A approximation for parallelism)
    var avg_a = vec2<f32>(0.0, 0.0);
    if (bucket_idx == 1u) {
        avg_a = vec2<f32>(input_data[0].x, input_data[0].y);
    } else {
        let prev_start = u32(floor(f32(bucket_idx - 2u) * bucket_size)) + 1u;
        let prev_end = u32(floor(f32(bucket_idx - 1u) * bucket_size)) + 1u;
        var sum = vec2<f32>(0.0, 0.0);
        for (var i = prev_start; i < prev_end; i = i + 1u) {
            sum = sum + vec2<f32>(input_data[i].x, input_data[i].y);
        }
        avg_a = sum / f32(prev_end - prev_start);
    }

    // Average of next bucket (Point C)
    var avg_c = vec2<f32>(0.0, 0.0);
    if (bucket_idx == params.output_count - 2u) {
        avg_c = vec2<f32>(input_data[params.input_count - 1u].x, input_data[params.input_count - 1u].y);
    } else {
        let next_start = u32(floor(f32(bucket_idx) * bucket_size)) + 1u;
        let next_end = u32(floor(f32(bucket_idx + 1u) * bucket_size)) + 1u;
        var sum = vec2<f32>(0.0, 0.0);
        for (var i = next_start; i < next_end; i = i + 1u) {
            sum = sum + vec2<f32>(input_data[i].x, input_data[i].y);
        }
        avg_c = sum / f32(next_end - next_start);
    }

    // Find point in current bucket that forms largest triangle with avg_a and avg_c
    var max_area = -1.0;
    var selected_idx = start_idx;

    for (var i = start_idx; i < end_idx; i = i + 1u) {
        let p = vec2<f32>(input_data[i].x, input_data[i].y);
        // Triangle area formula: 0.5 * |x1(y2 - y3) + x2(y3 - y1) + x3(y1 - y2)|
        let area = abs(
            avg_a.x * (p.y - avg_c.y) +
            p.x * (avg_c.y - avg_a.y) +
            avg_c.x * (avg_a.y - p.y)
        ) * 0.5;

        if (area > max_area) {
            max_area = area;
            selected_idx = i;
        }
    }

    output_data[bucket_idx] = input_data[selected_idx];
}
