use stl_io::{IndexedMesh, Triangle, Vector};


pub struct IndexedMeshIterator<'a>
{
    mesh:&'a IndexedMesh,
    next_triangle:usize
}

impl <'a> ExactSizeIterator for IndexedMeshIterator<'a>
{
    fn len(&self) -> usize {
        self.mesh.faces.len()
    }
}

impl <'a> Iterator for IndexedMeshIterator<'a>
{
    type Item=Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        
        let next_face=match self.mesh.faces.get(self.next_triangle)
        {
            Some(next_face) => next_face,
            None => return None,
        };
        
        self.next_triangle+=1;

        let mut vertices:[Vector<f32>;3]=[
            stl_io::Vector([0.0,0.0,0.0]),
            stl_io::Vector([0.0,0.0,0.0]),
            stl_io::Vector([0.0,0.0,0.0]),
        ];
        
        for i in 0..3
        {
            let vertex=self.mesh.vertices.get(next_face.vertices[i]);
            match vertex
            {
                Some(vertex)=>vertices[i]=*vertex,
                None=>{
                    eprintln!("Invalid mesh.");
                    return None;
                }
            }
        }

        Some(
            Triangle { 
                normal: next_face.normal, 
                vertices
            }
        )
    }
}

impl <'a> IndexedMeshIterator<'a>
{
    pub fn new(mesh:&'a IndexedMesh)->IndexedMeshIterator<'a>
    {
        IndexedMeshIterator
        {
            mesh,
            next_triangle:0
        }
    }
}