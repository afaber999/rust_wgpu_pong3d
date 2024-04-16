struct VsInput {
    @location(0) position : vec3f,
    @location(1) color : vec4f,
    @location(2) texcoord : vec2f,
}

struct VsOutput {
    @builtin(position) position : vec4f,
    @location(1) color : vec4f,
    @location(2) texcoord : vec2f,
}

@vertex
fn unlit_material_vs( in : VsInput ) -> VsOutput {
    var out : VsOutput;
    out.position = vec4f(in.position, 1.0);
    out.color = in.color;
    out.texcoord = in.texcoord;
    return out;
}


@group(0) @binding(0)
var t_diffuse : texture_2d<f32>;

@group(0) @binding(1)
var s_diffuse : sampler;

@fragment
fn unlit_material_fs( in: VsOutput ) -> @location(0) vec4f {
    //return vec4f(in.texcoord.x,in.texcoord.y,0.0,1.0);
    return textureSample(t_diffuse, s_diffuse, in.texcoord);
//    return textureSample(t_diffuse, s_diffuse, in.texcoord);
}
