use std::io::Cursor;
use glam::{Vec2, Vec3, Vec4};

pub enum Encode {
    UTF16,
    UTF8
}

pub enum IndexUnsigned {
    U8(u8),
    U16(u16),
    I32(i32),
}

pub enum Index {
    I8(i8),
    I16(i16),
    I32(i32),
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

pub struct PMXParser {
    pub cursor: Cursor<u8>,
    pub options: Option<PMXParserOptions>,
}

impl PMXParser {
    pub fn from_cursor(cur: Cursor<u8>) -> Self {
        Self {
            cursor: cur,
            options: None,
        }
    }

    pub fn with_options(self: &mut Self, options: PMXParserOptions) -> &mut Self {
        self.options = options.into();

        self
    }

    pub fn parse(self: &mut Self) -> PMXData {
        todo!()
    }
}
