struct VsOutput {
    @builtin(position) position : vec4f
}

@vertex
fn unlit_material_vs( @builtin(vertex_index) idx:u32 ) -> VsOutput {

    var position : array<vec4f, 3 > = array (
        vec4f(-0.5,-0.5, 0.0, 1.0),
        vec4f(-0.5, 0.5, 0.0, 1.0),
        vec4f( 0.5,-0.5, 0.0, 1.0),
    );

    var out : VsOutput;
    out.position = position[idx];
    return out;
}


@fragment
fn unlit_material_fs( ) -> @location(0) vec4f {
    return vec4f(0.0, 1.0, 0.0, 1.0 );
}

