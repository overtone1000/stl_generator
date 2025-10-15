use std::fs::OpenOptions;

use stl_generator::{commons::indexed_mesh_iterator::IndexedMeshIterator, objects::cable_retainer::create_cable_retainer};

fn main()->Result<(),Box<dyn std::error::Error>> {
    
    let mesh = create_cable_retainer(2.0,10.0,40.0,10.0)?;
    let meshiterator = IndexedMeshIterator::new(&mesh);

    let path = "output/mesh.stl";
    if std::fs::exists(path)? {
        std::fs::remove_file(path)?;
    }
    let mut file = OpenOptions::new().write(true).create_new(true).open(path).unwrap();
    
    stl_io::write_stl(&mut file, meshiterator).unwrap();

    Ok(())
}