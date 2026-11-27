//! Predefined material specifications in `Graphic3dNameOfMaterial` order.
//!
//! Each row provides both the legacy Phong colors (`colors`, `shininess`, `coefs`) and a
//! physical ASWF OpenPBR Surface v1.1 preset (`openpbr`).

/// Physical OpenPBR Surface v1.1 preset family for predefined CAD materials.
#[derive(Clone, Copy)]
pub(super) enum OpenPbrPreset {
    /// Matte/glossy dielectric base (`base.color`, `diffuse_roughness`, `specular.roughness`, `ior`, optional `coat_weight`).
    Dielectric {
        color: [f32; 3],
        diffuse_roughness: f32,
        roughness: f32,
        ior: f32,
        coat_weight: f32,
        coat_roughness: f32,
    },
    /// Conductor metal (`base.color` = F0, `specular.color` = F82 edge tint, `roughness`, `anisotropy`).
    Metal {
        f0: [f32; 3],
        f82: [f32; 3],
        roughness: f32,
        anisotropy: f32,
    },
    /// Fabric satin with microfiber fuzz and anisotropic sheen.
    Satin {
        color: [f32; 3],
        roughness: f32,
        anisotropy: f32,
        fuzz_weight: f32,
        fuzz_roughness: f32,
    },
    /// Translucent solid subsurface scattering (e.g., Jade).
    Subsurface {
        color: [f32; 3],
        sss_color: [f32; 3],
        weight: f32,
        radius: f32,
        radius_scale: [f32; 3],
        roughness: f32,
        ior: f32,
    },
    /// Self-emissive surface.
    Emissive {
        base_color: [f32; 3],
        emission_color: [f32; 3],
        luminance: f32,
        roughness: f32,
    },
    /// Transmissive dielectric with Beer-Lambert volume tint and optional chromatic dispersion.
    Glass {
        tint: [f32; 3],
        depth: f32,
        roughness: f32,
        dispersion_scale: f32,
        abbe_number: f32,
    },
    /// Thin-walled partially opaque dielectric sheet.
    Transparent { color: [f32; 3], opacity: f32 },
}

/// One predefined material row; unset members keep the defaults.
#[derive(Clone, Copy)]
pub(super) struct MaterialSpec {
    pub(super) physic: bool,
    pub(super) shininess: f32,
    pub(super) colors: [[f32; 3]; 4],
    pub(super) openpbr: OpenPbrPreset,
    /// `(RefractionIndex, TransparencyCoef)` of the transparent materials.
    pub(super) optics: Option<(f32, f32)>,
    /// `(AmbientCoef, DiffuseCoef)` when not `(0.25, 1)`.
    pub(super) coefs: Option<(f32, f32)>,
}

const fn spec(
    physic: bool,
    shininess: f32,
    colors: [[f32; 3]; 4],
    openpbr: OpenPbrPreset,
) -> MaterialSpec {
    MaterialSpec {
        physic,
        shininess,
        colors,
        openpbr,
        optics: None,
        coefs: None,
    }
}

const fn dielectric(
    color: [f32; 3],
    diffuse_roughness: f32,
    roughness: f32,
    ior: f32,
) -> OpenPbrPreset {
    OpenPbrPreset::Dielectric {
        color,
        diffuse_roughness,
        roughness,
        ior,
        coat_weight: 0.0,
        coat_roughness: 0.0,
    }
}

const fn metal(f0: [f32; 3], f82: [f32; 3], roughness: f32, anisotropy: f32) -> OpenPbrPreset {
    OpenPbrPreset::Metal {
        f0,
        f82,
        roughness,
        anisotropy,
    }
}

const fn glass(
    shininess: f32,
    colors: [[f32; 3]; 4],
    index: f32,
    preset: OpenPbrPreset,
) -> MaterialSpec {
    MaterialSpec {
        optics: Some((index, 0.80)),
        ..spec(true, shininess, colors, preset)
    }
}

const fn gray(value: f32) -> [f32; 3] {
    [value, value, value]
}

const BLACK: [f32; 3] = [0.0, 0.0, 0.0];

