use std::{collections::BTreeMap, ops::Index};

use stl_io::{IndexedMesh, IndexedTriangle, Normal, Vector, Vertex};

use crate::commons::{normal_calculation::calculate_normal_from_indices, polygon::create_clockwise_polygon};

pub fn create_cable_retainer(
    wall_thickness:f32,
    cable_diameter:f32,
    retainer_tunnel_length:f32,
    side_transition_length:f32
)-> Result<IndexedMesh,String> {

    const STEPS:usize=100;

    let total_height=cable_diameter+wall_thickness; //Just cover at top
    let floor=-cable_diameter/2.0;
    //let ceiling=total_height-floor;
    let total_width=total_height*2.0;

    let mut vertices:Vec<Vertex>=Vec::new();
    let mut faces:Vec<IndexedTriangle>=Vec::new();

    let calculate_top_index = |step_index:usize,x_index:usize|->usize
    {
        x_index+step_index*2
    };

    let get_top_z_frac = |step_frac:f32|
    {
        -(f32::cos(step_frac*std::f32::consts::PI*2.0)-1.0)/2.0
    };

    let get_top_y_frac = |step_frac:f32|
    {
        step_frac+f32::sin(step_frac*std::f32::consts::PI*4.0)/(8.0*std::f32::consts::PI)
    };

    assert_eq!(get_top_z_frac(0.0),0.0);
    assert_eq!(get_top_z_frac(0.5),1.0);
    assert_eq!(get_top_z_frac(1.0),0.0);
    assert_eq!(get_top_y_frac(0.0),0.0);
    assert_eq!(get_top_y_frac(0.5),0.5);
    assert_eq!(get_top_y_frac(1.0),1.0);

    //Top
    {
        for step_index in 0..STEPS
        {
            let step_frac=(step_index as f32)/((STEPS-1) as f32);

            let z:f32=get_top_z_frac(step_frac)*total_height+floor;
            let y:f32=get_top_y_frac(step_frac)*total_width;

            for x_index in 0..=1
            {
                let x=match x_index
                {
                    0=>-retainer_tunnel_length/2.0,
                    1=>retainer_tunnel_length/2.0,
                    _=>panic!("Unexpected value")
                };

                let index=calculate_top_index(step_index,x_index);


                assert_eq!(index,vertices.len());
                
                vertices.push(Vector::new([x,y,z]));
            }
        }

        for step_index in 0..STEPS-1
        {
            let indices=Vec::from(
                [
                    calculate_top_index(step_index,0),
                    calculate_top_index(step_index,1),
                    calculate_top_index(step_index+1,1),
                    calculate_top_index(step_index+1,0),                
                ]
            );

            for face in create_clockwise_polygon(indices, &vertices)
            {
                faces.push(face);
            }
        }
    }

    /*
    return Ok(IndexedMesh {
        vertices,
        faces
    });
    */

    const TOP_VERTEX_COUNT:usize=STEPS*2;
    assert_eq!(TOP_VERTEX_COUNT,vertices.len());

    const SIDE_VERTEX_STEPS:usize=25;

    let calculate_side_index = |x_index:usize,step_index:usize,sub_step_index:usize |
    {
        if sub_step_index == 0 {
            calculate_top_index(step_index,x_index)
        }
        else {
            TOP_VERTEX_COUNT+x_index*STEPS*(SIDE_VERTEX_STEPS-1)+step_index*(SIDE_VERTEX_STEPS-1)+(sub_step_index-1)
        }
    };
    //Sides
    {
        for x_index in 0..=1
        {
            let x_dir = match x_index
            {
                0=>-1f32,
                1=>1f32,
                _=>panic!("Shouldn't happen")
            };

            for step_index in 0..STEPS
            {
                let top_index=calculate_top_index(step_index,x_index);
                let top_vertex=vertices.get(top_index).expect("Should exist.").clone();


                for sub_step_index in 1..SIDE_VERTEX_STEPS //Start at 1 and treat 0 as the top point
                {
                    let side_index=calculate_side_index(x_index,step_index,sub_step_index);
                    let side_frac = (sub_step_index as f32)/((SIDE_VERTEX_STEPS-1) as f32);
                    
                    //println!("{},{},{}",x_index,step_index,sub_step_index);
                    assert_eq!(side_index,vertices.len());

                    let x= top_vertex[0]+side_frac*x_dir*side_transition_length;
                    let z = (top_vertex[2]-floor)*(f32::cos(side_frac*std::f32::consts::PI)+1.0)/2.0+floor;

                    vertices.push(Vector::new(
                        [
                            x,
                            top_vertex[1],
                            z
                        ]
                    ));
                }
            }
        }

        for x_index in 0..=1
        {
            for step_index in 0..STEPS-1
            {
                for sub_step_index in 0..SIDE_VERTEX_STEPS-1
                {
                    let mut indices=Vec::from(
                        [
                            calculate_side_index(x_index,step_index,sub_step_index),
                            calculate_side_index(x_index,step_index+1,sub_step_index),
                            calculate_side_index(x_index,step_index+1,sub_step_index+1),
                            calculate_side_index(x_index,step_index,sub_step_index+1),                
                        ]
                    );

                    if x_index==1
                    {
                        indices.reverse();
                    }

                    println!("{},{:?}",vertices.len(),indices);

                    for face in create_clockwise_polygon(indices, &vertices)
                    {
                        faces.push(face);
                    }
                }
            }
        }
    }

    //Bottom
    {
        const SUB_STEP_INDEX:usize = SIDE_VERTEX_STEPS-1;
        let indices = Vec::from(
            [
                calculate_side_index(0,0,SUB_STEP_INDEX),
                calculate_side_index(0,STEPS-1,SUB_STEP_INDEX),
                calculate_side_index(1,STEPS-1,SUB_STEP_INDEX),
                calculate_side_index(1,0,SUB_STEP_INDEX),
            ]
        );
        
        for face in create_clockwise_polygon(indices, &vertices)
        {
            faces.push(face);
        }

        println!("{:?}",faces);
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