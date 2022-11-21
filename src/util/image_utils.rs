use std::collections::HashMap;
use std::fs;
use std::fs::{copy, File};
use std::future::Future;
use std::io::{Cursor, Read};
use std::marker::PhantomData;
use std::thread::Builder;
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use image::codecs::png;
use image::codecs::png::PngDecoder;
use image::{GenericImageView, RgbaImage};
use image::io::Reader;
use log::error;
use raqote::{DrawOptions, DrawTarget, Image, Point, SolidSource, Source};
// use image::{RgbaImage, GenericImage, ImageBuffer, Rgba, DynamicImage, FlatSamples};
// use image::flat::SampleLayout;
// use image::imageops::{blur, FilterType, resize};
// use image::io::Reader;
// use imageproc::drawing::{Canvas, draw_antialiased_line_segment_mut, draw_filled_circle, draw_filled_ellipse_mut, draw_filled_rect_mut, draw_hollow_circle_mut, draw_hollow_ellipse_mut, draw_hollow_rect_mut, draw_line_segment_mut, draw_polygon_mut, draw_text_mut};
// use imageproc::rect::Rect;
// use log::{error, info};
// use rusttype::{Font, Scale};
use crate::util::file::generate_random_tmp_image_path;

//
// pub struct ImageUtils {
//     image_buffer: RgbaImage,
// }
//
// impl ImageUtils {
//     pub fn new(width: u32, height: u32) -> ImageUtils {
//         let buffer: RgbaImage = ImageBuffer::new(width, height);
//         Self {
//             image_buffer: buffer
//         }
//     }
//     pub fn new_buf(iamge:RgbaImage) -> ImageUtils {
//         Self {
//             image_buffer: iamge
//         }
//     }
//     pub fn new_file(file:String) -> ImageUtils {
//         let dynamic = image::open(file).unwrap();
//         Self {
//             image_buffer: dynamic.to_rgba8()
//         }
//     }
//     //设置图片的颜色
//     pub fn set_background_color(&mut self, color: [u8; 4]) {
//         for (x, y, pixel) in self.image_buffer.enumerate_pixels_mut() {
//             *pixel = Rgba([color[0], color[1], color[2], color[3]]);
//         }
//     }
//
//     //设置网络图片
//     pub async fn set_background_url(&mut self, url: &str) {
//         let image = get_url_image(url).await;
//         match image {
//             None => {
//                 error!("设置网络图片背景失败!!!");
//             }
//             Some(data) => {
//                 &self.image_buffer.copy_from::<RgbaImage>(&data, 0, 0);
//             }
//         }
//     }
//
//     //设置本地图片
//     pub fn set_background_file(&mut self, file: String) {
//         let image = image::open(file).unwrap();
//         &self.image_buffer.copy_from::<RgbaImage>(&image.to_rgba8(), 0, 0);
//     }
//
//     //设置高斯模糊
//     pub fn set_blur(&mut self, sigma: f32) {
//         let buffer = blur(&self.image_buffer, sigma);
//         &self.image_buffer.copy_from::<RgbaImage>(&buffer, 0, 0);
//     }
//
//     //设置ImageBuffer类型图片
//     pub fn set_background_img(&mut self, image: RgbaImage) {
//         &self.image_buffer.copy_from::<RgbaImage>(&image, 0, 0);
//     }
//
//     //设置字的字体
//     pub fn set_font(num: i8) -> Font<'static> {
//         match num {
//             0 => Font::try_from_bytes(include_bytes!("../../resources/font/萝莉体.ttc")).unwrap(),
//             _ => Font::try_from_bytes(include_bytes!("../../resources/font/萝莉体.ttc")).unwrap(),
//         }
//     }
//     //画字
//     pub fn draw_text(&mut self, content: Vec<&str>, x: i32, y: i32, font: i8, font_size: i32, color: [u8; 4]) {
//         let mut con = String::new();
//         for str in content {
//             con.push_str(str);
//         };
//         draw_text_mut(&mut self.image_buffer,
//                       Rgba([color[0], color[1], color[2], color[3]]),
//                       x,
//                       y,
//                       Scale { x: font_size as f32, y: font_size as f32 },
//                       &Self::set_font(font),
//                       con.as_str());
//     }
//     //画字 多行 x,y是起点
//     pub fn draw_text_newline(&mut self, content: Vec<&str>, x: i32, y: i32, font: i8, font_size: f32, color: [u8; 4]) {
//         for text in content {
//             draw_text_mut(&mut self.image_buffer,
//                           Rgba([color[0], color[1], color[2], color[3]]),
//                           x,
//                           y + font_size as i32,
//                           Scale { x: font_size, y: font_size },
//                           &Self::set_font(font),
//                           text);
//         }
//     }
//     //添加网络图片
//     pub async fn draw_image_url(&mut self, image_url: &str, x: u32, y: u32) {
//         let image = get_url_image(image_url).await;
//         let (w, h) = &self.image_buffer.dimensions();
//         match image {
//             None => {
//                 error!("设置网络图片背景失败!!!");
//             }
//             Some(data) => {
//                 for (x1, y1, pixel) in data.enumerate_pixels() {
//                     if  (x1 < *w) && (y1 < *h) {
//                         &self.image_buffer.put_pixel(x1+x, y1+y, *pixel);
//                     }
//                 }
//                 // &self.image_buffer.copy_from::<RgbaImage>(&data, x, y);
//             }
//         }
//     }
//
//     pub async fn draw_image_newline_url(&mut self, iuxy: HashMap<&str, [u32; 2]>) {
//         let (w, h) = &self.image_buffer.dimensions();
//         for (url, xy) in iuxy {
//             let image = get_url_image(url).await;
//             match image {
//                 None => {
//                     error!("设置网络图片背景失败!!!");
//                 }
//                 Some(data) => {
//                     for (x1, y1, pixel) in data.enumerate_pixels() {
//                         if  (x1 < *w) && (y1 < *h) {
//                             &self.image_buffer.put_pixel(x1+xy[0], y1+xy[1], *pixel);
//                         }
//                     }
//                     // &self.image_buffer.copy_from::<RgbaImage>(&data, xy[0], xy[1]);
//                 }
//             }
//         }
//     }
//
//     pub fn draw_image_file(&mut self, file: String, x: u32, y: u32) {
//         let (w, h) = &self.image_buffer.dimensions();
//         let image = image::open(file).unwrap();
//         let img = image.to_rgba8();
//         for (x1, y1, pixel) in img.enumerate_pixels() {
//             if (x1 < *w) && (y1 < *h) {
//                 &self.image_buffer.put_pixel(x1+x, y1+y, *pixel);
//             }
//         }
//         // &self.image_buffer.copy_from::<RgbaImage>(&img, x, y);
//     }
//
//     pub fn draw_image_newline_file(&mut self, fxy: HashMap<String, [u32; 2]>) {
//         let (w, h) = &self.image_buffer.dimensions();
//         for (file, xy) in fxy {
//             let image = image::open(file).unwrap();
//             for (x1, y1, pixel) in image.to_rgba8().enumerate_pixels() {
//                 if (x1 < *w) && (y1 < *h) {
//                     &self.image_buffer.put_pixel(x1+xy[0], y1+xy[1], *pixel);
//                 }
//             }
//             // &self.image_buffer.copy_from::<RgbaImage>(&img, xy[0], xy[1]);
//         }
//     }
//
//     pub fn draw_image_img(&mut self, image: RgbaImage, x: u32, y: u32) {
//         let (w, h) = &self.image_buffer.dimensions();
//         for (x1, y1, pixel) in image.enumerate_pixels() {
//             if  (x1 < *w) && (y1 < *h) {
//                 &self.image_buffer.put_pixel(x1+x, y1+y, *pixel);
//             }
//         }
//         // &self.image_buffer.copy_from::<RgbaImage>(&image, x, y);
//     }
//
//     pub fn draw_image_newline_img(&mut self, ixy: HashMap<RgbaImage, [u32; 2]>) {
//         let (w, h) = &self.image_buffer.dimensions();
//         for (img, xy) in ixy {
//             for (x1, y1, pixel) in img.enumerate_pixels() {
//                 if  (x1 < *w) && (y1 < *h) {
//                     &self.image_buffer.put_pixel(x1+xy[0], y1+xy[1], *pixel);
//                 }
//             }
//         }
//         // &self.image_buffer.copy_from::<RgbaImage>(&img, xy[0], xy[1]);
//     }
//
//     pub fn draw_image_color(&mut self, color: [u8; 4], xy: [u32; 2]) {
//         let (w, h) = &self.image_buffer.dimensions();
//         let mut buffer = ImageBuffer::new(*w, *h);
//         for (x, y, pixel) in buffer.enumerate_pixels_mut() {
//             *pixel = Rgba([color[0], color[1], color[2], color[3]]);
//         }
//         for (x1, y1, pixel) in buffer.enumerate_pixels() {
//             if (x1 < *w) && (y1 < *h) {
//                 &self.image_buffer.put_pixel(x1+xy[0], y1+xy[1], *pixel);
//             }
//         }
//
//         // &self.image_buffer.copy_from::<RgbaImage>(&buffer, xy[0], xy[1]);
//     }
//     //画矩形 (长方形/正方形)
//     pub fn draw_filled_rect(&mut self, xy: [i32; 2], wh: [u32; 2], color: [u8; 4]) {
//         draw_filled_rect_mut(
//             &mut self.image_buffer,
//             Rect::at(xy[0], xy[1]).of_size(wh[0], wh[1]),
//             Rgba([color[0], color[1], color[2], color[3]]),
//         );
//     }
//     //画矩形轮廓
//     pub fn draw_hollow_rect(&mut self, xy: [i32; 2], wh: [u32; 2], color: [u8; 4]) {
//         draw_hollow_rect_mut(
//             &mut self.image_buffer,
//             Rect::at(xy[0], xy[1]).of_size(wh[0], wh[1]),
//             Rgba([color[0], color[1], color[2], color[3]]),
//         );
//     }
//     //画圆
//     pub fn draw_hollow_circle(&mut self, center: [i32; 2], radius: i32, color: [u8; 4]) {
//         draw_hollow_circle_mut(
//             &mut self.image_buffer,
//             (center[0], center[1]), radius,
//             Rgba([color[0], color[1], color[2], color[3]]),
//         );
//     }
//     //画圆形轮廓
//     pub fn draw_filled_circle(&mut self, center: [i32; 2], radius: i32, color: [u8; 4]) {
//         draw_filled_circle(
//             &mut self.image_buffer,
//             (center[0], center[1]), radius,
//             Rgba([color[0], color[1], color[2], color[3]]),
//         );
//     }
//     //画椭圆
//     pub fn draw_filled_ellipse(&mut self, center: [i32; 2], wh: [i32; 2], color: [u8; 4]) {
//         draw_filled_ellipse_mut(
//             &mut self.image_buffer,
//             (center[0], center[1]),
//             wh[0],
//             wh[1],
//             Rgba([color[0], color[1], color[2], color[3]]),
//         );
//     }
//     //画椭圆轮廓
//     pub fn draw_hollow_ellipse(&mut self, center: [i32; 2], wh: [i32; 2], color: [u8; 4]) {
//         draw_hollow_ellipse_mut(
//             &mut self.image_buffer,
//             (center[0], center[1]),
//             wh[0],
//             wh[1],
//             Rgba([color[0], color[1], color[2], color[3]]),
//         );
//     }
//     //画线段
//     pub fn draw_line_segment(&mut self, start: [f32; 2], end: [f32; 2], color: [u8; 4]) {
//         draw_line_segment_mut(
//             &mut self.image_buffer,
//             (start[0], start[1]),
//             (end[0], end[1]),
//             Rgba([color[0], color[1], color[2], color[3]]),
//         );
//     }
//     //导出为ImageBuffer 类型
//     pub fn flush_image_buffer(self) -> RgbaImage {
//         self.image_buffer
//     }
//
//     //导出到本地
//     pub fn flush_file(self, path:&String) -> String {
//         self.image_buffer.save(path);
//         path.to_string()
//     }
//     //导出为base64图片
//     pub fn flush_base64(self, path: &str) -> String {
//         let path = path;
//         let result = self.image_buffer.to_vec();
//         let opt_bytes = String::from_utf8(result).unwrap();
//         opt_bytes
//     }
//
// }
//
//
// pub async fn get_url_image(url: &str) -> Option<RgbaImage> {
//     let data = reqwest::get(url).await;
//     let content = match data {
//         Ok(data) => { Some(data.bytes().await.unwrap()) }
//         Err(err) => {
//             error!("获取网络图片失败!!{}",err);
//             None
//         }
//     };
//     match content {
//         None => None,
//         Some(con) => {
//             Some(Reader::new(Cursor::new(con)).with_guessed_format().unwrap().decode().unwrap().to_rgba8())
//         }
//     }
// }
//
// pub async fn resize_image_url(url:&str,nwidth: u32, nheight: u32, filter: FilterType) ->Option<RgbaImage>{
//     let image = get_url_image(url).await;
//     match image {
//        None => None,
//        Some(con) => {
//            Some(resize(&con, nwidth, nwidth, filter))
//        }
//    }
// }
//
// pub fn resize_image_img(image:RgbaImage,nwidth: u32, nheight: u32, filter: FilterType) ->Option<RgbaImage>{
//     let buffer = resize(&image, nwidth, nwidth, filter);
//     Some(buffer)
// }
//
// pub fn resize_image_file(file:String,nwidth: u32, nheight: u32, filter: FilterType) ->Option<RgbaImage>{
//     let result = image::open(file).unwrap();
//     let buffer = resize(&result, nwidth, nwidth, filter);
//     Some(buffer)
// }
//
// pub fn resize_save_tmp(image:RgbaImage) -> String{
//     let tmp = generate_random_tmp_image_path("resize_save_tmp", "jpg", vec!["image"]);
//     image.save(&tmp);
//     tmp
// }
// impl From<RgbaImage> for ImageUtils {
//     fn from(value: RgbaImage) -> Self {
//         Self {
//             image_buffer: value,
//         }
//     }
// }


