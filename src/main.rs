#![allow(dead_code)]

pub mod bboxes;
pub mod camera;
pub mod hitables;
pub mod materials;
pub mod perlin;
pub mod rays;
pub mod textures;
pub mod vectors;

extern crate rand;
extern crate image;

use crate::hitables::bvh::BVHNode;
use crate::hitables::spheres::{Sphere, MovingSphere};
use crate::hitables::rectangles::*;
use crate::hitables::cuboids::Cuboid;
use crate::hitables::mediums::ConstantMedium;
use crate::hitables::transforms::{FlipNormals, RotateY, Translate};
use crate::hitables::{color_world, Hitable};

use crate::materials::Material;
use crate::camera::Camera;
use crate::textures::{CheckerTexture, ConstantTexture, NoiseTexture, ImageTexture};
use crate::vectors::Vec3;
use rand::Rng;
use std::rc::Rc;

fn random_scene<R: Rng>(rng: &mut R) -> Vec<Rc<dyn Hitable>> {
    let n = 500;
    let mut world: Vec<Rc<dyn Hitable>> = Vec::with_capacity(n + 1);
    let checker = Rc::new(CheckerTexture::new(
        Rc::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Rc::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    ));
    world.push(Rc::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Rc::new(Material::lambertian(checker)),
    )));
    for a in -10..10 {
        for b in -10..10 {
            let choose_material = rng.gen::<f32>();
            let center = Vec3::new(
                (a as f32) + 0.9 * rng.gen::<f32>(),
                0.2,
                (b as f32) + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_material < 0.8 {
                    world.push(Rc::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0., 0.5 * rng.gen::<f32>(), 0.),
                        0.,
                        1.,
                        0.2,
                        Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        ))))),
                    )));
                } else if choose_material < 0.95 {
                    world.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Material::metal(
                            Vec3::new(
                                0.5 * (1. + rng.gen::<f32>()),
                                0.5 * (1. + rng.gen::<f32>()),
                                0.5 * (1. + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        ),
                    ))));
                } else {
                    world.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Material::dielectric(1.5)),
                    )));
                }
            }
        }
    }
    world.push(Rc::new(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        Rc::new(Material::dielectric(1.5)),
    )));
    world.push(Rc::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.4, 0.2, 0.1))))),
    )));
    world.push(Rc::new(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.,
        Rc::new(Material::metal(Vec3::new(0.7, 0.6, 0.5), 0.)),
    )));
    world
}

fn two_spheres() -> Vec<Rc<dyn Hitable>> {
    let checker = Rc::new(CheckerTexture::new(
        Rc::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Rc::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    ));
    let n = 50;
    let mut world = Vec::with_capacity(n + 1);
    world.push(Rc::new(Sphere::new(
        Vec3::new(0., -10., 0.),
        10.,
        Rc::new(Material::lambertian(checker.clone())),
    )) as Rc<dyn Hitable>);
    world.push(Rc::new(Sphere::new(
        Vec3::new(0., 10., 0.),
        10.,
        Rc::new(Material::lambertian(checker)),
    )) as Rc<dyn Hitable>);
    world
}

fn two_perlin_spheres<R: Rng>(rng: &mut R) -> Vec<Rc<dyn Hitable>> {
    let perlin_texture = Rc::new(NoiseTexture::new(rng, 5.));
    let mut world = Vec::with_capacity(2);
    world.push(Rc::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Rc::new(Material::lambertian(perlin_texture.clone())),
    )) as Rc<dyn Hitable>);
    world.push(Rc::new(Sphere::new(
        Vec3::new(0., 2., 0.),
        2.,
        Rc::new(Material::lambertian(perlin_texture)),
    )) as Rc<dyn Hitable>);
    world
}

