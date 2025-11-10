use crate::SteeringTable;
use image::{Rgb, RgbImage};

pub fn get_data_a424(factor: f32, angle: f32) -> SteeringTable {
    let values = [
        [0.0; 14],
        [
            1.33333333, 1.33333333, 1.32992327, 1.25603865, 1.18993135, 1.13043478, 1.02766798,
            0.94202899, 0.86956522, 0.77961019, 0.68511199, 0.64516129, 0.64516129, 0.64516129,
        ],
        [
            2.66666667, 2.66666667, 2.65984655, 2.51207729, 2.3798627, 2.26086957, 2.05533597,
            1.88405797, 1.73913043, 1.55922039, 1.37022398, 1.29032258, 1.29032258, 1.29032258,
        ],
        [
            4.0, 4.0, 3.98976982, 3.76811594, 3.56979405, 3.39130435, 3.08300395, 2.82608696,
            2.60869565, 2.33883058, 2.05533597, 1.93548387, 1.93548387, 1.93548387,
        ],
        [
            5.33333333, 5.33333333, 5.31969309, 5.02415459, 4.7597254, 4.52173913, 4.11067194,
            3.76811594, 3.47826087, 3.11844078, 2.74044796, 2.58064516, 2.58064516, 2.58064516,
        ],
        [
            6.66666667, 6.66666667, 6.64961637, 6.28019324, 5.94965675, 5.65217391, 5.13833992,
            4.71014493, 4.34782609, 3.89805097, 3.42555995, 3.22580645, 3.22580645, 3.22580645,
        ],
        [
            8.0, 8.0, 7.97953964, 7.53623188, 7.1395881, 6.7826087, 6.16600791, 5.65217391,
            5.2173913, 4.67766117, 4.11067194, 3.22580645, 3.22580645, 3.22580645,
        ],
        [
            10.0, 10.0, 9.97442455, 9.42028986, 8.92448513, 8.47826087, 7.70750988, 7.06521739,
            6.52173913, 5.84707646, 4.11067194, 3.22580645, 3.22580645, 3.22580645,
        ],
        [
            12.0, 12.0, 11.9693095, 11.3043478, 10.7093822, 10.173913, 9.24901186, 8.47826087,
            7.82608696, 5.84707646, 4.11067194, 3.22580645, 3.22580645, 3.22580645,
        ],
        [
            14.6666667, 14.6666667, 14.629156, 13.8164251, 13.0892449, 12.4347826, 11.3043478,
            10.3623188, 7.82608696, 5.84707646, 4.11067194, 3.22580645, 3.22580645, 3.22580645,
        ],
        [
            18.6666667, 18.6666667, 18.6189258, 17.5845411, 16.6590389, 15.826087, 14.3873518,
            10.3623188, 7.82608696, 5.84707646, 4.11067194, 3.22580645, 3.22580645, 3.22580645,
        ],
        [
            24.0, 24.0, 23.9386189, 22.6086957, 21.4187643, 20.3478261, 14.3873518, 10.3623188,
            7.82608696, 5.84707646, 4.11067194, 3.22580645, 3.22580645, 3.22580645,
        ],
        [
            30.6666667, 30.6666667, 23.9386189, 22.6086957, 21.4187643, 20.3478261, 14.3873518,
            10.3623188, 7.82608696, 5.84707646, 4.11067194, 3.22580645, 3.22580645, 3.22580645,
        ],
    ];
    let key_steer_angle = [0, 10, 20, 30, 40, 50, 60, 75, 90, 110, 140, 180, 230];
    let key_speed = [0, 30, 40, 50, 60, 70, 90, 110, 130, 160, 200, 250, 300, 350];

    let old_max_steer = 230.0; // original table max steer
    let max_physical_steer = angle; // your wheel ±45°

    SteeringTable {
        wheel_angles: values,
        key_steer_angle,
        key_speed,
        max_wheel_angle: 30.6666667,
        scalling_factor: (old_max_steer / max_physical_steer) * factor,
    }
}

use plotters::prelude::*;

pub fn draw_steering_table(
    table: &SteeringTable,
    plots: Vec<Vec<f32>>, // wheel result
    path: &str,
) -> anyhow::Result<()> {
    let root = BitMapBackend::new(path, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_max = table.key_speed.last().copied().unwrap_or(350);
    let y_max = table.max_wheel_angle * table.scalling_factor;

    let mut chart = ChartBuilder::on(&root)
        .caption("TBD", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..x_max as f32, 0f32..y_max)?;

    chart
        .configure_mesh()
        .x_desc("Speed (km/h)")
        .y_desc("Wheel Angle (°)")
        .draw()?;

    for (i, row) in plots.iter().enumerate() {
        let series: Vec<(f32, f32)> = row
            .iter()
            .enumerate()
            .map(|(j, &wheel_angle)| (table.key_speed[j] as f32, wheel_angle))
            .collect();

        chart
            .draw_series(LineSeries::new(series, &Palette99::pick(i)))?
            .label(format!(
                "Steer {}°",
                table.key_steer_angle[i] as f32 * table.scalling_factor
            ))
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &Palette99::pick(i)));
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight) // position legend
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?; // actually draw it

    Ok(())
}
