use std::fs::OpenOptions;
use stl_io::{Normal, Vertex};

use stl_generator::objects::{cable_retainer::create_top, IndexedMeshIterator};

fn main()->Result<(),Box<dyn std::error::Error>> {
    
    let mesh = create_top()?;
    let meshiterator = IndexedMeshIterator::new(&mesh);

    let path = "output/mesh.stl";
    if std::fs::exists(path)? {
        std::fs::remove_file(path)?;
    }
    let mut file = OpenOptions::new().write(true).create_new(true).open(path).unwrap();
    
    stl_io::write_stl(&mut file, meshiterator).unwrap();

    Ok(())
}