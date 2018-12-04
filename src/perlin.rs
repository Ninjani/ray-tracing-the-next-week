use rand::Rng;
use crate::vectors::Vec3;

fn perlin_interpolate(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let (uu, vv, ww) = (
        u * u * (3. - 2. * u),
        v * v * (3. - 2. * v),
        w * w * (3. - 2. * w),
    );
    let mut accumulator = 0.;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - (i as f32), v - (j as f32), w - (k as f32));
                accumulator += (i as f32 * uu + (1. - i as f32) * (1. - uu))
                    * (j as f32 * vv + (1. - j as f32) * (1. - vv))
                    * (k as f32 * ww + (1. - k as f32) * (1. - ww))
                    * c[i][j][k].dot(weight);
            }
        }
    }
    accumulator
}

#[derive(Clone, Debug)]
pub struct Perlin {
    pub random_vector: Vec<Vec3>,
    pub perm_x: Vec<i64>,
    pub perm_y: Vec<i64>,
    pub perm_z: Vec<i64>,
}

impl Perlin {
    pub fn new<R: Rng>(rng: &mut R) -> Perlin {
        Perlin {
            random_vector: Perlin::perlin_generate(rng),
            perm_x: Perlin::perlin_generate_perm(rng),
            perm_y: Perlin::perlin_generate_perm(rng),
            perm_z: Perlin::perlin_generate_perm(rng),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f32 {
        let (u, v, w) = (p.x - p.x.floor(), p.y - p.y.floor(), p.z - p.z.floor());
        let (i, j, k) = (
            p.x.floor() as usize,
            p.y.floor() as usize,
            p.z.floor() as usize,
        );
        let mut c = [[[Vec3::new(0., 0., 0.); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vector[(self.perm_x[((i + di) & 255) as usize]
                                                           ^ self.perm_y[((j + dj) & 255) as usize]
                                                           ^ self.perm_z[((k + dk) & 255) as usize])
                                                           as usize];
                }
            }
        }
        perlin_interpolate(&c, u, v, w).abs()
    }

    fn perlin_generate<R: Rng>(rng: &mut R) -> Vec<Vec3> {
        let mut p = Vec::with_capacity(256);
        for _ in 0..256 {
            p.push(
                Vec3::new(
                    -1. + 2. * rng.gen::<f32>(),
                    -1. + 2. * rng.gen::<f32>(),
                    -1. + 2. * rng.gen::<f32>(),
                ).unit_vector(),
            );
        }
        p
    }

    fn permute<R: Rng>(p: &mut Vec<i64>, n: usize, rng: &mut R) {
        for i in (0..n).rev() {
            let target = (rng.gen::<f32>() * ((i + 1) as f32)) as usize;
            p.swap(i, target);
        }
    }

    fn perlin_generate_perm<R: Rng>(rng: &mut R) -> Vec<i64> {
        let mut p = Vec::with_capacity(256);
        for i in 0..256 {
            p.push(i);
        }
        Perlin::permute(&mut p, 256, rng);
        p
    }

    pub fn turbulence(&self, p: &Vec3, depth: usize) -> f32 {
        let (mut accumulator, mut temp_p, mut weight) = (0., *p, 1.);
        for _ in 0..depth {
            accumulator += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }
        accumulator.abs()
    }
}
