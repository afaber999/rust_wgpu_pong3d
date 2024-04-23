use std::ops::Deref;
use std::path::Path;
use crate::texture2d::Texture2d;
use crate::geometries::{ColorElement, NormalElement, PositionElement, TexCoordElement};

pub struct Material {
    pub name: String,
    pub diffuse_texture: Texture2d,
    pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

// model.rs
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}


pub struct ModelGeometry {
    pub positions: Vec<PositionElement>,
    pub colors : Vec<ColorElement>,
    pub normals : Vec<NormalElement>,
    pub tex_coords : Vec<TexCoordElement>,
    pub indices : Vec<u32>,
}

// impl<'a> ModelGeometry<'a> {
//     pub fn new() -> Self {
//         Self {
//             positions : &CUBE_POSITIONS,
//             normals : &CUBE_NORMALS,
//             colors : &CUBE_COLORS,
//             indices : &CUBE_INDICES,
//             tex_coords : &CUBE_TEXCOORDS,
//         }
//     }
// }

impl Model {
    /// constructor, expects a filepath to a 3D model.
    pub fn new( path: &str) -> ModelGeometry { //-> Self {
        
        //let mut meshes = Vec::new();
        // let mut texture_pool = TexturePool::new(gl.clone());

        let path = Path::new(path);

        // retrieve the directory path of the filepath
        let directory : String = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        let mut load_options = tobj::LoadOptions::default();
        load_options.triangulate = true;
        load_options.single_index = true;

        let obj = tobj::load_obj(path, &load_options);

        //dbg!(obj);
        let (models, materials_result) = obj.unwrap();


        let mut geos = Vec::new();

        for model in models.into_iter(){
            let mesh = &model.mesh;
            
            let positions:Vec<PositionElement> = mesh.positions.
                chunks(3).
                map(  | f| {
                    PositionElement { position: [f[0],f[1],f[2]] }
                }).collect();

            let colors:Vec<ColorElement> = mesh.vertex_color.
                chunks(3).
                map(  | f| {
                    ColorElement { color: [f[0],f[1],f[2], 1.0] }
                }).collect();

            let normals:Vec<NormalElement> = mesh.normals.
                chunks(3).
                map(  | f| {
                    NormalElement { normal: [f[0],f[1],f[2]] }
                }).collect();

            let tex_coords:Vec<TexCoordElement> = mesh.texcoords.
                chunks(2).
                map(  | f| {
                    TexCoordElement { position: [f[0],f[1]] }
                }).collect();

            let indices:Vec<u32> = mesh.indices.clone();

            geos.push( ModelGeometry {
                positions,
                colors,
                normals,
                tex_coords,
                indices,
            } );
        }

        geos.pop().unwrap()
    }
}

// pub fn load_model(
//     file_name: &str,
//     device: &wgpu::Device,
//     queue: &wgpu::Queue,
//     layout: &wgpu::BindGroupLayout,
// ) -> anyhow::Result<model::Model> {
//     let obj_text = load_string(file_name).await?;
//     let obj_cursor = Cursor::new(obj_text);
//     let mut obj_reader = BufReader::new(obj_cursor);

//     let (models, obj_materials) = tobj::load_obj_buf_async(
//         &mut obj_reader,
//         &tobj::LoadOptions {
//             triangulate: true,
//             single_index: true,
//             ..Default::default()
//         },
//         |p| async move {
//             let mat_text = load_string(&p).await.unwrap();
//             tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
//         },
//     )
//     .await?;

//     let mut materials = Vec::new();
//     for m in obj_materials? {
//         let diffuse_texture = load_texture(&m.diffuse_texture, device, queue).await?;
//         let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//             layout,
//             entries: &[
//                 wgpu::BindGroupEntry {
//                     binding: 0,
//                     resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 1,
//                     resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
//                 },
//             ],
//             label: None,
//         });

//         materials.push(model::Material {
//             name: m.name,
//             diffuse_texture,
//             bind_group,
//         })
//     }

//     let meshes = models
//         .into_iter()
//         .map(|m| {
//             let vertices = (0..m.mesh.positions.len() / 3)
//                 .map(|i| model::ModelVertex {
//                     position: [
//                         m.mesh.positions[i * 3],
//                         m.mesh.positions[i * 3 + 1],
//                         m.mesh.positions[i * 3 + 2],
//                     ],
//                     tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
//                     normal: [
//                         m.mesh.normals[i * 3],
//                         m.mesh.normals[i * 3 + 1],
//                         m.mesh.normals[i * 3 + 2],
//                     ],
//                 })
//                 .collect::<Vec<_>>();

//             let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//                 label: Some(&format!("{:?} Vertex Buffer", file_name)),
//                 contents: bytemuck::cast_slice(&vertices),
//                 usage: wgpu::BufferUsages::VERTEX,
//             });
//             let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//                 label: Some(&format!("{:?} Index Buffer", file_name)),
//                 contents: bytemuck::cast_slice(&m.mesh.indices),
//                 usage: wgpu::BufferUsages::INDEX,
//             });

//             model::Mesh {
//                 name: file_name.to_string(),
//                 vertex_buffer,
//                 index_buffer,
//                 num_elements: m.mesh.indices.len() as u32,
//                 material: m.mesh.material_id.unwrap_or(0),
//             }
//         })
//         .collect::<Vec<_>>();

//     Ok(model::Model { meshes, materials })
// }

