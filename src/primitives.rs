use once_cell::sync::Lazy;

use crate::Vertex;

// yes, this is just the raw data for a (subdivided) icosphere
// yes, there really is no better way to make just a plain sphere in opengl, can you believe that?
// if you know a way let me know!!

pub(crate) static CIRCLE_VERTS: Lazy<[Vertex; 128]> = Lazy::new(|| {
    let mut verts = [Vertex { position: (0.0, 0.0, 0.0), uv: (0.0, 0.0) }; 128];
    let len = verts.len();
    for (i, vert) in verts.iter_mut().enumerate() {
        vert.position.0 = ((i as f32 / len as f32) * std::f32::consts::TAU).sin();
        vert.position.1 = ((i as f32 / len as f32) * std::f32::consts::TAU).cos();
    }
    verts
});

pub(crate) const CIRCLE_INDICES: [u16; 128] = {
    let mut i = 0;
    let mut indices = [0; 128];
    while i < 128 {
        indices[i as usize] = i;
        i += 1;
    }
    indices
};

pub(crate) const BOX_VERTS: [Vertex; 8] = [
    Vertex { position: (0.0, 1.0, 0.0), uv: (0.0, 0.0) },
    Vertex { position: (1.0, 1.0, 0.0), uv: (0.0, 0.0) },
    Vertex { position: (0.0, 0.0, 0.0), uv: (0.0, 0.0) },
    Vertex { position: (1.0, 0.0, 0.0), uv: (0.0, 0.0) },
    Vertex { position: (0.0, 1.0, 1.0), uv: (0.0, 0.0) },
    Vertex { position: (1.0, 1.0, 1.0), uv: (0.0, 0.0) },
    Vertex { position: (0.0, 0.0, 1.0), uv: (0.0, 0.0) },
    Vertex { position: (1.0, 0.0, 1.0), uv: (0.0, 0.0) },
];

pub(crate) const BOX_INDICES: [u16; 24] = [0, 1, 1, 3, 3, 2, 2, 0, 0, 4, 1, 5, 3, 7, 2, 6, 4, 5, 5, 7, 7, 6, 6, 4u16];

