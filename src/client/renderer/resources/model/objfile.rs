use {
    super::ModelVertex,
    std::{
        collections::HashMap,
        path::Path,
    },
};

pub use err::ObjFileError;

#[derive(Debug)]
pub struct ObjFile {
    material_path: String,
    positions: Vec<[f32; 3]>,
    tex: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
    pub faces: Vec<(String, Vec<[u32; 3]>)>,
    pub vertices: Vec<ModelVertex>,
    pub materials: HashMap<String, (String, String)>
}

impl ObjFile {
    pub fn from_file(path: &str) -> Result<Self, ObjFileError> {
        let path = Path::new(path);
        let src = crate::files::read_file(path)?.0;

        let mut objfile = ObjFile {
            material_path: String::new(),
            positions: Vec::new(),
            tex: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
            vertices: Vec::new(),
            materials: HashMap::new(),
        };

        let lines = src.lines();

        let mut linectr: usize = 0;

        let mut face_vecs: usize = 0;
        let mut current_vert: u32 = 0;

        let mut map: HashMap<(u32, u32, u32), u32> = HashMap::new();


        for line in lines {
            linectr += 1;
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
            if tokens.is_empty() {continue}

            match tokens[0] {
                "mtllib" => {
                    Self::check_tokens(tokens.len(), 1, linectr, path)?;
                    objfile.material_path = tokens[1].to_owned();
                }
                "v" => {
                    Self::check_tokens(tokens.len(), 3, linectr, path)?;
                    let verts = tokens[1..].iter().map(|v| v.parse::<f32>()
                        .map_err(|e| ObjFileError::Parsing(linectr, path.to_owned(), e.to_string())))
                        .collect::<Result<Vec<_>,_>>()?;
                    objfile.positions.push([ verts[0], verts[1], verts[2] ]);
                },
                "vt" => {
                    Self::check_tokens(tokens.len(), 2, linectr, path)?;
                    let tex = tokens[1..].iter().map(|v| v.parse::<f32>()
                        .map_err(|e| ObjFileError::Parsing(linectr, path.to_owned(), e.to_string())))
                        .collect::<Result<Vec<_>,_>>()?;
                    objfile.tex.push([ tex[0], tex[1] ]);
                },
                "vn" => {
                    Self::check_tokens(tokens.len(), 3, linectr, path)?;
                    let normals = tokens[1..].iter().map(|v| v.parse::<f32>()
                        .map_err(|e| ObjFileError::Parsing(linectr, path.to_owned(), e.to_string())))
                        .collect::<Result<Vec<_>,_>>()?;
                    objfile.normals.push([ normals[0], normals[1], normals[2] ]);
                },
                "usemtl" => {
                    Self::check_tokens(tokens.len(), 1, linectr, path)?;
                    objfile.faces.push((tokens[1].to_string(), Vec::new()));
                    face_vecs += 1;
                }
                "f" => {
                    Self::check_tokens(tokens.len(), 3, linectr, path)?;
                    let indvec = tokens[1..].iter().map(|v|
                        v.split('/')
                            .map(|i| i.parse::<u32>()
                            .map_err(|e| ObjFileError::Parsing(linectr, path.to_owned(), e.to_string())))
                            .collect::<Result<Vec<u32>, _>>())
                        .collect::<Result<Vec<_>,_>>()?;
                    let mut face: Vec<u32> = Vec::new();
                    for indices in indvec {
                        if indices.len() != 3 { return Err(ObjFileError::Mismatch(linectr, path.to_owned(), 3, indices.len())) }

                        face.push(*map.entry((indices[0], indices[1], indices[2])).or_insert_with(|| -> u32 {
                            objfile.vertices.push(ModelVertex {
                                position: objfile.positions[(indices[0] - 1) as usize],
                                tex_coords: objfile.tex[(indices[1] - 1) as usize],
                                normal: objfile.normals[(indices[2] - 1) as usize],
                                tan: [0.; 3],
                                bitan: [0.; 3],
                            });
                            current_vert += 1;
                            current_vert - 1
                        }));
                    }
                    if objfile.faces.is_empty() {return Err(ObjFileError::MissingMaterial(linectr, Path::new(path).to_owned()))}
                    objfile.faces[face_vecs - 1].1.push([face[0], face[1], face[2]]);
                }
                _ => continue,
            };
        }

        let mtlpath = Path::new(path).parent().ok_or(ObjFileError::MissingParent(Path::new(path).to_owned()))?.join(Path::new(&objfile.material_path));
        let mtlsrc = crate::files::read_file(&mtlpath)?.0;
        let mtllines = mtlsrc.lines();

        linectr = 0;
        let mut current_mat = String::new();
        for line in mtllines {
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
            linectr += 1;

            if tokens.is_empty() {continue}
            use ObjFileError::Mtl;
            match tokens[0] {
                "newmtl" => {
                    if tokens.len() < 2 { return Err(Mtl(linectr, mtlpath, "\n        Material expected")) }
                    objfile.materials.insert(tokens[1].to_owned(), (String::new(), String::new()));
                    current_mat = tokens[1].to_owned();
                }
                "map_Kd" => {
                    if tokens.len() < 2 { return Err(Mtl(linectr, mtlpath, "\n        Diffuse map expected")) }
                    objfile.materials.get_mut(&current_mat).ok_or(Mtl(linectr, mtlpath.clone(), "\n        Texture defined before the first material"))?.0 = tokens[1].to_owned();
                }
                "map_Bump" => {
                    if tokens.len() < 2 { return Err(Mtl(linectr, mtlpath, "\n        Normal map expected")) }
                    objfile.materials.get_mut(&current_mat).ok_or(Mtl(linectr, mtlpath.clone(), "\n        Texture defined before the first material"))?.1 = tokens[1].to_owned();
                }
                _ => continue,
            }
        }

        Ok(objfile)
    }

    fn check_tokens(len: usize, c: usize, linectr: usize, path: &Path) -> Result<(), ObjFileError> {
        if len != (c + 1) {Err(ObjFileError::Mismatch(linectr, path.to_owned(), c, len - 1))}
        else {Ok(())}
    }
}

pub mod err {
    use {
        crate::files::FileError,
        std::{
            path::PathBuf,
        },
    };

    pub enum ObjFileError {
        File(FileError),
        Mtl(usize, PathBuf, &'static str),
        Parsing(usize, PathBuf, String),
        Mismatch(usize, PathBuf, usize, usize),
        MissingMaterial(usize, PathBuf),
        MissingParent(PathBuf),
    }

    use crate::err::macros::*;

    impl_error!(ObjFileError,
        File(e) => "With file: {}", e;
        Mtl(line, path, msg) => "At line {} of {}: {}", line, path.display(), msg;
        Parsing(line, path, msg) => "At line {} of {}: Failed parsing: {}", line, path.display(), msg;
        Mismatch(line, path, expected, received) => "At line {} of {}:\n        Expected {} values; received {}", line, path.display(), expected, received;
        MissingMaterial(line, path) => "At {} of {}: Material missing", line, path.display();
        MissingParent(path) => "Couldn't find parent of {}", path.display()
    );

    impl_error_conversion!(ObjFileError, FileError => File);
}

