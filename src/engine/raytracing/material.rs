
#[derive(Debug)]
pub struct Material {
    pub diffuse: f64,
    pub reflectance: f64,
    pub refraction: f64,
    pub reflect_fuzziness: f64,
}

impl Material {
    pub const METAL: Material = Material {
        diffuse: 0.0,
        reflectance: 0.5,
        refraction: 0.0,
        reflect_fuzziness: 0.10,
    };
    pub const MIRROR: Material = Material {
        diffuse: 0.0,
        reflectance: 0.5,
        refraction: 0.0,
        reflect_fuzziness: 0.0,
    };
    pub const RUBBER: Material = Material {
        diffuse: 0.5,
        reflectance: 0.0,
        refraction: 0.0,
        reflect_fuzziness: 0.0,
    };
    pub const GLASS: Material = Material {
        diffuse: 0.0,
        reflectance: 0.05,
        refraction: 1.6,
        reflect_fuzziness: 0.0,
    };
    pub const WATER: Material = Material {
        diffuse: 0.0,
        reflectance: 0.2,
        refraction: 1.33,
        reflect_fuzziness: 0.0,
    };
}