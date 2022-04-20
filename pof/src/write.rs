use std::{
    f32::consts::PI,
    io::{self, Write},
};

use byteorder::{WriteBytesExt, LE};
use dae_parser::Document;
use glm::{Mat4x4, Vec3};
extern crate nalgebra_glm as glm;

use crate::{
    BspData, BspNode, Dock, EyePoint, GlowPointBank, Insignia, Model, ObjVec, ObjectId, Path, ShieldData, ShieldNode, SpecialPoint, SubObject,
    ThrusterBank, Turret, Vec3d, Version, WeaponHardpoint,
};

pub(crate) trait Serialize {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()>;
}

impl Serialize for () {
    fn write_to(&self, _: &mut impl Write) -> io::Result<()> {
        Ok(())
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        if let Some(x) = self {
            x.write_to(w)?
        }
        Ok(())
    }
}

impl Serialize for String {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        (**self).write_to(w)
    }
}
impl Serialize for str {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        let padding_length = (3_usize.wrapping_sub(self.len()) % 4) + 1;
        w.write_u32::<LE>((self.len() + padding_length) as u32)?;

        w.write_all(self.as_bytes())?;

        w.write_all(&[0; 4][..padding_length])
    }
}

impl<A: Serialize, B: Serialize, C: Serialize> Serialize for (A, B, C) {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        self.0.write_to(w)?;
        self.1.write_to(w)?;
        self.2.write_to(w)
    }
}
impl<A: Serialize, B: Serialize> Serialize for (A, B) {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        self.0.write_to(w)?;
        self.1.write_to(w)
    }
}
impl<T: Serialize> Serialize for Vec<T> {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        (**self).write_to(w)
    }
}
impl<T: Serialize> Serialize for [T] {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        w.write_u32::<LE>((self.len()) as u32)?;

        for item in self {
            item.write_to(w)?;
        }
        Ok(())
    }
}

impl Serialize for u32 {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        w.write_u32::<LE>(*self)
    }
}
impl Serialize for u16 {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        w.write_u16::<LE>(*self)
    }
}
impl Serialize for u8 {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        w.write_u8(*self)
    }
}
impl Serialize for f32 {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        w.write_f32::<LE>(*self)
    }
}
impl Serialize for i32 {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        w.write_i32::<LE>(*self)
    }
}
impl Serialize for crate::NormalVec3 {
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        self.0.write_to(w)
    }
}

fn align_buf(buf: &mut Vec<u8>) -> io::Result<()> {
    let padding_length = buf.len().wrapping_neg() % 4;
    buf.write_all(&[0; 4][..padding_length])
}

pub(crate) fn write_shield_node(buf: &mut Vec<u8>, shield_node: &ShieldNode, chunk_type_is_u8: bool) -> io::Result<()> {
    macro_rules! write_chunk_type {
        ($n:expr) => {
            if chunk_type_is_u8 {
                buf.write_u8($n as u8)
            } else {
                buf.write_u32::<LE>($n)
            }
        };
    }
    match shield_node {
        ShieldNode::Split { bbox, front, back } => {
            let base = buf.len();
            write_chunk_type!(ShieldNode::SPLIT)?;
            let chunk_size_pointer = Fixup::new(buf, base)?;

            bbox.sanitize().write_to(buf)?;
            let front_offset = Fixup::new(buf, base)?;
            let back_offset = Fixup::new(buf, base)?;

            front_offset.finish(buf);
            write_shield_node(buf, front, chunk_type_is_u8)?;

            back_offset.finish(buf);
            write_shield_node(buf, back, chunk_type_is_u8)?;

            chunk_size_pointer.finish(buf);
        }
        ShieldNode::Leaf { bbox, poly_list } => {
            let base = buf.len();
            write_chunk_type!(ShieldNode::LEAF)?;
            let chunk_size_pointer = Fixup::new(buf, base)?;

            bbox.sanitize().write_to(buf)?;
            poly_list.write_to(buf)?;
            chunk_size_pointer.finish(buf);
        }
    }
    Ok(())
}

#[must_use]
struct Fixup {
    location: usize,
    base: usize,
}
impl Fixup {
    fn new(buf: &mut Vec<u8>, base: usize) -> io::Result<Self> {
        let n = buf.len();
        buf.write_u32::<LE>(0)?;
        Ok(Self { location: n, base })
    }
    fn finish(self, buf: &mut Vec<u8>) -> u32 {
        let size = (buf.len() - self.base) as u32;
        buf[self.location..][..4].copy_from_slice(&u32::to_le_bytes(size));
        size
    }
}

