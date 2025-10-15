use stl_io::{IndexedTriangle, Vertex};

use super::normal_calculation::calculate_normal_from_indices;

pub fn create_clockwise_polygon(indices:Vec<usize>, vertices:&Vec<Vertex>)->Vec<IndexedTriangle>
{
    let mut retval=Vec::new();
    for i in 1..indices.len()-1
    {
        let indices=
        [
            indices.get(0).expect("Should exist.").clone(),
            indices.get(i).expect("Should exist.").clone(),
            indices.get(i+1).expect("Should exist.").clone()
        ];

        retval.push(
            IndexedTriangle {
                 normal: calculate_normal_from_indices(indices,&vertices), 
                 vertices: indices
            }
        );
    }

    retval
}