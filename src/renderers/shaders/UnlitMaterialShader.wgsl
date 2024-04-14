struct VsInput {
    @location(0) position : vec3f,
    @location(1) color : vec4f,
}

struct VsOutput {
    @builtin(position) position : vec4f,
    @location(1) color : vec4f,
}

@vertex
fn unlit_material_vs( in : VsInput ) -> VsOutput {
    var out : VsOutput;
    out.position = vec4f(in.position, 1.0);
    out.color = in.color;
    return out;
}


@fragment
fn unlit_material_fs( in: VsOutput ) -> @location(0) vec4f {
    return in.color;
}