pub(crate) fn write_bsp_data(buf: &mut Vec<u8>, version: Version, bsp_data: &BspData) -> io::Result<()> {
    const MAX_NORMS_PER_VERT: u8 = 0xCC; //u8::MAX;

    fn write_bsp_node(buf: &mut Vec<u8>, verts: &[Vec3d], version: Version, bsp_node: &BspNode) -> io::Result<()> {
        match bsp_node {
            BspNode::Split { bbox, front, back } => {
                let base = buf.len();
                if version >= Version::V23_00 {
                    buf.write_u32::<LE>(BspData::SORTNORM2)?;
                } else {
                    buf.write_u32::<LE>(BspData::SORTNORM)?;
                }
                let chunk_size_pointer = Fixup::new(buf, base)?;

                if version >= Version::V23_00 {
                    let front_offset = Fixup::new(buf, base)?;
                    let back_offset = Fixup::new(buf, base)?;

                    bbox.write_to(buf)?;

                    if !matches!(**front, BspNode::Empty) {
                        front_offset.finish(buf);
                        write_bsp_node(buf, verts, version, front)?;
                    } // otherwise front_offset = 0

                    if !matches!(**back, BspNode::Empty) {
                        back_offset.finish(buf);
                        write_bsp_node(buf, verts, version, back)?;
                    } // otherwise back_offset = 0

                    chunk_size_pointer.finish(buf);
                } else {
                    Vec3d::ZERO.write_to(buf)?; // plane_normal: unused
                    Vec3d::ZERO.write_to(buf)?; // plane_point: unused

                    buf.write_u32::<LE>(0)?;

                    let front_offset = Fixup::new(buf, base)?;
                    let back_offset = Fixup::new(buf, base)?;

                    buf.write_u32::<LE>(0)?;
                    buf.write_u32::<LE>(0)?;
                    buf.write_u32::<LE>(0)?;
                    if version >= Version::V20_00 {
                        bbox.sanitize().write_to(buf)?;
                    }

                    if !matches!(**front, BspNode::Empty) {
                        front_offset.finish(buf);
                        write_bsp_node(buf, verts, version, front)?;
                    } // otherwise front_offset = 0

                    if !matches!(**back, BspNode::Empty) {
                        back_offset.finish(buf);
                        write_bsp_node(buf, verts, version, back)?;
                    } // otherwise back_offset = 0

                    chunk_size_pointer.finish(buf);
                }
            }
            BspNode::Leaf { bbox, poly } => {
                let base = buf.len();
                if version >= Version::V23_00 {
                    buf.write_u32::<LE>(BspData::TMAPPOLY2)?;
                    let chunk_size_pointer = Fixup::new(buf, base)?;

                    bbox.write_to(buf)?;

                    poly.normal.write_to(buf)?;
                    poly.texture.write_to(buf)?;
                    (poly.verts.len() as u32).write_to(buf)?;

                    for vert in &poly.verts {
                        vert.vertex_id.0.write_to(buf)?;
                        vert.normal_id.0.write_to(buf)?;
                        vert.uv.write_to(buf)?;
                    }

                    chunk_size_pointer.finish(buf);
                } else {
                    buf.write_u32::<LE>(BspData::BOUNDBOX)?;
                    let chunk_size_pointer = Fixup::new(buf, base)?;

                    bbox.sanitize().write_to(buf)?;
                    chunk_size_pointer.finish(buf);

                    let base = buf.len();
                    buf.write_u32::<LE>(BspData::TMAPPOLY)?;
                    let chunk_size_pointer = Fixup::new(buf, base)?;

                    poly.normal.write_to(buf)?;
                    Vec3d::ZERO.write_to(buf)?;
                    0f32.write_to(buf)?;
                    (poly.verts.len() as u32).write_to(buf)?;
                    poly.texture.write_to(buf)?;

                    for vert in &poly.verts {
                        u16::try_from(vert.vertex_id.0).unwrap().write_to(buf)?;
                        u16::try_from(vert.normal_id.0).unwrap().write_to(buf)?;
                        vert.uv.write_to(buf)?;
                    }

                    chunk_size_pointer.finish(buf);
                }
            }
            BspNode::Empty => {}
        }

        if version < Version::V23_00 {
            buf.write_u32::<LE>(BspData::ENDOFBRANCH)?;
            buf.write_u32::<LE>(0)?;
        }

        Ok(())
    }

    let base = buf.len();
    buf.write_u32::<LE>(BspData::DEFFPOINTS)?;

    let chunk_size_pointer = Fixup::new(buf, base)?;

    (bsp_data.verts.len() as u32).write_to(buf)?;
    (bsp_data.norms.len() as u32).write_to(buf)?;
    let vertex_data_offset = Fixup::new(buf, base)?;

    let mut num_norms = bsp_data.norms.len();
    for _ in 0..bsp_data.verts.len() {
        let old_num_norms = num_norms;
        num_norms = num_norms.saturating_sub(MAX_NORMS_PER_VERT.into());

        buf.push((old_num_norms - num_norms) as u8);
    }
    assert!(num_norms == 0);

    align_buf(buf)?;

    vertex_data_offset.finish(buf);

    let mut norm_iter = bsp_data.norms.iter();
    let mut num_norms = bsp_data.norms.len();
    for &vert in &bsp_data.verts {
        vert.write_to(buf)?;

        let old_num_norms = num_norms;
        num_norms = num_norms.saturating_sub(MAX_NORMS_PER_VERT.into());

        for _ in num_norms..old_num_norms {
            norm_iter.next().unwrap().write_to(buf)?;
        }
    }

    chunk_size_pointer.finish(buf);

    write_bsp_node(buf, &bsp_data.verts, version, &bsp_data.collision_tree)?;

    Ok(())
}