fn simple_light<R: Rng>(rng: &mut R) -> Vec<Rc<dyn Hitable>> {
    let perlin_texture = Rc::new(NoiseTexture::new(rng, 4.));
    let mut world = Vec::with_capacity(4);
    world.push(Rc::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Rc::new(Material::lambertian(perlin_texture.clone())),
    )) as Rc<dyn Hitable>);
    world.push(Rc::new(Sphere::new(
        Vec3::new(0., 2., 0.),
        2.,
        Rc::new(Material::lambertian(perlin_texture)),
    )) as Rc<dyn Hitable>);
    //    world.push(Rc::new(Sphere::new(Vec3::new(0., 2., 0.), 2.,
    //                                    Material::diffuse_light(Rc::new(ConstantTexture::new(Vec3::new(4., 4., 4.)))))) as Rc<dyn Hitable>);
    world.push(Rc::new(Sphere::new(
        Vec3::new(0., 7., 0.),
        2.,
        Rc::new(Material::diffuse_light(Rc::new(ConstantTexture::new(Vec3::new(4., 4., 4.))))),
    )) as Rc<dyn Hitable>);
    world.push(Rc::new(XYRectangle::new(
        3.,
        5.,
        1.,
        3.,
        -2.,
        Rc::new(Material::diffuse_light(Rc::new(ConstantTexture::new(Vec3::new(4., 4., 4.))))),
    )) as Rc<dyn Hitable>);
    world
}

fn cornell_box() -> Vec<Rc<dyn Hitable>> {
    let mut world = Vec::with_capacity(6);
    let red = Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)))));
    let white = Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))));
    let green = Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)))));
    let light = Rc::new(Material::diffuse_light(Rc::new(ConstantTexture::new(Vec3::new(15., 15., 15.)))));
    let yzrect = YZRectangle::new(0., 555., 0., 555., 555., green);
    world.push(Rc::new(FlipNormals::new(Rc::new(yzrect))) as Rc<dyn Hitable>);
    world.push(Rc::new(YZRectangle::new(0., 555., 0., 555., 0., red)) as Rc<dyn Hitable>);
    world.push(Rc::new(XZRectangle::new(213., 343., 227., 332., 554., light)) as Rc<dyn Hitable>);
    let xzrect = XZRectangle::new(0., 555., 0., 555., 555., Rc::clone(&white));
    world.push(Rc::new(FlipNormals::new(Rc::new(xzrect))) as Rc<dyn Hitable>);
    world.push(Rc::new(XZRectangle::new(0., 555., 0., 555., 0., Rc::clone(&white))) as Rc<dyn Hitable>);
    let xyrect = XYRectangle::new(0., 555., 0., 555., 555., Rc::clone(&white));
    world.push(Rc::new(FlipNormals::new(Rc::new(xyrect))) as Rc<dyn Hitable>);
    world
}

fn cornell_box_with_cuboids<R: Rng>(rng: &mut R) -> Vec<Rc<dyn Hitable>> {
    let mut world = Vec::with_capacity(8);
    let red = Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)))));
    let white = Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))));
    let green = Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)))));
    let light = Rc::new(Material::diffuse_light(Rc::new(ConstantTexture::new(Vec3::new(15., 15., 15.)))));
    let yzrect = YZRectangle::new(0., 555., 0., 555., 555., green);
    world.push(Rc::new(FlipNormals::new(Rc::new(yzrect))) as Rc<dyn Hitable>);
    world.push(Rc::new(YZRectangle::new(0., 555., 0., 555., 0., red)) as Rc<dyn Hitable>);
    world.push(Rc::new(XZRectangle::new(213., 343., 227., 332., 554., light)) as Rc<dyn Hitable>);
    let xzrect = XZRectangle::new(0., 555., 0., 555., 555., Rc::clone(&white));
    world.push(Rc::new(FlipNormals::new(Rc::new(xzrect))) as Rc<dyn Hitable>);
    world.push(Rc::new(XZRectangle::new(0., 555., 0., 555., 0., Rc::clone(&white))) as Rc<dyn Hitable>);
    let xyrect = XYRectangle::new(0., 555., 0., 555., 555., Rc::clone(&white));
    world.push(Rc::new(FlipNormals::new(Rc::new(xyrect))) as Rc<dyn Hitable>);

    world.push(Rc::new(Translate::new(
        Rc::new(RotateY::new(
            Rc::new(Cuboid::new(
                Vec3::new(0., 0., 0.),
                Vec3::new(165., 165., 165.),
                &Rc::clone(&white),
                rng,
            )),
            -18.,
        )),
        Vec3::new(130., 0., 65.),
    )) as Rc<dyn Hitable>);
    world.push(Rc::new(Translate::new(
        Rc::new(RotateY::new(
            Rc::new(Cuboid::new(
                Vec3::new(0., 0., 0.),
                Vec3::new(165., 330., 165.),
                &Rc::clone(&white),
                rng,
            )),
            15.,
        )),
        Vec3::new(265., 0., 295.),
    )) as Rc<dyn Hitable>);
    world
}

