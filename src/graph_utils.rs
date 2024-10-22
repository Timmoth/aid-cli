use crate::math_utils;
use std::cmp::{max, min};

/// Interpolate between two points (x0, y0) and (x1, y1), and generate intermediate points.
fn interpolate(p1: (f32, f32), p2: (f32, f32), num_points: usize) -> Vec<(f32, f32)> {
    let mut interpolated_points = Vec::new();

    for i in 0..=num_points {
        let t = i as f32 / num_points as f32;
        let x = p1.0 * (1.0 - t) + p2.0 * t;
        let y = p1.1 * (1.0 - t) + p2.1 * t;
        interpolated_points.push((x, y));
    }

    interpolated_points
}

pub fn plot_chart(points: &Vec<(f32, f32)>, expression: String) {
    let width = 50;
    let height = 20;

    // Find min and max values for x and y to set the chart bounds
    let (min_x, max_x) = points.iter().fold(
        (f32::INFINITY, f32::NEG_INFINITY),
        |(min_x, max_x), &(x, _)| (min_x.min(x), max_x.max(x)),
    );

    let (min_y, max_y) = points.iter().fold(
        (f32::INFINITY, f32::NEG_INFINITY),
        |(min_y, max_y), &(_, y)| (min_y.min(y), max_y.max(y)),
    );

    let axis_x = if min_y <= 0.0 && max_y >= 0.0 {
        Some((0.0 - min_y) / (max_y - min_y) * (height - 1) as f32)
    } else if max_y < 0.0 {
        Some(height as f32 - 1.0)
        // x-axis at the top
    } else {
        Some(-1.0)
        // x-axis at the bottom
    };

    let axis_y = if min_x <= 0.0 && max_x >= 0.0 {
        Some(((0.0 - min_x) / (max_x - min_x) * (width - 1) as f32) as f32)
    } else if max_x < 0.0 {
        // y-axis at the left
        Some(width as f32 - 1.0)
    } else {
        Some(-1.0)
        // y-axis at the right
    };

    println!("plot: {}", expression);
    println!("X range: [{}, {}]", min_x, max_x);
    println!("Y range: [{}, {}]", min_y, max_y);
    // Create a 2D grid initialized with spaces
    let mut grid = vec![vec![' '; width]; height];

    // Draw the axis
    if let Some(axis_x) = axis_x {
        if axis_x >= 0.0 {
            let xaxis = axis_x as usize;
            if height > xaxis {
                for i in 0..width {
                    grid[height - 1 - xaxis][i] = '-'; // x-axis (horizontal)
                }
            }
        }
    }

    if let Some(axis_y) = axis_y {
        if axis_y >= 0.0 {
            let axisy = axis_y as usize;
            for i in 0..height {
                grid[i][axisy] = '|'; // y-axis (vertical)
                if let Some(axis_x) = axis_x {
                    if axis_x >= 0.0 {
                        let xaxis = axis_x as usize;

                        grid[height - 1 - xaxis][axisy] = '+'; // Intersection of x and y axes
                    }
                }
            }
        }
    }

    // Plot interpolated points
    for pair in points.windows(2) {
        let interpolated_points = interpolate(pair[0], pair[1], 10); // Interpolating with 10 points between each pair
        for &(x, y) in &interpolated_points {
            let scaled_x = ((x - min_x) / (max_x - min_x) * (width - 1) as f32) as usize;
            let scaled_y = ((y - min_y) / (max_y - min_y) * (height - 1) as f32) as usize;

            if height > scaled_y {
                grid[height - 1 - scaled_y][scaled_x] = '*';
            }
        }
    }

    // Print the grid
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }
}

pub fn generate_points(start: f32, end: f32, step: f32, expression: &String) -> Vec<(f32, f32)> {
    let exp = math_utils::parse_expr(&expression).unwrap();

    let mut points = Vec::new();

    let mut x = start;
    while x <= end {
        let y = math_utils::evaluate(&exp.1, x as f64) as f32;
        points.push((x, y));
        x += step;
    }

    points
}
