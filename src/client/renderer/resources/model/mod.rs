use {
    crate::client::renderer::{
        gpu::buffer::{Buffer, self},
    },
    super::err::ResourceError,
    super::material::Material,
};

pub mod objfile;
pub mod err;

pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
    pub tan: [f32; 3],
    pub bitan: [f32; 3],
}

impl Vertex for ModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x3, 3 => Float32x3, 4 => Float32x3];

        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3D {
    pub position: [f32; 3],
}

impl Vertex for Vertex3D {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];

        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex3D>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material<f32>>,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: buffer::Buffer,
    pub index_buffer: buffer::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

pub fn load_model(
    file_name: &str,
    state: &State,
    path_m: &PathManager
) -> Result<Model, ResourceError> {
    let obj = objfile::ObjFile::from_file(file_name)?;

    let mut materials = Vec::new();
    let mut map: HashMap<String, usize> = std::collections::HashMap::with_capacity(obj.materials.len());
    for (ctr, (name, (dif, nor))) in obj.materials.iter().enumerate() {
        // let tex = DifNormTexture::from_files(
        //     device, queue,
        //     &format!("assets/textures/{dif}"),
        //     &format!("assets/textures/{nor}"),
        //     ""
        // )?;

        map.insert(name.clone(), ctr);

        let material = Material::from_paths(
            state,
            &[
                (dif, TextureEntry::DIFFUSE_MAP_ENTRY),
                (nor, TextureEntry::NORMAL_MAP_ENTRY)
            ],
            0.1,
            name,
            path_m
        )?;

        materials.push(material)
    }

    let mut meshes = Vec::new();
    for m in obj.faces {
        let vertices_org = obj.vertices.clone();
        let indices = m.1;

        let mut vertices = Vec::new();
        for vert in vertices_org {
            vertices.push(
                ModelVertex {
                    position: vert.position,
                    tex_coords: [vert.tex_coords[0], 1.0 - vert.tex_coords[1]],
                    normal: vert.normal,
                    tan: vert.tan,
                    bitan: vert.bitan,
                }
            );
        }

        let mut triangles_included = vec![0; vertices.len()];

        use crate::common::math::vec::{Vec3, Vec2};

        for c in indices.iter() {
            let v0 = vertices[c[0] as usize];
            let v1 = vertices[c[1] as usize];
            let v2 = vertices[c[2] as usize];


            let pos0: Vec3<_> = v0.position.into();
            let pos1: Vec3<_> = v1.position.into();
            let pos2: Vec3<_> = v2.position.into();

            let uv0: Vec2<_> = v0.tex_coords.into();
            let uv1: Vec2<_> = v1.tex_coords.into();
            let uv2: Vec2<_> = v2.tex_coords.into();

            // Calculate the edges of the triangle
            let delta_pos1 = pos1 - pos0;
            let delta_pos2 = pos2 - pos0;

            // This will give us a direction to calculate the
            // tangent and bitangent
            let delta_uv1 = uv1 - uv0;
            let delta_uv2 = uv2 - uv0;

            // Solving the following system of equations will
            // give us the tangent and bitangent.
            //     delta_pos1 = delta_uv1.x * T + delta_u.y * B
            //     delta_pos2 = delta_uv2.x * T + delta_uv2.y * B
            // Luckily, the place I found this equation provided
            // the solution!
            let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
            let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
            // We flip the bitangent to enable right-handed normal
            // maps with wgpu texture coordinate system
            let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * -r;

            // We'll use the same tangent/bitangent for each vertex in the triangle
            vertices[c[0] as usize].tan =
                (tangent + Vec3::from(vertices[c[0] as usize].tan)).into();
            vertices[c[1] as usize].tan =
                (tangent + Vec3::from(vertices[c[1] as usize].tan)).into();
            vertices[c[2] as usize].tan =
                (tangent + Vec3::from(vertices[c[2] as usize].tan)).into();
            vertices[c[0] as usize].bitan =
                (bitangent + Vec3::from(vertices[c[0] as usize].bitan)).into();
            vertices[c[1] as usize].bitan =
                (bitangent + Vec3::from(vertices[c[1] as usize].bitan)).into();
            vertices[c[2] as usize].bitan =
                (bitangent + Vec3::from(vertices[c[2] as usize].bitan)).into();

            // Used to average the tangents/bitangents
            triangles_included[c[0] as usize] += 1;
            triangles_included[c[1] as usize] += 1;
            triangles_included[c[2] as usize] += 1;
        }

        // Average the tangents/bitangents
        for (i, n) in triangles_included.into_iter().enumerate() {
            let denom = 1.0 / n as f32;
            let mut v = &mut vertices[i];
            v.tan = (Vec3::from(v.tan) * denom).into();
            v.bitan = (Vec3::from(v.bitan) * denom).into();
        }

        let vertex_buffer = Buffer::new_vertex(&state.device, &vertices, &format!("{file_name} Vertex Buffer"));
        let index_buffer = Buffer::new_index(&state.device, &indices, &format!("{file_name} Index Buffer"));

        meshes.push(
            Mesh {
                name: file_name.to_string(),
                vertex_buffer,
                index_buffer,
                num_elements: indices.len() as u32 * 3,
                material: *map.get(&m.0).unwrap(),
            }
        )
    }

    Ok(Model { meshes, materials })
}

use core::ops::Range;
use std::collections::HashMap;

use crate::client::{renderer::state::State, PathManager};

use super::image::texture::TextureEntry;
pub trait DrawModel<'a> {
    fn draw_mesh<T: bytemuck::Pod>(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material<T>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_mesh_instanced<T: bytemuck::Pod>(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material<T>,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );

    fn draw_model(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh<T: bytemuck::Pod>(&mut self, mesh: &'b Mesh, material: &'b Material<T>, camera_bind_group: &'b wgpu::BindGroup, light_bind_group: &'a wgpu::BindGroup) {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group, light_bind_group);
    }

    fn draw_mesh_instanced<T: bytemuck::Pod>(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material<T>,
        instances: core::ops::Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bg.group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.set_bind_group(2, light_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_model(&mut self, model: &'b Model, camera_bind_group: &'b wgpu::BindGroup, light_bind_group: &'a wgpu::BindGroup) {
        self.draw_model_instanced(model, 0..1, camera_bind_group, light_bind_group);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'b Model,
        instances: core::ops::Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group, light_bind_group);
        }
    }
}






pub trait DrawLight<'a> {
    fn draw_light_mesh(
        &mut self,
        mesh: &'a Mesh,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );

    fn draw_light_model(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_light_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawLight<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_light_mesh(
        &mut self,
        mesh: &'b Mesh,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
    ) {
        self.draw_light_mesh_instanced(mesh, 0..1, camera_bind_group, light_bind_group);
    }

    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, camera_bind_group, &[]);
        self.set_bind_group(1, light_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_light_model(
        &mut self,
        model: &'b Model,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
    ) {
        self.draw_light_model_instanced(model, 0..1, camera_bind_group, light_bind_group);
    }
    fn draw_light_model_instanced(
        &mut self,
        model: &'b Model,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            self.draw_light_mesh_instanced(mesh, instances.clone(), camera_bind_group, light_bind_group);
        }
    }
}