fn write_chunk_raw(w: &mut impl Write, chunk_name: &[u8; 4], f: impl FnOnce(&mut Vec<u8>) -> io::Result<()>) -> io::Result<()> {
    w.write_all(chunk_name)?;
    //println!("writing chunk {}", std::str::from_utf8(chunk_name).unwrap());

    let mut buf = vec![];

    f(&mut buf)?;

    w.write_u32::<LE>((buf.len()) as u32)?;
    //println!("writing chunk length {}", buf.len());
    w.write_all(&buf)
}

fn write_chunk<T: Serialize>(w: &mut impl Write, chunk_name: &[u8; 4], data: Option<&T>) -> io::Result<()> {
    if let Some(data) = data {
        write_chunk_raw(w, chunk_name, |w| data.write_to(w))?
    }
    Ok(())
}

fn write_chunk_vec<T: Serialize>(w: &mut impl Write, chunk_name: &[u8; 4], data: &[T]) -> io::Result<()> {
    if !data.is_empty() {
        write_chunk_raw(w, chunk_name, |w| data.write_to(w))?
    }
    Ok(())
}

fn write_subobjects(w: &mut impl Write, chunk_name: &[u8; 4], objects: &[SubObject]) -> io::Result<()> {
    fn write_subobject(w: &mut impl Write, chunk_name: [u8; 4], objects: &[SubObject], written: &mut [bool], id: ObjectId) -> io::Result<()> {
        if !written[id.0 as usize] {
            let obj = &objects[id.0 as usize];
            // ensure parents are written before children
            if let Some(parent) = obj.parent {
                write_subobject(w, chunk_name, objects, written, parent)?;
            }
            write_chunk(w, &chunk_name, Some(obj))?;
            written[id.0 as usize] = true;
        }
        Ok(())
    }

    let mut written = vec![false; objects.len()];
    for i in 0..objects.len() as u32 {
        write_subobject(w, *chunk_name, objects, &mut written, ObjectId(i))?;
    }
    Ok(())
}

impl Model {
    pub fn write(&self, w: &mut impl Write) -> io::Result<()> {
        // set the version to be using be all the serializers
        crate::VERSION.with(|f| {
            f.set(self.version);
        });
        w.write_all(b"PSPO")?;

        w.write_i32::<LE>(Version::V23_00.into())?;
        //w.write_i32::<LE>(self.version.into())?;

        write_chunk_raw(w, if self.version >= Version::V21_16 { b"HDR2" } else { b"OHDR" }, |w| {
            if self.version >= Version::V21_16 {
                self.header.max_radius.write_to(w)?;
                self.header.obj_flags.write_to(w)?;
                self.header.num_subobjects.write_to(w)?;
            } else {
                self.header.num_subobjects.write_to(w)?;
                self.header.max_radius.write_to(w)?;
                self.header.obj_flags.write_to(w)?;
            }
            self.header.bbox.sanitize().write_to(w)?;
            self.header.detail_levels.write_to(w)?;
            self.sub_objects
                .iter()
                .filter(|obj| obj.is_debris_model)
                .map(|obj| obj.obj_id)
                .collect::<Vec<_>>()
                .write_to(w)?;
            if self.version >= Version::V20_09 {
                self.header.mass.write_to(w)?;
                self.header.center_of_mass.write_to(w)?;
                self.header.moment_of_inertia.write_to(w)?;
            } else if self.version >= Version::V19_03 {
                let vol_mass = (self.header.mass / 4.65).powf(1.5);
                vol_mass.write_to(w)?;
                self.header.center_of_mass.write_to(w)?;
                let mut moi = self.header.moment_of_inertia;
                moi *= self.header.mass / vol_mass;
                moi.write_to(w)?;
            }
            if self.version >= Version::V20_14 {
                self.header.cross_sections.write_to(w)?;
            }
            if self.version >= Version::V20_07 {
                self.header.bsp_lights.write_to(w)?;
            }
            Ok(())
        })?;
        write_subobjects(w, if self.version >= Version::V21_16 { b"OBJ2" } else { b"SOBJ" }, &self.sub_objects)?;
        write_chunk_vec(w, b"TXTR", &self.textures)?;
        write_chunk_vec(w, b"PATH", &self.paths)?;
        write_chunk_vec(w, b"SPCL", &self.special_points)?;
        write_chunk_vec(w, b"EYE ", &self.eye_points)?;
        if !self.primary_weps.is_empty() {
            write_chunk(w, b"GPNT", Some(&self.primary_weps))?;
        }
        if !self.secondary_weps.is_empty() {
            write_chunk(w, b"MPNT", Some(&self.secondary_weps))?;
        }
        if !self.turrets.is_empty() {
            write_chunk(w, b"TGUN", Some(&self.turrets))?;
        }
        write_chunk_vec(w, b"FUEL", &self.thruster_banks)?;
        write_chunk_vec(w, b"GLOW", &self.glow_banks)?;
        if !self.comments.is_empty() {
            write_chunk_raw(w, b"PINF", |w| {
                let padding_length = (3_usize.wrapping_sub(self.comments.len()) % 4) + 1;
                w.write_all(self.comments.as_bytes())?;

                w.write_all(&[0; 4][..padding_length])
            })?;
        }
        write_chunk_vec(w, b"DOCK", &self.docking_bays)?;
        write_chunk_vec(w, b"INSG", &self.insignias)?;
        if let Some(shield_data) = &self.shield_data {
            write_chunk_raw(w, b"SHLD", |w| {
                shield_data.verts.write_to(w)?;
                shield_data.polygons.write_to(w)
            })?;

            if self.version >= Version::V21_18 {
                write_chunk(w, if self.version >= Version::V22_00 { b"SLC2" } else { b"SLDC" }, shield_data.collision_tree.as_ref())?;
            }
        }
        if self.visual_center != Vec3d::default() {
            write_chunk(w, b"ACEN", Some(&self.visual_center))?;
        }

        Ok(())
    }
}

