use stl_io::{Normal, Vertex};
use vec3_rs::Vector3;

pub fn calculate_normal_from_indices (indices:[usize;3], vertices:&Vec<Vertex>) -> Normal
{
    let triangle_vertices = indices.map(
        |i|{
            vertices.get(i).expect("Invalid vertex.")
        }
    );

    const FIRST_VERTEX:usize=0;
    const SECOND_VERTEX:usize=1;
    const THIRD_VERTEX:usize=2;

    let v1:Vector3<f32>=Vector3::new(
        triangle_vertices[SECOND_VERTEX][0]-triangle_vertices[FIRST_VERTEX][0],
        triangle_vertices[SECOND_VERTEX][1]-triangle_vertices[FIRST_VERTEX][1],
        triangle_vertices[SECOND_VERTEX][2]-triangle_vertices[FIRST_VERTEX][2],
    );

    let v2:Vector3<f32>=Vector3::new(
        triangle_vertices[THIRD_VERTEX][0]-triangle_vertices[SECOND_VERTEX][0],
        triangle_vertices[THIRD_VERTEX][1]-triangle_vertices[SECOND_VERTEX][1],
        triangle_vertices[THIRD_VERTEX][2]-triangle_vertices[SECOND_VERTEX][2],
    );

    let normal = v1.cross(&v2);

    Normal::new(
        [
            normal.get_x(),
            normal.get_y(),
            normal.get_z()
        ]
    )
}