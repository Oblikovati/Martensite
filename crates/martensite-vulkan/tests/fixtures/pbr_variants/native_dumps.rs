// Native dumps in DUMPS order; the reference scene pbr_variants/pbr_variants.
const NATIVE_LLVMPIPE: [&[u8]; 9] = [
    include_bytes!("llvmpipe/pbr-clip1.ppm"),
    include_bytes!("llvmpipe/pbr-clip2.ppm"),
    include_bytes!("llvmpipe/pbr-clipn.ppm"),
    include_bytes!("llvmpipe/pbr-chain.ppm"),
    include_bytes!("llvmpipe/pbr-edges.ppm"),
    include_bytes!("llvmpipe/pbr-hollow-edges.ppm"),
    include_bytes!("llvmpipe/pbrfacet-edges.ppm"),
    include_bytes!("llvmpipe/pbr-oit-weighted.ppm"),
    include_bytes!("llvmpipe/pbr-oit-peeling.ppm"),
];
const NATIVE_AMD: [&[u8]; 9] = [
    include_bytes!("amd/pbr-clip1.ppm"),
    include_bytes!("amd/pbr-clip2.ppm"),
    include_bytes!("amd/pbr-clipn.ppm"),
    include_bytes!("amd/pbr-chain.ppm"),
    include_bytes!("amd/pbr-edges.ppm"),
    include_bytes!("amd/pbr-hollow-edges.ppm"),
    include_bytes!("amd/pbrfacet-edges.ppm"),
    include_bytes!("amd/pbr-oit-weighted.ppm"),
    include_bytes!("amd/pbr-oit-peeling.ppm"),
];