// ==============================================================================
// DAE Writing
// ==============================================================================

use dae_parser::*;

// turns a direction vector into a dae Rotate struct
// mostly for the purposes of storing a normal into a node's transform
fn vec_to_rotation(vec: &Vec3d, up: UpAxis) -> Rotate {
    let v1 = glm::Vec3::from(*vec).normalize();
    let v2 = glm::Vec3::z_axis();
    let mut cross = v1.cross(&v2);
    if cross.magnitude() < 0.001 {
        cross = *glm::Vec3::y_axis() // forward for DAE
    }
    let axis = Vec3d::from(cross.normalize()).to_coord(up);
    Rotate::new(axis.into(), v1.dot(&v2).acos() * (180.0 / PI))
}

// turns properties into a series of dae nodes
fn make_properties_node(properties: &String, id: String) -> Node {
    let mut node = Node::new(format!("#{}properties", id), Some(format!("#{}properties", id)));

    for substr in properties.split('\n') {
        node.children
            .push(Node::new(format!("#{}:{}", id, substr), Some(format!("{}:{}", id, substr.trim_end()))));
        // trailing spaces can mess up parsing, so remove them
    }

    node
}

fn make_thrusters_node(thruster_banks: &[ThrusterBank], up: UpAxis) -> Node {
    let mut node = Node::new("#thrusters", Some(format!("#thrusters")));

    for (i, bank) in thruster_banks.iter().enumerate() {
        let mut bank_node = Node::new(format!("#t-bank{}", i), Some(format!("#t-bank{}", i)));

        for (j, point) in bank.glows.iter().enumerate() {
            let mut point_node = Node::new(format!("#tb{}-point{}", i, j), Some(format!("#tb{}-point{}", i, j)));
            let radius = point.radius;
            let pos = point.position.to_coord(up);
            point_node.push_transform(Translate::new(pos.into()));
            point_node.push_transform(vec_to_rotation(&point.normal, up));
            point_node.push_transform(Scale::new([radius, radius, radius]));

            bank_node.children.push(point_node);
        }

        if !bank.properties.is_empty() {
            bank_node.children.push(make_properties_node(&bank.properties, format!("tb{}-", i)));
        }

        node.children.push(bank_node);
    }

    node
}

fn make_paths_node(paths: &[Path], up: UpAxis) -> Node {
    let mut node = Node::new("#paths", Some(format!("#paths")));

    for (i, path) in paths.iter().enumerate() {
        let mut path_node = Node::new(format!("#p{}", i), Some(format!("#path{}", i)));

        for (j, point) in path.points.iter().enumerate() {
            let mut point_node = Node::new(format!("#path{}-{}", i, j), Some(format!("#p{}-point{}", i, j)));
            let radius = point.radius;
            let pos = point.position.to_coord(up);
            point_node.push_transform(Translate::new(pos.into()));
            point_node.push_transform(Scale::new([radius, radius, radius]));

            path_node.children.push(point_node);
        }

        path_node
            .children
            .push(Node::new(format!("#p{}-name", i), Some(format!("#p{}-name:{}", i, path.name))));

        path_node
            .children
            .push(Node::new(format!("#p{}-parent", i), Some(format!("#p{}-parent:{}", i, path.parent))));

        node.children.push(path_node);
    }

    node
}