fn cornell_smoke<R: Rng>(rng: &mut R) -> Vec<Rc<dyn Hitable>> {
    let mut world = Vec::with_capacity(8);
    let red = Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)))));
    let white = Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))));
    let green = Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)))));
    let light = Rc::new(Material::diffuse_light(Rc::new(ConstantTexture::new(Vec3::new(15., 15., 15.)))));
    let yzrect = YZRectangle::new(0., 555., 0., 555., 555., green);
    world.push(Rc::new(FlipNormals::new(Rc::new(yzrect))) as Rc<dyn Hitable>);
    world.push(Rc::new(YZRectangle::new(0., 555., 0., 555., 0., red)) as Rc<dyn Hitable>);
    world.push(Rc::new(XZRectangle::new(113., 443., 127., 432., 554., light)) as Rc<dyn Hitable>);
    let xzrect = XZRectangle::new(0., 555., 0., 555., 555., Rc::clone(&white));
    world.push(Rc::new(FlipNormals::new(Rc::new(xzrect))) as Rc<dyn Hitable>);
    world.push(Rc::new(XZRectangle::new(0., 555., 0., 555., 0., Rc::clone(&white))) as Rc<dyn Hitable>);
    let xyrect = XYRectangle::new(0., 555., 0., 555., 555., Rc::clone(&white));
    world.push(Rc::new(FlipNormals::new(Rc::new(xyrect))) as Rc<dyn Hitable>);

    let b1 = Rc::new(Translate::new(
        Rc::new(RotateY::new(
            Rc::new(Cuboid::new(
                Vec3::new(0., 0., 0.),
                Vec3::new(165., 165., 165.),
                &Rc::clone(&white),
                rng,
            )),
            -18.,
        )),
        Vec3::new(130., 0., 65.),
    )) as Rc<dyn Hitable>;
    let b2 = Rc::new(Translate::new(
        Rc::new(RotateY::new(
            Rc::new(Cuboid::new(
                Vec3::new(0., 0., 0.),
                Vec3::new(165., 330., 165.),
                &Rc::clone(&white),
                rng,
            )),
            15.,
        )),
        Vec3::new(265., 0., 295.),
    )) as Rc<dyn Hitable>;
    world.push(Rc::new(ConstantMedium::new(
        b1,
        0.01,
        Rc::new(ConstantTexture::new(Vec3::new(1., 1., 1.))),
    )) as Rc<dyn Hitable>);
    world.push(Rc::new(ConstantMedium::new(
        b2,
        0.01,
        Rc::new(ConstantTexture::new(Vec3::new(0., 0., 0.))),
    )) as Rc<dyn Hitable>);
    world
}

