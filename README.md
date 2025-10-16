# stl_generator
Programmatic generation of STLs

## To Do
-[ ] Manifold error on cable retainer had to be fixed in an external program (see cable_retainer_bug.png). Best guess is that the bottom needs a polygon for EACH vertex at the bottom of the main part.
    - With 11 steps and 6 substeps, there are 24 naked edges. Seems like 4 for each substep.
    - With 4 steps and 3 substeps, there are 12 naked edges. Seems like 4 for each substep!
    - With 3 steps and 2 substeps, there are 8 naked edges. 4 per substep
    - There are always 2 planar holes. Looks like the corner triangles that are parallel are the problem. Probably need to extend out the bottom a bit somehow.