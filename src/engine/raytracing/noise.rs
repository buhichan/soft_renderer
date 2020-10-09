use crate::rand3;

fn smoothstep (edge0: f64, edge1: f64, mut x: f64) -> f64{
  // Scale, bias and saturate x to 0..1 range
  x = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0); 
  // Evaluate polynomial
  return x * x * (3.0 - 2.0 * x);
}

fn clamp(x: f64, lowerlimit: f64, upperlimit: f64) -> f64 {
  if x < lowerlimit {
    return lowerlimit;
  }else if x > upperlimit {
    return upperlimit;
  }
  return x;
}

fn simple_interpolate(a: f64, b: f64, x: f64)->f64
{
   return a + smoothstep(0.0,1.0,x) * (b-a);
}


fn interpolatedNoise3D( x: f64, y: f64, z: f64) -> f64
{
    let integer_x: f64 = x - x.fract();
    let fractional_x: f64 = x - integer_x;

    let integer_y: f64 = y - y.fract();
    let fractional_y: f64 = y - integer_y;

    let integer_z: f64 = z - z.fract();
    let fractional_z: f64 = z - integer_z;

    let v1: f64 = rand3(integer_x, integer_y, integer_z);
    let v2: f64 = rand3(integer_x+1.0, integer_y, integer_z);
    let v3: f64 = rand3(integer_x, integer_y+1.0, integer_z);
    let v4: f64 = rand3(integer_x+1.0, integer_y +1.0, integer_z);

    let v5: f64 = rand3(integer_x, integer_y, integer_z+1.0);
    let v6: f64 = rand3(integer_x+1.0, integer_y, integer_z+1.0);
    let v7: f64 = rand3(integer_x, integer_y+1.0, integer_z+1.0);
    let v8: f64 = rand3(integer_x+1.0, integer_y +1.0, integer_z+1.0);

    let i1: f64 = simple_interpolate(v1,v5, fractional_z);
    let i2: f64 = simple_interpolate(v2,v6, fractional_z);
    let i3: f64 = simple_interpolate(v3,v7, fractional_z);
    let i4: f64 = simple_interpolate(v4,v8, fractional_z);

    let ii1: f64 = simple_interpolate(i1,i2,fractional_x);
    let ii2: f64 = simple_interpolate(i3,i4,fractional_x);

    return simple_interpolate(ii1 , ii2 , fractional_y);
}

pub fn perlin_noise (x: f64, y : f64, z: f64, wavelength: f64) -> f64
{
   return interpolatedNoise3D(x/wavelength, y/wavelength, z/wavelength);
}