fn make_weapons_node(weapons: &[Vec<WeaponHardpoint>], kind: &str, up: UpAxis) -> Node {
    let mut node = Node::new(format!("#{} weapons", kind), Some(format!("#{} weapons", kind)));

    for (i, bank) in weapons.iter().enumerate() {
        let mut bank_node = Node::new(format!("#w{}-bank{}", &kind[0..1], i), Some(format!("#w{}-bank{}", &kind[0..1], i)));

        for (j, point) in bank.iter().enumerate() {
            let mut point_node = Node::new(format!("#w{}b{}-point{}", &kind[0..1], i, j), Some(format!("#w{}b{}-point{}", &kind[0..1], i, j)));
            let pos = point.position.to_coord(up);
            point_node.push_transform(Translate::new(pos.into()));
            point_node.push_transform(vec_to_rotation(&point.normal.0, up));

            if point.offset != 0.0 {
                point_node.children.push(Node::new(
                    format!("#w{}b{}-point{}-offset", &kind[0..1], i, j),
                    Some(format!("#w{}b{}-point{}-offset:{}", &kind[0..1], i, j, point.offset)),
                ));
            }

            bank_node.children.push(point_node);
        }

        node.children.push(bank_node);
    }

    node
}

fn make_docking_bays_node(docks: &[Dock], up: UpAxis) -> Node {
    let mut node = Node::new(format!("#docking bays"), Some(format!("#docking bays")));

    for (i, dock) in docks.iter().enumerate() {
        let mut bay_node = Node::new(format!("#bay{}", i), Some(format!("#bay{}", i)));

        let fvec: Vec3 = dock.fvec.0.to_coord(up).into();
        let uvec = dock.uvec.0.to_coord(up).into();
        let mat = nalgebra::Matrix::from_columns(&[fvec.cross(&uvec), fvec, uvec]);
        let mut mat: Mat4x4 = glm::mat3_to_mat4(&mat);
        mat.append_translation_mut(&dock.position.to_coord(up).into());
        bay_node.push_transform(mat);

        if dock.path.is_some() {
            bay_node
                .children
                .push(Node::new(format!("#d{}-path", i), Some(format!("#d{}-path:{}", i, dock.path.unwrap().0 as u32))));
        }

        if !dock.properties.is_empty() {
            bay_node.children.push(make_properties_node(&dock.properties, format!("d{}-", i)));
        }

        node.children.push(bay_node);
    }

    node
}

fn make_glows_node(glows: &[GlowPointBank], up: UpAxis) -> Node {
    let mut node = Node::new("#glows", Some(format!("#glows")));

    for (i, glow_bank) in glows.iter().enumerate() {
        let mut bank_node = Node::new(format!("#g{}", i), Some(format!("#glowbank{}", i)));

        for (j, point) in glow_bank.glow_points.iter().enumerate() {
            let mut point_node = Node::new(format!("#g{}-{}", i, j), Some(format!("#g{}-point{}", i, j)));
            let radius = point.radius;
            let pos = point.position.to_coord(up);
            point_node.push_transform(Translate::new(pos.into()));
            if point.normal.is_null() {
                point_node.name = Some(format!("#g{}-omnipoint{}", i, j));
            } else {
                point_node.push_transform(vec_to_rotation(&point.normal, up));
            }
            point_node.push_transform(Scale::new([radius, radius, radius]));

            bank_node.children.push(point_node);
        }

        bank_node
            .children
            .push(Node::new(format!("#g{}-type", i), Some(format!("#g{}-type:{}", i, glow_bank.glow_type))));

        bank_node
            .children
            .push(Node::new(format!("#g{}-lod", i), Some(format!("#g{}-lod:{}", i, glow_bank.lod))));

        bank_node
            .children
            .push(Node::new(format!("#g{}-parent", i), Some(format!("#g{}-parent:{}", i, glow_bank.obj_parent.0))));

        bank_node
            .children
            .push(Node::new(format!("#g{}-ontime", i), Some(format!("#g{}-ontime:{}", i, glow_bank.on_time))));

        bank_node
            .children
            .push(Node::new(format!("#g{}-offtime", i), Some(format!("#g{}-offtime:{}", i, glow_bank.off_time))));

        bank_node
            .children
            .push(Node::new(format!("#g{}-disptime", i), Some(format!("#g{}-disptime:{}", i, glow_bank.disp_time))));

        if !glow_bank.properties.is_empty() {
            bank_node.children.push(make_properties_node(&glow_bank.properties, format!("g{}-", i)));
        }

        node.children.push(bank_node);
    }

    node
}