fn final_scene<R: Rng>(rng: &mut R) -> Vec<Rc<dyn Hitable>> {
    let nb = 20;
    let mut world = Vec::with_capacity(30);
    let mut boxlist = Vec::with_capacity(10000);
    let mut boxlist_2 = Vec::with_capacity(10000);
    let white = Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))));
    let ground = Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.43, 0.83, 0.53)))));
    for i in 0..nb {
        for j in 0..nb {
            let w = 100.;
            let (x0, y0, z0) = (-1000. + (i as f32) * w, 0., -1000. + (j as f32) * w);
            let (x1, y1, z1) = (x0 + w, 100. * (rng.gen::<f32>() + 0.01), z0 + w);
            boxlist.push(Rc::new(Cuboid::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                &Rc::clone(&ground),
                rng,
            )) as Rc<dyn Hitable>);
        }
    }
    world.push(Rc::new(BVHNode::bvh_node(&mut boxlist, 0., 1., rng)) as Rc<dyn Hitable>);
    let light = Rc::new(Material::diffuse_light(Rc::new(ConstantTexture::new(Vec3::new(7., 7., 7.)))));
    world.push(Rc::new(XZRectangle::new(
        123.,
        423.,
        147.,
        412.,
        554.,
        Rc::clone(&light),
    )) as Rc<dyn Hitable>);
    let center = Vec3::new(400., 400., 200.);
    world.push(Rc::new(MovingSphere::new(
        center,
        center + Vec3::new(30., 0., 0.),
        0.,
        1.,
        50.,
        Rc::new(Material::lambertian(Rc::new(ConstantTexture::new(Vec3::new(0.7, 0.3, 0.1))))),
    )) as Rc<dyn Hitable>);
    world.push(Rc::new(Sphere::new(
        Vec3::new(260., 150., 45.),
        50.,
        Rc::new(Material::dielectric(1.5)),
    )) as Rc<dyn Hitable>);
    world.push(Rc::new(Sphere::new(
        Vec3::new(0., 150., 145.),
        50.,
        Rc::new(Material::metal(Vec3::new(0.8, 0.8, 0.9), 10.0)),
    )) as Rc<dyn Hitable>);
    let boundary = Rc::new(Sphere::new(
        Vec3::new(360., 150., 145.),
        70.,
        Rc::new(Material::dielectric(1.5)),
    )) as Rc<dyn Hitable>;
    world.push(Rc::clone(&boundary));
    world.push(Rc::new(ConstantMedium::new(
        boundary,
        0.2,
        Rc::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9))),
    )) as Rc<dyn Hitable>);
    let boundary =
        Rc::new(Sphere::new(Vec3::empty(), 5000., Rc::new(Material::dielectric(1.5)))) as Rc<dyn Hitable>;
    world.push(Rc::new(ConstantMedium::new(
        boundary,
        0.0001,
        Rc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    )) as Rc<dyn Hitable>);
    let earth = Rc::new(Material::lambertian(Rc::new(ImageTexture::new("earth.png"))));
    world.push(Rc::new(Sphere::new(Vec3::new(400., 200., 400.), 100., earth)) as Rc<dyn Hitable>);
    let perlin_texture = NoiseTexture::new(rng, 0.1);
    world.push(Rc::new(Sphere::new(
        Vec3::new(220., 280., 300.),
        80.,
        Rc::new(Material::lambertian(Rc::new(perlin_texture))),
    )) as Rc<dyn Hitable>);
    let ns = 1000;
    for _ in 0..ns {
        boxlist_2.push(Rc::new(Sphere::new(
            Vec3::new(
                165. * rng.gen::<f32>(),
                165. * rng.gen::<f32>(),
                165. * rng.gen::<f32>(),
            ),
            10.,
            Rc::clone(&white),
        )) as Rc<dyn Hitable>);
    }
    world.push(Rc::new(Translate::new(
        Rc::new(RotateY::new(
            Rc::new(BVHNode::bvh_node(&mut boxlist_2, 0., 1., rng)),
            15.,
        )),
        Vec3::new(-100., 270., 395.),
    )) as Rc<dyn Hitable>);
    world
}

fn main() {
    let nx = 500;
    let ny = 500;
    let ns = 200;
    let mut rng = rand::thread_rng();
    print!("P3\n{}\n{}\n255\n", nx, ny);
    //let look_from = Vec3::new(13., 2., 3.);
    let look_from = Vec3::new(478., 278., -600.);
    //let look_at = Vec3::new(0., 0., 0.);
    let look_at = Vec3::new(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0., 1., 0.),
        40.,
        (nx as f32) / (ny as f32),
        aperture,
        dist_to_focus,
        0.,
        1.,
    );
    let external_light = false;
    //let world = BVHNode::bvh_node(&mut random_scene(&mut rng), 0., 1., &mut rng);
    //let world = BVHNode::bvh_node(&mut cornell_box_with_cuboids(&mut rng), 0., 1., &mut rng);
    let world = BVHNode::bvh_node(&mut final_scene(&mut rng), 0., 1., &mut rng);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::empty();
            for _ in 0..ns {
                let s = ((i as f32) + rng.gen::<f32>()) / (nx as f32);
                let t = ((j as f32) + rng.gen::<f32>()) / (ny as f32);
                let ray = &cam.get_ray(s, t, &mut rng);
                color += color_world(&ray, &world, 0, external_light, &mut rng);
            }
            color /= ns as f32;
            color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
            color *= 255.99;
            print!(
                "{} {} {}\n",
                color.x.min(255.) as u8,
                color.y.min(255.) as u8,
                color.z.min(255.) as u8
            );
        }
    }
}
