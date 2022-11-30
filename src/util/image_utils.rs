use std::fs::File;
use std::io::BufReader;
use image::ImageFormat;
use image::imageops::FilterType;
use plotters::prelude::{BitMapBackend, BitMapElement, ChartBuilder, DrawingBackend, IntoDrawingArea, LabelAreaPosition, WHITE};
use crate::util::file::{get_image_path, tmp_random_image_path};
//摸了 plotters 画图有点难
pub fn sign_image()  {
    let tmp = tmp_random_image_path("sign", "png", vec!["sign"]);
    let sign_background = get_image_path(vec!["sign","sign_background.png"]);
    let mut root = BitMapBackend::new(tmp.as_str(), (1000, 562)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root)
        .margin(0)
        .build_cartesian_2d(0.0..1.0, 0.0..1.0).unwrap();
    chart.configure_mesh().disable_mesh().draw().unwrap();
    let image = image::open(sign_background.as_str()).unwrap();

    let elem: BitMapElement<_> = ((0.0, 1.0), image).into();
    chart.draw_series(std::iter::once(elem)).unwrap();

    root.present().unwrap();
}
