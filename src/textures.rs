use image::{DynamicImage, GenericImage};
use crate::perlin::Perlin;
use rand::Rng;
use crate::vectors::Vec3;
use std::rc::Rc;
use std::fmt::Debug;

pub trait Texture: Debug {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}


#[derive(Clone, Debug)]
pub struct ConstantTexture {
    pub color: Vec3,
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> ConstantTexture {
        ConstantTexture { color }
    }
}

impl Texture for ConstantTexture {
    #[allow(unused_variables)]
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Clone, Debug)]
pub struct CheckerTexture {
    pub even: Rc<dyn Texture>,
    pub odd: Rc<dyn Texture>,
}
impl CheckerTexture {
    pub fn new(even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> CheckerTexture {
        CheckerTexture { even, odd }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone, Debug)]
pub struct NoiseTexture {
    pub noise: Box<Perlin>,
    pub scale: f32,
}

impl NoiseTexture {
    pub fn new<R: Rng>(rng: &mut R, scale: f32) -> NoiseTexture {
        NoiseTexture {
            noise: Box::new(Perlin::new(rng)),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    #[allow(unused_variables)]
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        // Vec3::new(1., 1., 1.) * self.noise.noise(&(p * self.scale))
        // Vec3::new(1., 1., 1.) * self.noise.turbulence(&(p * self.scale), 7)
        Vec3::new(1., 1., 1.)
            * 0.5
            * (1. + (self.scale * p.z + 5. * self.noise.turbulence(&(p * self.scale), 7)).sin())
    }
}

#[derive(Clone, Debug)]
pub struct ImageTexture {
    pub nx: usize,
    pub ny: usize,
    pub data: Vec<u8>,
}
impl ImageTexture {
    pub fn new(image_file: &str) -> Self {
        let img = image::open(image_file).unwrap();
        let (nx, ny) = (img.dimensions().0 as usize, img.dimensions().1 as usize);
        let data = get_img_data(&img, nx, ny);
        ImageTexture {
            nx, ny, data
        }
    }
}

fn get_img_data(img: &DynamicImage, nx: usize, ny: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(nx * ny * 3);
    for (_, _, pixel) in img.pixels() {
        data.push(pixel[0]);
        data.push(pixel[1]);
        data.push(pixel[2]);
    }
    data
}

impl Texture for ImageTexture {
    #[allow(unused_variables)]
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let i = ((u * self.nx as f32) as usize).max(0).min(self.nx - 1);
        let j = (((1. - v) * self.ny as f32 - 0.001) as usize).max(0).min(self.ny - 1);
        let (r, g, b) = (
            f32::from(self.data[3 * i + 3 * self.nx * j]) / 255.,
            f32::from(self.data[3 * i + 3 * self.nx * j + 1]) / 255.,
            f32::from(self.data[3 * i + 3 * self.nx * j + 2]) / 255.,
        );
        Vec3::new(r, g, b)
    }
}