/// Predefined CAD material table in `Graphic3dNameOfMaterial` order.
pub(super) const MATERIAL_SPECS: [MaterialSpec; 26] = [
    // Brass
    spec(
        true,
        0.65,
        [
            [0.088428, 0.041081, 0.002090],
            [0.570482, 0.283555, 0.012335],
            [0.992, 0.941, 0.808],
            BLACK,
        ],
        metal([0.888, 0.720, 0.340], [0.990, 0.940, 0.800], 0.20, 0.0),
    ),
    // Bronze
    spec(
        true,
        0.65,
        [
            [0.037301, 0.014931, 0.004305],
            [0.468185, 0.153344, 0.027491],
            [0.590, 0.408, 0.250],
            BLACK,
        ],
        metal([0.710, 0.450, 0.230], [0.920, 0.780, 0.620], 0.26, 0.0),
    ),
    // Copper
    spec(
        true,
        0.65,
        [
            [0.030370, 0.006451, 0.001780],
            [0.323236, 0.059254, 0.007584],
            [0.950, 0.640, 0.540],
            BLACK,
        ],
        metal([0.955, 0.637, 0.538], [0.990, 0.900, 0.850], 0.18, 0.0),
    ),
    // Gold
    spec(
        true,
        0.80,
        [
            [0.073239, 0.043234, 0.009264],
            [0.525643, 0.295700, 0.010023],
            [1.000, 0.710, 0.290],
            BLACK,
        ],
        metal([0.944, 0.776, 0.373], [0.998, 0.981, 0.751], 0.12, 0.0),
    ),
    // Pewter
    spec(
        true,
        0.50,
        [
            [0.010979, 0.004795, 0.012335],
            [0.152583, 0.188174, 0.253972],
            [0.333, 0.333, 0.522],
            BLACK,
        ],
        metal([0.560, 0.570, 0.610], [0.780, 0.790, 0.820], 0.35, 0.0),
    ),
    // Plastered
    spec(
        false,
        0.01,
        [gray(0.13), gray(0.75), gray(0.003936), BLACK],
        dielectric(gray(0.75), 0.80, 0.85, 1.45),
    ),
    // Plastified
    spec(
        false,
        0.0078125,
        [gray(0.25), gray(0.24), gray(0.004896), BLACK],
        dielectric(gray(0.50), 0.10, 0.45, 1.50),
    ),
    // Silver
    spec(
        true,
        0.75,
        [
            [0.061465, 0.061465, 0.050876],
            [0.354692, 0.354692, 0.354692],
            [0.950, 0.930, 0.880],
            BLACK,
        ],
        metal([0.972, 0.960, 0.915], [0.999, 0.997, 0.990], 0.10, 0.0),
    ),
    // Steel
    spec(
        true,
        0.90,
        [
            [0.019607, 0.019607, 0.027212],
            [0.214041, 0.223414, 0.233022],
            [0.560, 0.570, 0.580],
            BLACK,
        ],
        metal([0.562, 0.565, 0.578], [0.820, 0.825, 0.840], 0.20, 0.0),
    ),
    // Stone
    MaterialSpec {
        coefs: Some((0.19 * 0.25, 0.75)),
        ..spec(
            true,
            0.17,
            [
                [0.030074, 0.020069, 0.013011],
                [0.522522, 0.318547, 0.183064],
                [0.98 * 0.08, 1.0 * 0.08, 0.60 * 0.08],
                BLACK,
            ],
            dielectric([0.522, 0.319, 0.183], 0.70, 0.75, 1.52),
        )
    },
    // Shiny_plastified
    spec(
        false,
        1.00,
        [gray(0.22), gray(0.50), gray(1.0), BLACK],
        OpenPbrPreset::Dielectric {
            color: gray(0.50),
            diffuse_roughness: 0.0,
            roughness: 0.25,
            ior: 1.50,
            coat_weight: 0.60,
            coat_roughness: 0.05,
        },
    ),
    // Satined
    spec(
        false,
        0.09375,
        [gray(0.165), gray(0.40), gray(0.162647), BLACK],
        OpenPbrPreset::Satin {
            color: gray(0.45),
            roughness: 0.35,
            anisotropy: 0.40,
            fuzz_weight: 0.65,
            fuzz_roughness: 0.35,
        },
    ),
    // Metalized
    spec(
        false,
        0.13,
        [BLACK, gray(0.47), gray(0.170645), BLACK],
        metal(gray(0.47), gray(0.75), 0.30, 0.0),
    ),
    // Ionized
    spec(
        false,
        0.05,
        [BLACK, gray(1.0), gray(0.342392), gray(1.0)],
        OpenPbrPreset::Emissive {
            base_color: gray(0.10),
            emission_color: gray(1.0),
            luminance: 3.0,
            roughness: 0.20,
        },
    ),
    // Chrome
    spec(
        true,
        0.90,
        [
            [0.033105, 0.033105, 0.041436],
            [0.263273, 0.263273, 0.263273],
            [0.975, 0.975, 0.975],
            BLACK,
        ],
        metal([0.550, 0.556, 0.554], [0.890, 0.895, 0.895], 0.05, 0.0),
    ),
    // Aluminium
    spec(
        true,
        0.75,
        [
            [0.073239, 0.073239, 0.073239],
            [0.318547, 0.318547, 0.318547],
            [0.910, 0.920, 0.920],
            BLACK,
        ],
        metal([0.913, 0.921, 0.925], [0.970, 0.975, 0.980], 0.22, 0.25),
    ),
    // Obsidian
    spec(
        true,
        0.3,
        [
            [0.004305, 0.003936, 0.005532],
            [0.028053, 0.024515, 0.041436],
            [0.333, 0.329, 0.346],
            BLACK,
        ],
        dielectric([0.028, 0.025, 0.041], 0.0, 0.12, 1.50),
    ),
    // Neon
    spec(
        true,
        0.05,
        [BLACK, BLACK, gray(0.62), [0.0, 0.787412, 0.142892]],
        OpenPbrPreset::Emissive {
            base_color: [0.0, 0.08, 0.02],
            emission_color: [0.0, 1.0, 0.46],
            luminance: 4.0,
            roughness: 0.15,
        },
    ),
    // Jade
    spec(
        true,
        0.10,
        [
            [0.016338, 0.040729, 0.021493],
            [0.252950, 0.767769, 0.354692],
            [0.316, 0.316, 0.316],
            BLACK,
        ],
        OpenPbrPreset::Subsurface {
            color: [0.21, 0.46, 0.23],
            sss_color: [0.25, 0.68, 0.34],
            weight: 0.65,
            radius: 0.30,
            radius_scale: [0.40, 0.90, 0.45],
            roughness: 0.15,
            ior: 1.66,
        },
    ),
    // Charcoal
    spec(
        true,
        0.01,
        [gray(0.003936), gray(0.019607), BLACK, BLACK],
        dielectric(gray(0.025), 0.90, 0.85, 1.50),
    ),
    // Water
    glass(
        0.90,
        [
            [0.170645, 0.170645, 0.191627],
            [0.003936, 0.003936, 0.006571],
            [0.380, 0.380, 0.380],
            BLACK,
        ],
        1.333,
        OpenPbrPreset::Glass {
            tint: [0.85, 0.95, 0.98],
            depth: 1.0,
            roughness: 0.02,
            dispersion_scale: 0.10,
            abbe_number: 55.0,
        },
    ),
    // Glass
    glass(
        0.50,
        [
            [0.263273, 0.290143, 0.290143],
            [0.003936, 0.006571, 0.006571],
            [0.920, 0.920, 0.920],
            BLACK,
        ],
        1.52,
        OpenPbrPreset::Glass {
            tint: [0.92, 0.98, 0.96],
            depth: 0.50,
            roughness: 0.03,
            dispersion_scale: 0.15,
            abbe_number: 58.0,
        },
    ),
    // Diamond
    glass(
        0.90,
        [gray(0.263273), gray(0.010023), gray(0.970), BLACK],
        2.418,
        OpenPbrPreset::Glass {
            tint: [0.98, 0.98, 1.00],
            depth: 0.0,
            roughness: 0.01,
            dispersion_scale: 1.00,
            abbe_number: 55.3,
        },
    ),
    // Transparent
    MaterialSpec {
        optics: Some((1.0, 0.80)),
        ..spec(
            true,
            0.90,
            [gray(0.263273), gray(0.010023), gray(0.970), BLACK],
            OpenPbrPreset::Transparent {
                color: gray(0.90),
                opacity: 0.20,
            },
        )
    },
    // Default
    spec(
        false,
        0.039,
        [gray(0.15), gray(0.65), BLACK, BLACK],
        dielectric(gray(0.65), 0.0, 0.35, 1.50),
    ),
    // UserDefined
    spec(
        true,
        0.039,
        [gray(0.1), gray(0.6), gray(0.2), BLACK],
        dielectric(gray(0.60), 0.0, 0.35, 1.50),
    ),
];