pub(crate) const SPHERE_VERTS: [Vertex; 162] = [
    Vertex {
        position: (-0.6708191, -0.2763973, -0.6881907),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.6881894, -0.5257362, -0.4999969),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.8618032, -0.2763963, -0.42532396),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.4839714, -0.5023017, -0.7165645),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.4360068, -0.251152, -0.8641879),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.276388, -0.4472199, -0.8506492),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.4472105, -0.7236105, -0.52572864),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.2328215, -0.6575192, -0.7165631),
        uv: (0.0, 0.0),
    },
    Vertex { position: (-0.5877856, 0.0, -0.80901676), uv: (0.0, 0.0) },
    Vertex { position: (-0.8090184, 0.0, -0.5877833), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.3090172, -0.000000787155, -0.9510564),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.1624555, -0.8506544, -0.49999517),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.05279034, -0.7236117, -0.6881853),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.029639238, -0.5023019, -0.86418414),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.1552151, -0.2511515, -0.9554221),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.138199, -0.2763975, -0.9510548), uv: (0.0, 0.0) },
    Vertex { position: (-0.0, 0.0, -1.0), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.8310506, -0.5022987, -0.23885329),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.6381946, -0.72360927, -0.2628637),
        uv: (0.0, 0.0),
    },
    Vertex { position: (-0.7534417, -0.6575148, 0.0), uv: (0.0, 0.0) },
    Vertex { position: (-0.89442617, -0.44721556, 0.0), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.9566258, -0.2511494, -0.1476184),
        uv: (0.0, 0.0),
    },
    Vertex { position: (-0.95105785, 0.0, -0.3090126), uv: (0.0, 0.0) },
    Vertex {
        position: (-1.0, 0.000000623164, -0.000000360779),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.9566258, -0.2511494, 0.1476184),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.3618013, -0.8944284, -0.2628644),
        uv: (0.0, 0.0),
    },
    Vertex { position: (-0.5257298, -0.8506517, 0.0), uv: (0.0, 0.0) },
    Vertex { position: (-0.251147, -0.9679489, 0.0), uv: (0.0, 0.0) },
    Vertex {
        position: (0.1381987, -0.89442915, -0.42532057),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.07760661, -0.9679496, -0.23885268),
        uv: (0.0, 0.0),
    },
    Vertex { position: (-0.0, -1.0, 0.0), uv: (0.0, 0.0) },
    Vertex {
        position: (0.2628688, -0.5257377, -0.8090116),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.361805, -0.7236108, -0.5877792), uv: (0.0, 0.0) },
    Vertex {
        position: (0.5319409, -0.5023017, -0.6817124),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.44721556, -0.2763977, -0.8506482),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.3090172, 0.0, -0.9510564), uv: (0.0, 0.0) },
    Vertex { position: (0.5877856, 0.0, -0.8090167), uv: (0.0, 0.0) },
    Vertex {
        position: (0.4253227, -0.8506542, -0.3090113),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.6095467, -0.6575189, -0.4428564),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.20318088, -0.96794957, -0.14761779),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.447211, -0.8944285, -0.000000737955),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.20318088, -0.96794957, 0.14761779),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.6871586, -0.2511519, -0.6817153),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.7236073, -0.4472195, -0.5257253),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.8127292, -0.5023006, -0.2952377),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.8606976, -0.2511509, -0.4428575),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.8090187, -0.00000190229, -0.587783),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.95105785, 0.0, -0.3090126), uv: (0.0, 0.0) },
    Vertex {
        position: (0.6708176, -0.72361004, -0.16245769),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.670817, -0.7236106, 0.1624568), uv: (0.0, 0.0) },
    Vertex { position: (0.85064787, -0.5257359, 0.0), uv: (0.0, 0.0) },
    Vertex { position: (0.9472131, -0.276396, -0.1624575), uv: (0.0, 0.0) },
    Vertex { position: (1.0, 0.0, 0.0), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.8310506, -0.5022987, 0.2388534),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.6381947, -0.7236095, 0.2628628),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.6881894, -0.5257362, 0.4999969),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.8618031, -0.2763963, 0.4253239),
        uv: (0.0, 0.0),
    },
    Vertex { position: (-0.95105785, 0.0, 0.3090126), uv: (0.0, 0.0) },
    Vertex { position: (-0.8090184, 0.0, 0.5877833), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.3618012, -0.8944289, 0.2628625),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.4472106, -0.7236114, 0.5257271),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.07760661, -0.9679496, 0.23885268),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.1624555, -0.8506544, 0.49999517),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.1381968, -0.8944291, 0.4253213), uv: (0.0, 0.0) },
    Vertex { position: (-0.670819, -0.2763973, 0.6881907), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.4839714, -0.5023017, 0.7165645),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.2328215, -0.6575192, 0.7165631),
        uv: (0.0, 0.0),
    },
    Vertex { position: (-0.4360068, -0.251152, 0.8641879), uv: (0.0, 0.0) },
    Vertex { position: (-0.276388, -0.4472199, 0.8506492), uv: (0.0, 0.0) },
    Vertex { position: (-0.5877856, 0.0, 0.8090167), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.3090165, -0.000000491972, 0.9510567),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.052789085, -0.7236108, 0.6881862),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.029639238, -0.5023019, 0.86418414),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.1552151, -0.2511515, 0.9554221),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.138199, -0.2763975, 0.9510548), uv: (0.0, 0.0) },
    Vertex { position: (-0.0, 0.0, 1.0), uv: (0.0, 0.0) },
    Vertex { position: (0.4253227, -0.8506542, 0.3090113), uv: (0.0, 0.0) },
    Vertex { position: (0.3618035, -0.7236116, 0.5877792), uv: (0.0, 0.0) },
    Vertex { position: (0.6095467, -0.6575189, 0.4428564), uv: (0.0, 0.0) },
    Vertex { position: (0.5319409, -0.5023017, 0.6817125), uv: (0.0, 0.0) },
    Vertex { position: (0.2628688, -0.5257377, 0.8090117), uv: (0.0, 0.0) },
    Vertex {
        position: (0.44721556, -0.2763977, 0.8506484),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.3090172, 0.0, 0.9510564), uv: (0.0, 0.0) },
    Vertex { position: (0.5877856, 0.0, 0.80901676), uv: (0.0, 0.0) },
    Vertex { position: (0.8127292, -0.5023006, 0.2952377), uv: (0.0, 0.0) },
    Vertex { position: (0.9472131, -0.276396, 0.1624575), uv: (0.0, 0.0) },
    Vertex { position: (0.8606976, -0.2511509, 0.4428575), uv: (0.0, 0.0) },
    Vertex { position: (0.95105785, 0.0, 0.3090126), uv: (0.0, 0.0) },
    Vertex { position: (0.7236073, -0.4472195, 0.5257253), uv: (0.0, 0.0) },
    Vertex {
        position: (0.68715847, -0.2511519, 0.6817153),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.8090193, 0.0, 0.5877821), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.68715847, 0.2511519, -0.6817153),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.44721568, 0.2763968, -0.8506484),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.8606976, 0.2511509, -0.4428575),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.7236073, 0.4472195, -0.5257253),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.5319409, 0.5023017, -0.6817125),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.8127292, 0.5023006, -0.2952377),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.6095467, 0.6575189, -0.4428564),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.9472129, 0.2763966, -0.1624578),
        uv: (0.0, 0.0),
    },
    Vertex { position: (-0.9472131, 0.2763959, 0.1624576), uv: (0.0, 0.0) },
    Vertex { position: (-0.85064787, 0.5257359, 0.0), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.67081654, 0.7236108, -0.162457),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.4253227, 0.8506542, -0.3090113),
        uv: (0.0, 0.0),
    },
    Vertex { position: (-0.4472099, 0.8944291, 0.0), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.1381992, 0.2763968, -0.9510551),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.2628688, 0.5257377, -0.8090117),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.1552151, 0.2511515, -0.9554221), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.36180368, 0.72361225, -0.5877785),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.029639238, 0.5023019, -0.86418414),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.052789558, 0.7236124, -0.68818486),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.1381973, 0.8944299, -0.42531946),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.1624555, 0.8506544, -0.49999517),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.20318088, 0.96794957, -0.14761779),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.20318088, 0.96794957, 0.14761779),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.07760661, 0.9679496, -0.23885268),
        uv: (0.0, 0.0),
    },
    Vertex { position: (-0.0, 1.0, 0.0), uv: (0.0, 0.0) },
    Vertex { position: (0.4360068, 0.251152, -0.8641879), uv: (0.0, 0.0) },
    Vertex { position: (0.276388, 0.4472199, -0.8506492), uv: (0.0, 0.0) },
    Vertex { position: (0.2328215, 0.6575192, -0.7165631), uv: (0.0, 0.0) },
    Vertex { position: (0.4839714, 0.5023017, -0.7165645), uv: (0.0, 0.0) },
    Vertex {
        position: (0.4472092, 0.72361165, -0.5257282),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.67082036, 0.2763962, -0.68818974),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.8618041, 0.2763944, -0.42532316),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.6881894, 0.5257362, -0.4999969), uv: (0.0, 0.0) },
    Vertex {
        position: (0.3618003, 0.89442915, -0.2628629),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.251147, 0.9679489, 0.0), uv: (0.0, 0.0) },
    Vertex {
        position: (0.63819355, 0.72361004, -0.2628641),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.5257298, 0.8506517, 0.0), uv: (0.0, 0.0) },
    Vertex { position: (0.7534417, 0.6575148, 0.0), uv: (0.0, 0.0) },
    Vertex { position: (0.9566258, 0.2511494, -0.1476184), uv: (0.0, 0.0) },
    Vertex { position: (0.8310506, 0.5022987, -0.2388534), uv: (0.0, 0.0) },
    Vertex { position: (0.9566258, 0.2511494, 0.1476184), uv: (0.0, 0.0) },
    Vertex { position: (0.89442617, 0.44721556, 0.0), uv: (0.0, 0.0) },
    Vertex { position: (-0.8606976, 0.2511509, 0.4428575), uv: (0.0, 0.0) },
    Vertex { position: (-0.8127292, 0.5023006, 0.2952377), uv: (0.0, 0.0) },
    Vertex { position: (-0.67081654, 0.7236108, 0.162457), uv: (0.0, 0.0) },
    Vertex { position: (-0.6095467, 0.6575189, 0.4428564), uv: (0.0, 0.0) },
    Vertex { position: (-0.4253227, 0.8506542, 0.3090113), uv: (0.0, 0.0) },
    Vertex { position: (-0.7236073, 0.4472195, 0.5257253), uv: (0.0, 0.0) },
    Vertex { position: (-0.6871586, 0.2511519, 0.6817153), uv: (0.0, 0.0) },
    Vertex { position: (-0.447215, 0.2763972, 0.85064876), uv: (0.0, 0.0) },
    Vertex { position: (-0.5319409, 0.5023017, 0.6817124), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.1381973, 0.8944299, 0.42531946),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (0.07760661, 0.9679496, 0.23885268),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.36180368, 0.72361225, 0.5877785),
        uv: (0.0, 0.0),
    },
    Vertex {
        position: (-0.052789558, 0.7236124, 0.68818486),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.1624555, 0.8506544, 0.49999517), uv: (0.0, 0.0) },
    Vertex {
        position: (-0.13819848, 0.2763971, 0.9510551),
        uv: (0.0, 0.0),
    },
    Vertex { position: (-0.2628688, 0.5257377, 0.8090116), uv: (0.0, 0.0) },
    Vertex { position: (0.1552151, 0.2511515, 0.9554221), uv: (0.0, 0.0) },
    Vertex {
        position: (0.029639238, 0.5023019, 0.86418414),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.3618003, 0.89442915, 0.2628629), uv: (0.0, 0.0) },
    Vertex { position: (0.4472091, 0.7236116, 0.5257282), uv: (0.0, 0.0) },
    Vertex {
        position: (0.63819355, 0.72361004, 0.2628641),
        uv: (0.0, 0.0),
    },
    Vertex { position: (0.6881894, 0.5257362, 0.4999969), uv: (0.0, 0.0) },
    Vertex { position: (0.8618042, 0.2763962, 0.4253219), uv: (0.0, 0.0) },
    Vertex { position: (0.8310506, 0.5022987, 0.23885329), uv: (0.0, 0.0) },
    Vertex { position: (0.4360068, 0.251152, 0.8641879), uv: (0.0, 0.0) },
    Vertex { position: (0.276388, 0.4472199, 0.8506492), uv: (0.0, 0.0) },
    Vertex { position: (0.2328215, 0.6575192, 0.7165631), uv: (0.0, 0.0) },
    Vertex { position: (0.4839714, 0.5023017, 0.7165645), uv: (0.0, 0.0) },
    Vertex { position: (0.6708205, 0.2763974, 0.6881892), uv: (0.0, 0.0) },
];

