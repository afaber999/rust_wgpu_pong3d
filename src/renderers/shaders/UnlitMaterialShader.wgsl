struct VsInput {
    @location(0) position : vec3f,
}

struct VsOutput {
    @builtin(position) position : vec4f
}

@vertex
fn unlit_material_vs( in : VsInput ) -> VsOutput {
    var out : VsOutput;
    out.position = vec4f(in.position, 1.0);
    return out;
}


@fragment
fn unlit_material_fs( ) -> @location(0) vec4f {
    return vec4f(0.0, 1.0, 0.0, 1.0 );
}