fn make_specials_node(special_points: &[SpecialPoint], up: UpAxis) -> Node {
    let mut node = Node::new(format!("#special points"), Some(format!("#special points")));

    for (i, point) in special_points.iter().enumerate() {
        let mut point_node = Node::new(format!("#s{}", i), Some(format!("#s{}:{}", i, point.name)));

        let radius = point.radius;
        let pos = point.position.to_coord(up);
        point_node.push_transform(Translate::new(pos.into()));
        point_node.push_transform(Scale::new([radius, radius, radius]));

        if !point.properties.is_empty() {
            point_node.children.push(make_properties_node(&point.properties, format!("s{}-", i)));
        }

        node.children.push(point_node);
    }

    node
}

fn make_eyes_node(eye_points: &[EyePoint], up: UpAxis) -> Node {
    let mut node = Node::new(format!("#eye points"), Some(format!("#eye points")));

    for (i, point) in eye_points.iter().enumerate() {
        let mut point_node = Node::new(format!("#e{}", i), Some(format!("#e-point{}", i)));

        let pos = point.offset.to_coord(up);
        point_node.push_transform(Translate::new(pos.into()));
        point_node.push_transform(vec_to_rotation(&point.normal.0, up));

        point_node
            .children
            .push(Node::new(format!("#e{}-parent", i), Some(format!("#e{}-parent:{}", i, point.attached_subobj.0))));

        node.children.push(point_node);
    }

    node
}

fn make_visual_center_node(visual_center: &Vec3d, up: UpAxis) -> Node {
    let mut node = Node::new(format!("#visual-center"), Some(format!("#visual-center")));

    node.push_transform(Translate::new(visual_center.to_coord(up).into()));

    node
}

fn make_insignia_node(insignia: &Insignia, geometries: &mut Vec<Geometry>, id: usize, up: UpAxis) -> Node {
    let geo_id = format!("insig{}-geometry", id);
    let pos_id = format!("insig{}-geometry-position", id);
    let vert_id = format!("insig{}-geometry-vertex", id);
    let pos_array_id = format!("{}-array", pos_id);

    let mut positions = vec![];
    for vert in &insignia.vertices {
        positions.extend_from_slice(&<[_; 3]>::from(vert.to_coord(up)));
    }

    let mut tricount = 0;
    let mut indices = vec![];

    for (polyvert1, polyvert2, polyvert3) in &insignia.faces {
        indices.push(polyvert1.vertex_id.0 as _);

        // intentional swizzle
        indices.push(polyvert3.vertex_id.0 as _);

        indices.push(polyvert2.vertex_id.0 as _);

        tricount += 1;
    }

    let instance = Instance::<Geometry>::new(Url::Fragment(geo_id.clone()));

    geometries.push(Geometry::new_mesh(
        geo_id,
        vec![Source::new_local(
            pos_id.clone(),
            Param::new_xyz(),
            ArrayElement::Float(FloatArray { id: Some(pos_array_id), val: positions.into() }), // TODO make a new func
        )],
        Vertices::new(vert_id.clone(), vec![Input::new(Semantic::Position, Url::Fragment(pos_id.clone()))]),
        vec![Primitive::Triangles(Triangles::new(
            None,
            vec![InputS::new(Semantic::Vertex, Url::Fragment(vert_id), 0, None)],
            tricount,
            indices.into_boxed_slice(),
        ))],
    ));

    let mut node = Node::new(format!("insig{}", id), Some(format!("insignia {}", id)));

    node.instance_geometry.push(instance);
    node.push_transform(Translate::new(insignia.offset.to_coord(up).into()));

    node
}

fn make_shield_node(shield: &ShieldData, geometries: &mut Vec<Geometry>, up: UpAxis) -> Node {
    let geo_id = format!("shield-geometry");
    let pos_id = format!("shield-geometry-position");
    let vert_id = format!("shield-geometry-vertex");
    let pos_array_id = format!("{}-array", pos_id);
    let norm_id = format!("shield-geometry-normal");
    let norm_array_id = format!("{}-array", norm_id);

    let mut positions = vec![];
    for vert in &shield.verts {
        positions.extend_from_slice(&<[_; 3]>::from(vert.to_coord(up)));
    }

    let mut normals = vec![];
    let mut tricount = 0;
    let mut indices = vec![];

    for poly in &shield.polygons {
        normals.extend_from_slice(&<[_; 3]>::from(poly.normal.to_coord(up)));

        indices.push(poly.verts.0 .0 as _);
        indices.push(tricount as _);

        // intentional swizzle
        indices.push(poly.verts.2 .0 as _);
        indices.push(tricount as _);

        indices.push(poly.verts.1 .0 as _);
        indices.push(tricount as _);

        tricount += 1;
    }

    let instance = Instance::<Geometry>::new(Url::Fragment(geo_id.clone()));

    geometries.push(Geometry::new_mesh(
        geo_id.clone(),
        vec![
            Source::new_local(
                pos_id.clone(),
                Param::new_xyz(),
                ArrayElement::Float(FloatArray { id: Some(pos_array_id), val: positions.into() }), // TODO make a new func
            ),
            Source::new_local(norm_id.clone(), Param::new_xyz(), ArrayElement::Float(FloatArray { id: Some(norm_array_id), val: normals.into() })),
        ],
        Vertices::new(vert_id.clone(), vec![Input::new(Semantic::Position, Url::Fragment(pos_id))]),
        vec![Primitive::Triangles(Triangles::new(
            None,
            vec![
                InputS::new(Semantic::Vertex, Url::Fragment(vert_id), 0, None),
                InputS::new(Semantic::Normal, Url::Fragment(norm_id), 1, None),
            ],
            tricount,
            indices.into_boxed_slice(),
        ))],
    ));

    let mut node = Node::new("shield", Some(String::from("shield")));

    node.instance_geometry.push(instance);

    node
}

