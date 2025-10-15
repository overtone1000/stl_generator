use std::{collections::BTreeMap, ops::Index};

use stl_io::{IndexedMesh, IndexedTriangle, Normal, Vector, Vertex};

use crate::commons::{normal_calculation::calculate_normal_from_indices, polygon::create_clockwise_polygon};

pub fn create_top()-> Result<IndexedMesh,String> {

    const STEPS:usize=100;

    let mut vertices:Vec<Vertex>=Vec::new();
    let mut faces:Vec<IndexedTriangle>=Vec::new();

    let calculate_index = |step_index:usize,x_index:usize|->usize
    {
        x_index+step_index*2
    };

    for step_index in 0..STEPS
    {
        let step_frac=(step_index as f32)/((STEPS-1) as f32);

        let z:f32=-f32::cos(step_frac*std::f32::consts::PI*2.0)/2.0+1.0;
        let y:f32=step_frac+f32::sin(step_frac*std::f32::consts::PI*4.0)/(8.0*std::f32::consts::PI);

        println!("{}:{}",y,z);

        for x_index in 0..=1
        {
            let x=match x_index
            {
                0=>0f32,
                1=>1f32,
                _=>panic!("Unexpected value")
            };

            let index=calculate_index(step_index,x_index);

            if index!=vertices.len()
            {
                panic!("Unexpected index {}, expected {}", vertices.len(),index);
            }

            vertices.push(Vector::new([x,y,z]));
        }
    }

    for step_index in 0..STEPS-1
    {
        let indices=Vec::from(
            [
                calculate_index(step_index,0),
                calculate_index(step_index,1),
                calculate_index(step_index+1,1),
                calculate_index(step_index+1,0),                
            ]
        );

        for face in create_clockwise_polygon(indices, &vertices)
        {
            faces.push(face);
        }
    }

    let retval = IndexedMesh {
        vertices,
        faces
    };

    
    /*
    match retval.validate()
    {
        Ok(_)=>Ok(retval),
        Err(e)=>Err(format!("{:?}",e))
    }
    */

    Ok(retval)
}