pub(crate) const SPHERE_INDICES: [u16; 960] = [
    0, 1, 2, 0, 3, 1, 4, 5, 3, 1, 3, 6, 3, 7, 6, 8, 0, 9, 9, 0, 2, 4, 3, 0, 8, 4, 0, 8, 10, 4, 6, 7, 11, 3, 5, 7, 7, 12, 11, 5, 13, 7, 13, 12, 7, 14,
    13, 5, 15, 13, 14, 4, 14, 5, 10, 14, 4, 10, 16, 14, 17, 1, 18, 2, 1, 17, 17, 18, 19, 20, 17, 19, 21, 17, 20, 9, 2, 22, 22, 2, 21, 2, 17, 21, 23,
    22, 21, 23, 21, 24, 18, 6, 25, 1, 6, 18, 19, 18, 26, 18, 25, 26, 26, 25, 27, 6, 11, 25, 11, 28, 29, 25, 11, 29, 25, 29, 27, 27, 29, 30, 13, 31,
    12, 15, 31, 13, 31, 32, 12, 31, 33, 32, 34, 33, 31, 16, 15, 14, 35, 15, 16, 34, 31, 15, 35, 34, 15, 36, 34, 35, 12, 28, 11, 12, 32, 28, 32, 37,
    28, 33, 38, 32, 32, 38, 37, 28, 39, 29, 29, 39, 30, 28, 37, 39, 40, 39, 37, 41, 39, 40, 42, 33, 34, 42, 43, 33, 33, 43, 38, 44, 38, 43, 45, 44,
    43, 36, 42, 34, 36, 46, 42, 42, 45, 43, 46, 45, 42, 46, 47, 45, 48, 37, 38, 48, 38, 44, 40, 37, 48, 49, 40, 48, 49, 48, 50, 50, 48, 44, 51, 50,
    44, 51, 44, 45, 47, 51, 45, 52, 51, 47, 53, 20, 19, 24, 20, 53, 53, 19, 54, 55, 53, 54, 56, 53, 55, 24, 21, 20, 57, 23, 24, 24, 53, 56, 57, 24,
    56, 57, 56, 58, 54, 19, 26, 59, 26, 27, 54, 26, 59, 60, 54, 59, 55, 54, 60, 59, 27, 61, 61, 27, 30, 60, 59, 62, 62, 59, 61, 63, 62, 61, 56, 55,
    64, 64, 55, 65, 65, 55, 60, 65, 60, 66, 67, 65, 68, 58, 56, 64, 58, 64, 69, 64, 65, 67, 69, 64, 67, 70, 69, 67, 66, 60, 62, 68, 65, 66, 71, 66,
    62, 72, 68, 66, 72, 66, 71, 73, 68, 72, 73, 72, 74, 73, 67, 68, 70, 67, 73, 75, 70, 73, 41, 61, 30, 63, 61, 41, 41, 30, 39, 76, 41, 40, 76, 63,
    41, 71, 62, 63, 77, 71, 63, 77, 63, 76, 78, 77, 76, 79, 77, 78, 80, 72, 71, 74, 72, 80, 80, 71, 77, 79, 80, 77, 81, 80, 79, 75, 73, 74, 75, 74,
    82, 74, 80, 81, 82, 74, 81, 82, 81, 83, 76, 40, 49, 78, 76, 49, 78, 49, 84, 84, 49, 50, 85, 84, 50, 85, 50, 51, 52, 85, 51, 86, 84, 85, 87, 85,
    52, 87, 86, 85, 88, 78, 84, 86, 88, 84, 88, 79, 78, 89, 79, 88, 81, 79, 89, 86, 89, 88, 87, 90, 86, 83, 81, 89, 90, 89, 86, 90, 83, 89, 91, 8, 9,
    91, 92, 8, 93, 9, 22, 91, 9, 93, 94, 91, 93, 95, 92, 91, 94, 95, 91, 96, 94, 93, 97, 95, 94, 97, 94, 96, 98, 93, 22, 96, 93, 98, 98, 22, 23, 99,
    98, 23, 100, 98, 99, 100, 96, 98, 101, 96, 100, 102, 97, 101, 97, 96, 101, 103, 101, 135, 92, 10, 8, 92, 104, 10, 105, 104, 92, 104, 16, 10, 104,
    106, 16, 95, 105, 92, 107, 95, 97, 107, 105, 95, 105, 108, 104, 109, 108, 105, 102, 107, 97, 110, 107, 102, 109, 105, 107, 110, 109, 107, 111,
    109, 110, 102, 101, 103, 112, 102, 103, 112, 110, 102, 114, 110, 112, 115, 114, 112, 106, 35, 16, 116, 35, 106, 108, 106, 104, 108, 117, 106,
    117, 116, 106, 118, 108, 109, 111, 118, 109, 118, 117, 108, 118, 119, 117, 111, 120, 118, 116, 36, 35, 119, 121, 116, 116, 121, 36, 121, 46, 36,
    121, 122, 46, 117, 119, 116, 120, 119, 118, 123, 122, 121, 119, 123, 121, 120, 123, 119, 114, 111, 110, 124, 120, 111, 114, 124, 111, 115, 125,
    114, 125, 124, 114, 126, 123, 120, 124, 126, 120, 125, 127, 124, 127, 126, 124, 127, 128, 126, 122, 47, 46, 122, 129, 47, 130, 129, 122, 129, 52,
    47, 132, 131, 129, 123, 130, 122, 126, 130, 123, 130, 132, 129, 128, 132, 130, 128, 130, 126, 99, 23, 57, 133, 99, 57, 134, 100, 99, 134, 99,
    133, 136, 134, 138, 101, 100, 135, 135, 100, 134, 135, 134, 136, 103, 135, 137, 137, 135, 136, 133, 57, 58, 138, 133, 139, 133, 58, 139, 139, 58,
    69, 140, 139, 69, 138, 134, 133, 136, 138, 141, 141, 139, 140, 141, 138, 139, 136, 141, 144, 112, 103, 113, 113, 103, 137, 115, 112, 113, 115,
    113, 143, 113, 142, 143, 137, 136, 144, 137, 144, 142, 113, 137, 142, 142, 144, 145, 142, 145, 146, 140, 69, 70, 147, 140, 70, 148, 140, 147,
    147, 70, 75, 149, 147, 75, 148, 141, 140, 144, 141, 148, 150, 148, 147, 144, 148, 145, 145, 148, 150, 115, 143, 125, 143, 151, 125, 143, 142,
    146, 143, 146, 151, 146, 152, 151, 125, 151, 127, 127, 153, 128, 151, 153, 127, 152, 154, 153, 151, 152, 153, 131, 52, 129, 131, 87, 52, 155, 90,
    87, 131, 155, 87, 156, 155, 131, 132, 156, 131, 128, 156, 132, 153, 156, 128, 156, 154, 155, 153, 154, 156, 149, 75, 82, 149, 82, 157, 150, 147,
    149, 158, 150, 149, 158, 149, 157, 146, 145, 159, 145, 150, 159, 146, 159, 152, 159, 150, 158, 159, 158, 160, 157, 82, 83, 160, 157, 161, 155,
    161, 90, 161, 157, 83, 161, 83, 90, 160, 158, 157, 159, 160, 152, 154, 161, 155, 154, 160, 161, 152, 160, 154,
];

