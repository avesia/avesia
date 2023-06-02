use std::convert::TryFrom;
use glam::{Vec2, Vec3, Vec4};
use nom::{
    bytes::complete::tag,
    sequence::tuple,
    combinator::map,
    IResult, number::complete::{be_f32, be_u8}
};

pub enum Encode {
    UTF16 = 0,
    UTF8 = 1
}

// Covert u8 to enum
impl TryFrom<u8> for Encode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match (value) {
            v if v == Self::UTF16 as u8 => Ok(Self::UTF16),
            v if v == Self::UTF8 as u8 => Ok(Self::UTF8),
            _ => Err(()),
        }
    }
}

#[repr(u8)]
pub enum IndexUnsigned {
    U8(u8) = 1,
    U16(u16) = 2,
    I32(i32) = 4,
}

// Covert u8 to enum
impl TryFrom<u8> for IndexUnsigned {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match (value) {
            v if v == Self::U8 as u8 => Ok(Self::U8(0)),
            v if v == Self::U16 as u8 => Ok(Self::U16(0)),
            v if v == Self::I32 as u8 => Ok(Self::I32(0)),
            _ => Err(()),
        }
    }
}

#[repr(u8)]
pub enum Index {
    I8(i8) = 1,
    I16(i16) = 2,
    I32(i32) = 4,
}

// Covert u8 to enum
impl TryFrom<u8> for Index {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match (value) {
            v if v == Self::I8 as u8 => Ok(Self::I8(0)),
            v if v == Self::I16 as u8 => Ok(Self::I16(0)),
            v if v == Self::I32 as u8 => Ok(Self::I32(0)),
            _ => Err(()),
        }
    }
}

pub struct PMXHeader {
    pub version: f32,
 
    pub encoding: Encode,
    pub additional_uv_size: u8,
    pub vertex_index_size: IndexUnsigned,
    pub texture_index_size: Index,
    pub material_index_size: Index,
    pub bone_index_size: Index,
    pub morph_index_size: Index,
    pub rigid_body_index_size: Index,
}

pub struct PMXModelInfo {
    pub model_name: String,
    pub model_name_english: String,
    pub model_comments: String,
    pub model_comments_english: String,
}

pub enum VertexDeform {
    BDEF1 {
        bone_index: Index,
    },
    BDEF2 {
        bone_index_1: Index,
        bone_index_2: Index,
        bone_weight_1: f32,
    },
    BDEF4 {
        bone_index_1: Index,
        bone_index_2: Index,
        bone_index_3: Index,
        bone_index_4: Index,
        bone_weight_1: f32,
        bone_weight_2: f32,
        bone_weight_3: f32,
        bone_weight_4: f32,
    },
    SDEF {
        bone_index_1: Index,
        bone_index_2: Index,
        bone_weight_1: f32,
        sdef_c: Vec3,
        sdef_r0: Vec3,
        sdef_r1: Vec3,
    }
}

pub struct PMXVertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub additional_uvs: Vec<Vec4>,
    pub weight: VertexDeform,
    pub edge_scale: f32,
}

pub struct PMXSurface {
    pub vertex_index: Index,
}

pub struct PMXTexture {
    pub path: String,
}

pub struct PMXMaterial {
    pub name: String,
    pub name_english: String,
}

pub struct PMXData {
    pub header: PMXHeader,
    pub model_info: PMXModelInfo,
    pub vertices: Vec<PMXVertex>,
    pub surfaces: Vec<PMXSurface>,
    pub textures: Vec<PMXTexture>,
    pub materials: Vec<PMXMaterial>,
} 

pub struct PMXParserOptions {
    
}

pub struct PMXParser<'a> {
    pub bytes: &'a [u8],
    pub options: Option<PMXParserOptions>,
}

impl PMXParser<'_> {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {
            bytes: buf,
            options: None,
        }
    }

    pub fn with_options(self: &mut Self, options: PMXParserOptions) -> &mut Self {
        self.options = options.into();

        self
    }

    pub fn parse(self: &mut Self) -> PMXData {
        let header = parse_header(self.bytes).unwrap();

        PMXData { header: header.1, }
    }
}

pub fn parse_header<'a>(buf: &[u8]) -> IResult<&'a[u8], PMXHeader> {
    map(
        tuple((
            tag("PMX "),
            be_f32, // version
            be_u8,
            be_u8, // encoding
            be_u8, // additional uv size
            be_u8, // vertex index size
            be_u8, // texture index size
            be_u8, // material index size
            be_u8, // bone index size
            be_u8, // morph index size
            be_u8, // rigid body index size
        )),
        |(
            _,
            version,
            _,
            encoding,
            additional_uv_size,
            vertex_index_size,
            texture_index_size,
            material_index_size,
            bone_index_size,
            morph_index_size,
            rigid_body_index_size
        )| {
            PMXHeader {
                version:                version, 
                encoding:               encoding.try_into().unwrap(),
                additional_uv_size:     additional_uv_size,
                vertex_index_size:      vertex_index_size.try_into().unwrap(),
                texture_index_size:     texture_index_size.try_into().unwrap(),
                material_index_size:    material_index_size.try_into().unwrap(),
                bone_index_size:        bone_index_size.try_into().unwrap(),
                morph_index_size:       morph_index_size.try_into().unwrap(),
                rigid_body_index_size:  rigid_body_index_size.try_into().unwrap(),
            }
        }
    )(buf)
}