fn make_subobj_node(
    up: UpAxis, subobjs: &ObjVec<SubObject>, subobj: &SubObject, turrets: &[Turret], geometries: &mut Vec<Geometry>, materials: &[String],
) -> Node {
    let geo_id = format!("{}-geometry", subobj.name);
    let pos_id = format!("{}-geometry-position", subobj.name);
    let vert_id = format!("{}-geometry-vertex", subobj.name);
    let pos_array_id = format!("{}-array", pos_id);
    let norm_id = format!("{}-geometry-normal", subobj.name);
    let norm_array_id = format!("{}-array", norm_id);
    let uv_id = format!("{}-geometry-uv", subobj.name);
    let uv_array_id = format!("{}-array", uv_id);

    let mut positions = vec![];
    for vert in &subobj.bsp_data.verts {
        positions.extend_from_slice(&<[_; 3]>::from(vert.to_coord(up)));
    }

    let mut normals = vec![];
    for norm in &subobj.bsp_data.norms {
        normals.extend_from_slice(&<[_; 3]>::from(norm.to_coord(up)));
    }

    let mut uv_coords = vec![];
    let mut uv_len = 0;
    let mut prim_elems = vec![(vec![], vec![]); materials.len() + 1];

    for (_, poly) in subobj.bsp_data.collision_tree.leaves() {
        let (vert_count, indices) = &mut prim_elems[poly.texture.0 as usize + 1];
        vert_count.push(poly.verts.len() as u32);
        for vert in poly.verts.iter().rev() {
            indices.push(vert.vertex_id.0 as _);
            indices.push(vert.normal_id.0 as _);
            indices.push(uv_len);
            uv_len += 1;
            uv_coords.push(vert.uv.0);
            uv_coords.push(1. - vert.uv.1);
        }
    }

    let mut instance = Instance::<Geometry>::new(Url::Fragment(geo_id.clone()));
    let bind_materials = prim_elems[1..]
        .iter()
        .zip(materials)
        .filter(|((vcount, _), _)| !vcount.is_empty())
        .map(|(_, mat)| InstanceMaterial::new(mat.to_string(), Url::Fragment(mat.clone()), vec![BindVertexInput::new("UVMap", "TEXCOORD", Some(0))]))
        .collect::<Vec<_>>();
    if !bind_materials.is_empty() {
        instance.data.bind_material = Some(BindMaterial::new(bind_materials));
    }

    geometries.push(Geometry::new_mesh(
        geo_id,
        vec![
            Source::new_local(
                pos_id.clone(),
                Param::new_xyz(),
                ArrayElement::Float(FloatArray { id: Some(pos_array_id), val: positions.into() }), // TODO make a new func
            ),
            Source::new_local(norm_id.clone(), Param::new_xyz(), ArrayElement::Float(FloatArray { id: Some(norm_array_id), val: normals.into() })),
            Source::new_local(uv_id.clone(), Param::new_st(), ArrayElement::Float(FloatArray { id: Some(uv_array_id), val: uv_coords.into() })),
        ],
        Vertices::new(vert_id.clone(), vec![Input::new(Semantic::Position, Url::Fragment(pos_id))]),
        prim_elems
            .into_iter()
            .zip([None].into_iter().chain(materials.iter().map(Some)))
            .filter(|((vcount, _), _)| !vcount.is_empty())
            .map(|((vcount, indices), material)| {
                // TODO maybe triangles sometimes?
                Primitive::PolyList(PolyList::new(
                    material.cloned(),
                    vec![
                        InputS::new(Semantic::Vertex, Url::Fragment(vert_id.clone()), 0, None),
                        InputS::new(Semantic::Normal, Url::Fragment(norm_id.clone()), 1, None),
                        InputS::new(Semantic::TexCoord, Url::Fragment(uv_id.clone()), 2, None),
                    ],
                    vcount.into_boxed_slice(),
                    indices.into_boxed_slice(),
                ))
            })
            .collect(),
    ));

    let mut node = Node::new(subobj.name.clone(), Some(subobj.name.clone()));
    node.push_transform(Translate::new(subobj.offset.to_coord(up).into()));

    // kind of expensive to do per subobj?
    for (i, turret) in turrets.iter().enumerate() {
        if turret.gun_obj == subobj.obj_id {
            for (j, point) in turret.fire_points.iter().enumerate() {
                let name = if turret.base_obj == subobj.obj_id {
                    format!("#t{}-point{}", i, j)
                } else {
                    format!("#t{}-gun-point{}", i, j)
                };

                let mut gunpoint_node = Node::new(name.clone(), Some(name));
                gunpoint_node.push_transform(Translate::new(point.to_coord(up).into()));
                gunpoint_node.push_transform(vec_to_rotation(&turret.normal.0, up));
                node.children.push(gunpoint_node);
            }
        }
    }

    if !subobj.properties.is_empty() {
        node.children.push(make_properties_node(&subobj.properties, format!("{}-", subobj.name)));
    }

    if subobj.movement_type != Default::default() {
        node.children
            .push(Node::new(format!("{}-mov-type", subobj.name), Some(format!("#{}-mov-type:{}", subobj.name, subobj.movement_type as i32))));
    }

    if subobj.movement_axis != Default::default() {
        node.children
            .push(Node::new(format!("{}-mov-axis", subobj.name), Some(format!("#{}-mov-axis:{}", subobj.name, subobj.movement_axis as i32))));
    }

    node.instance_geometry.push(instance);
    node.children.extend(
        subobj
            .children
            .iter()
            .map(|&id| make_subobj_node(up, subobjs, &subobjs[id], turrets, geometries, materials)),
    );

    node
}

