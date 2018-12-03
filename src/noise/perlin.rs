extern crate rand;
extern crate tdmath;

use self::rand::Rng;
use tdmath::Vector3;
use std::num::Wrapping;

pub struct Perlin {
    ranfloat: Vec<f32>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            ranfloat: Perlin::perlin_generate(),
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Vector3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = Wrapping(p.x.floor() as u8);
        let j = Wrapping(p.y.floor() as u8);
        let k = Wrapping(p.z.floor() as u8);
        let mut c: [[[f32; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let xi = (i + Wrapping(di as u8)).0;
                    let yi = (j + Wrapping(dj as u8)).0;
                    let zi = (k + Wrapping(dk as u8)).0;
                    let x = self.perm_x[xi as usize];
                    let y = self.perm_y[yi as usize];
                    let z = self.perm_z[zi as usize];

                    c[di][dj][dk] = self.ranfloat[x ^ y ^ z];
                }
            }
        }

        return Perlin::trilinear_interp(c, u, v, w);
    }

    fn perlin_generate() -> Vec<f32> {
        let mut p = Vec::with_capacity(256);
        let mut rng = rand::thread_rng();

        for _ in 0..256 {
            p.push(rng.gen::<f32>());
        }

        return p;
    }

    fn permute(p: &mut Vec<usize>, n: usize) {
        let mut rng = rand::thread_rng();

        for i in (1..n).rev() {
            let target = (rng.gen::<f32>() * (i as f32 + 1.0)) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = Vec::with_capacity(256);
        for i in 0..256 {
            p.push(i);
        }
        Perlin::permute(&mut p, 256);
        return p;
    }

    fn trilinear_interp(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {      
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f32 * u + (1.0 - i as f32) * (1.0-u)) *
                             (j as f32 * v + (1.0 - j as f32) * (1.0-v)) *
                             (k as f32 * w + (1.0 - k as f32) * (1.0-w)) *
                             c[i][j][k]
                }
            }
        }

        return accum.abs();
    }
}
