use std::f64::consts::PI;

#[cfg(test)]
mod test {
    use num_traits::Float;
    use std::time::Instant;

    #[inline]
    fn foo<F>(x: F) -> F where F: Float {
        F::from(1.0).unwrap() + x
    }

    #[inline]
    fn bar(x: f64) -> f64 {
        1.0 + x
    }

    #[test]
    fn float_conversion() {
        let n = 1_000_000;
        let mut f1 = 0.0;
        let mut f2 = 0.0;
        let mut f3 = 0.0;
        {
            let tic = Instant::now();
            for _ in 0..n {
                f1 = foo(f1);
            }
            println!("{:?}", tic.elapsed());
        }
        {
            let tic = Instant::now();
            for _ in 0..n {
                f2 = 1.0 + f2;
            }
            println!("{:?}", tic.elapsed());
        }
        {
            let tic = Instant::now();
            for _ in 0..n {
                f3 = bar(f3);
            }
            println!("{:?}", tic.elapsed());
        }
        assert_eq!(f1, f2);
        assert_eq!(f1, f3);
    }
}

#[allow(dead_code)]
fn vec() {
    use cgmath::*;
    let a = vec3(1., 0., 0.);
    let b = vec3(0., 1., 0.);
    println!("{:?} {:?}", a.cross(b), a.dot(b));
    a.extend(1.0);
}

#[allow(dead_code)]
fn transform() {
    use cgmath::*;
    let a = vec3(0., 0., 1.);
    let m = Basis3::look_at(vec3(0., 1., 0.), vec3(0., 0., 1.));
    let b = m.rotate_vector(a);
    println!("transform: {:?}", m);
    println!("{:?} -> {:?}", a, b);

    let to_local: Matrix4<f64> = Matrix4::look_at(Point3::new(1., 1., 1.), Point3::new(1., 2., 1.), Vector3::unit_x());
    let to_global = to_local.inverse_transform().unwrap();
    println!("to local:  {:?}\nto global: {:?}", to_local, to_global);
    let a = Point3::new(0., 1., 0.);
    let b = to_global.transform_point(a);
    let c = to_local.transform_point(b);
    println!("{:?} -> {:?} -> {:?}", a, b, c);

    let rot = Matrix4::from_angle_z(Rad(PI / 2.));
    let x = Point3::new(1., 0., 1.);
    let y = rot.transform_point(x);
    println!("{:?}", rot);
    println!("{:?} -> {:?}", x, y);
}

#[allow(dead_code)]
fn image() {
    use image::*;
    let img = open("./Madeline.jpeg").unwrap();
    let img = img.into_rgb();
    println!("image: {:?} x {:?}", img.width(), img.height());

    let mut img = ImageBuffer::new(1024, 768);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let g = (f64::sin((x + y) as f64 * 0.01).powi(2) * 255.) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = Rgb([r, g, b]);
    }
    img.save("test_img.png").unwrap();
}

fn main() {
    // image();
    // vec();
    // transform();
}