pub(crate) const ARROWHEAD_VERTS: [Vertex; 9] = [
Vertex { position: (0.0, 0.0, -0.3535533), uv: (0.5, 1.0) },
Vertex { position: (0.0, 2.0, 0.0), uv: (0.5, 0.5) },
Vertex { position: (0.25, 0.0, -0.25), uv: (0.853553, 0.853553) },
Vertex { position: (0.3535533, 0.0, 0.0), uv: (1.0, 0.5) },
Vertex { position: (0.25, 0.0, 0.25), uv: (0.853553, 0.146447) },
Vertex { position: (-0.0, 0.0, 0.3535533), uv: (0.5, 0.0) },
Vertex { position: (-0.25, 0.0, 0.25), uv: (0.146447, 0.146447) },
Vertex { position: (-0.3535533, 0.0, -0.0), uv: (0.0, 0.5) },
Vertex { position: (-0.25, 0.0, -0.25), uv: (0.146446, 0.853553) }
];
pub(crate) const ARROWHEAD_INDICES: [u16; 42] = [0, 1, 2, 2, 1, 3, 3, 1, 4, 4, 1, 5, 5, 1, 6, 6, 1, 7, 4, 6, 8, 7, 1, 8, 8, 1, 0, 8, 0, 2, 2, 3, 4, 4, 5, 6, 6, 7, 8, 8, 2, 4];

