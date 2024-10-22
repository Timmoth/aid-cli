use crate::{graph_utils, math_utils};
pub fn evaluate(expression: String) {
    match math_utils::parse_expr(&expression) {
        Ok((_, ast)) => {
            println!("{}", math_utils::evaluate(&ast, 0.0));
        }
        Err(e) => println!("Error parsing expression: {:?}", e),
    }
}

pub fn plot(startx: f32, endx: f32, step: f32, expression: String) {
    // Example points
    let points = graph_utils::generate_points(startx, endx, step, &expression);
    // Plot the chart
    graph_utils::plot_chart(&points, expression);
}