impl Model {
    pub fn write_dae(&self, w: &mut impl Write) -> Result<(), dae_parser::Error> {
        let mut geometries = vec![];

        let mut nodes = vec![];
        let materials: Vec<String> = self.textures.iter().map(|tex| format!("{}-material", tex)).collect();

        let up = UpAxis::YUp;

        for subobj in &self.sub_objects {
            if subobj.parent.is_none() {
                let mut top_level_node = make_subobj_node(up, &self.sub_objects, subobj, &self.turrets, &mut geometries, &materials);

                for (i, insignia) in self.insignias.iter().enumerate() {
                    if self.get_detail_level(subobj.obj_id) == Some(insignia.detail_level) {
                        top_level_node.children.push(make_insignia_node(insignia, &mut geometries, i, up))
                    }
                }

                nodes.push(top_level_node);
            }
        }

        if self.shield_data.is_some() {
            nodes.push(make_shield_node((self.shield_data.as_ref()).unwrap(), &mut geometries, up));
        }

        if !self.thruster_banks.is_empty() {
            nodes.push(make_thrusters_node(&self.thruster_banks, up));
        }

        if !self.paths.is_empty() {
            nodes.push(make_paths_node(&self.paths, up));
        }

        if !self.primary_weps.is_empty() {
            nodes.push(make_weapons_node(&self.primary_weps, "primary", up));
        }

        if !self.secondary_weps.is_empty() {
            nodes.push(make_weapons_node(&self.secondary_weps, "secondary", up));
        }

        if !self.docking_bays.is_empty() {
            nodes.push(make_docking_bays_node(&self.docking_bays, up));
        }

        if !self.glow_banks.is_empty() {
            nodes.push(make_glows_node(&self.glow_banks, up));
        }

        if !self.special_points.is_empty() {
            nodes.push(make_specials_node(&self.special_points, up));
        }

        if !self.eye_points.is_empty() {
            nodes.push(make_eyes_node(&self.eye_points, up));
        }

        if !self.visual_center.is_null() {
            nodes.push(make_visual_center_node(&self.visual_center, up));
        }

        let mut doc = Document::create_now();
        doc.push_library(
            self.textures
                .iter()
                .map(|tex| Effect::shader(format!("{}-effect", tex), Lambert::default()))
                .collect(),
        );

        doc.push_library(
            self.textures
                .iter()
                .map(|tex| Material::new(format!("{}-material", tex), tex.clone(), Url::Fragment(format!("{}-effect", tex))))
                .collect(),
        );

        doc.push_library(geometries);

        let mut scene = VisualScene::new("Scene", None);
        scene.nodes = nodes;
        doc.push_library(vec![scene]);

        doc.scene = Some(Scene::new(Instance::new(Url::Fragment("Scene".to_string()))));

        doc.asset.up_axis = up;

        doc.write_to(w)
    }
}
