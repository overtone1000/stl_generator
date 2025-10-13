use std::collections::BTreeMap;

use stl_io::{IndexedMesh, IndexedTriangle, Normal, Vertex};

pub fn create_top()-> Result<IndexedMesh,String> {

    const STEPS:usize=100;

    let mut vertices:Vec<Vertex>=Vec::new();
    let mut faces:Vec<IndexedTriangle>=Vec::new();

    let calculate_index = |x:usize,y:usize|->usize
    {
        x+y*STEPS
    };

    let get_top_vertex=|x:usize,y:usize|->Vertex
    {
        let xf=(x as f32)/((STEPS-1) as f32);
        let yf=(y as f32)/((STEPS-1) as f32);

        let z = f32::max(0.0,xf-0.5)*f32::max(0.0,yf-0.5);

        Vertex::new([xf,yf,z])
    };

    for y in 0..STEPS
    {
        for x in 0..STEPS
        {
            let index=calculate_index(x,y);

            if index!=vertices.len()
            {
                return Err(format!("Vertex index determination is incorrect. Expected index {} but found {}",index,vertices.len()));
            }

            vertices.push(get_top_vertex(x,y));
        }
    }

    for y in 0..STEPS-1
    {
        for x in 0..STEPS-1
        {
            let x0y0 = calculate_index(x,y);
            let x1y0 = calculate_index(x+1,y);
            let x0y1 = calculate_index(x,y+1);
            let x1y1 = calculate_index(x+1,y+1);

            faces.push(IndexedTriangle{
                normal:Normal::new([0.0,0.0,0.1]),
                vertices:[x0y0,x1y0,x0y1]
            });

            faces.push(IndexedTriangle{
                normal:Normal::new([0.0,0.0,0.1]),
                vertices:[x1y1,x1y0,x0y1]
            });
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