pub struct ImageUtils {
    pub image: DrawTarget,
}

impl ImageUtils {
    //新建一个 DrawTarget
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            image: DrawTarget::new(width, height),
        }
    }
    //新建一个 DrawTarget 并设置单一颜色的纯色背景
    pub fn new_clear(width: i32, height: i32, color: [u8; 4]) -> Self {
        let mut target = DrawTarget::new(width, height);
        target.clear(SolidSource::from_unpremultiplied_argb(color[0], color[1], color[2], color[3]));
        Self {
            image: target,
        }
    }
    //新建一个 DrawTarget 并设置一张图片作为背景
    pub fn new_image(width: i32, height: i32, file: &str) -> Self {
        let mut target = DrawTarget::new(width, height);
        let (w, h, v) = Self::image_file(file);
        target.draw_image_at(0., 0., &Image {
            width: w,
            height: h,
            data: &v[..],
        }, &DrawOptions::new());
        Self {
            image: target,
        }
    }

    pub fn draw_text(&mut self, text: &str, font_size: f32, font: FamilyName, xy: [f32; 2], color: [u8; 4]) {
        let font = SystemSource::new()
            .select_best_match(&[font], &Properties::new())
            .unwrap()
            .load()
            .unwrap();
        self.image.draw_text(
            &font,
            font_size,
            text,
            Point {
                x: xy[0],
                y: xy[1],
                _unit: PhantomData,
            },
            &Source::Solid(SolidSource { r: color[0], g: color[1], b: color[2], a: color[3] }), &DrawOptions::new(),
        );
    }

    pub fn draw_fill_rect(&mut self, xy: [f32; 2], wh: [f32; 2], color: [u8; 4]) {
        self.image.fill_rect(
            xy[0], xy[1], wh[0], wh[1],
            &Source::Solid(SolidSource::from_unpremultiplied_argb(color[0], color[1], color[2], color[3])), &DrawOptions::new(),
        )
    }

    pub fn draw_image(&mut self, xy: [f32; 2], file: &str) {
        let (w, h, v) = Self::image_file(file);
        self.image.draw_image_at(
            xy[0], xy[1],
            &Image {
                width: w,
                height: h,
                data: &v[..],
            }, &DrawOptions::new(),
        )
    }

    pub fn draw_image_size(&mut self, xy: [f32; 2], wh: [f32; 2], file: &str) {
        let (w, h, v) = Self::image_file(file);
        self.image.draw_image_with_size_at(
            wh[0], wh[1], xy[0], xy[1],
            &Image {
                width: w,
                height: h,
                data: &v[..],
            }, &DrawOptions::new(),
        )
    }

    pub async fn draw_image_url(&mut self, xy: [f32; 2], url: &str) {
        let image = Self::get_url_image(url).await;
        let (w, h, v) = Self::image_url(image);
        self.image.draw_image_at(
            xy[0], xy[1],
            &Image {
                width: w,
                height: h,
                data: &v[..],
            }, &DrawOptions::new(),
        )
    }

    pub async fn draw_image_url_size(&mut self, xy: [f32; 2], wh: [f32; 2], url: &str) {
        let image = Self::get_url_image(url).await;
        let (w, h, v) = Self::image_url(image);
        self.image.draw_image_with_size_at(
            wh[0], wh[1], xy[0], xy[1],
            &Image {
                width: w,
                height: h,
                data: &v[..],
            }, &DrawOptions::new(),
        )
    }

    fn image_file(file: &str) -> (i32, i32, Vec<u32>) {
        let img = image::open(file).unwrap();
        let (w, h) = img.dimensions();
        let mut image: Vec<u32> = Vec::new();
        let rgba = img.to_rgba8().into_vec();
        for i in rgba.chunks(4) {
            // image.push(0xff << 24 | ((i[0] as u32) << 16) | ((i[1] as u32) << 8) | (i[2] as u32));
            let source = SolidSource::from_unpremultiplied_argb(i[3], i[0], i[1], i[2]);
            image.push(
                (source.a as u32) << 24
                    | (source.r as u32) << 16
                    | (source.g as u32) << 8
                    | (source.b as u32),
            );
        }
        (
            w as i32,
            h as i32,
            image,
        )
    }
    fn image_url(data: Option<RgbaImage>) -> (i32, i32, Vec<u32>) {
        match data {
            None => {
                (
                    0,
                    0,
                    vec![],
                )
            }
            Some(data) => {
                let (w, h) = data.dimensions();
                let mut image: Vec<u32> = Vec::new();
                let rgba = data.into_vec();
                for i in rgba.chunks(4) {
                    // image.push(0xff << 24 | ((i[0] as u32) << 16) | ((i[1] as u32) << 8) | (i[2] as u32));
                    let source = SolidSource::from_unpremultiplied_argb(i[3], i[0], i[1], i[2]);
                    image.push(
                        (source.a as u32) << 24
                            | (source.r as u32) << 16
                            | (source.g as u32) << 8
                            | (source.b as u32),
                    );
                }
                (
                    w as i32,
                    h as i32,
                    image,
                )
            }
        }
    }

    pub fn save(&mut self, file: &str) {
        self.image.write_png(file);
    }

    async fn get_url_image(url: &str) -> Option<RgbaImage> {
        let data = reqwest::get(url).await;
        let content = match data {
            Ok(data) => { Some(data.bytes().await.unwrap()) }
            Err(err) => {
                error!("获取网络图片失败!!{}",err);
                None
            }
        };
        match content {
            None => None,
            Some(con) => {
                Some(Reader::new(Cursor::new(con)).with_guessed_format().unwrap().decode().unwrap().to_rgba8())
            }
        }
    }
}


