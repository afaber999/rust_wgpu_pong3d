struct VsInput {
    @location(0) position : vec3f,
    @location(1) color : vec4f,
    @location(2) texcoord : vec2f,
    @location(3) normal : vec3f,
}

struct VsOutput {
    @builtin(position) position : vec4f,
    @location(1) color : vec4f,
    @location(2) texcoord : vec2f,
    @location(3) normal : vec3f,
}

struct Material {
    @location(0) ambient_color : vec3f,
    @location(1) diffuse_color : vec3f,
    @location(2) specular_color : vec3f,
}

@group(0) @binding(0)
var<uniform> model_matrix : mat4x4f;

@group(1) @binding(0)
var<uniform> view_projection : mat4x4f;

@group(2) @binding(0)
var t_diffuse : texture_2d<f32>;

@group(2) @binding(1)
var s_diffuse : sampler;

@group(3) @binding(0)
var<uniform> material : Material;


@vertex
fn material_vs( in : VsInput ) -> VsOutput {
    var out : VsOutput;
    // COLUMN MAJOR LH PROJECTION * VIEW * MODEL
    out.position =   view_projection * model_matrix * vec4f(in.position, 1.0);
    //out.position =   model_matrix * vec4f(in.position, 1.0);
    out.color = in.color;
    out.texcoord = in.texcoord;
    out.normal = in.normal;
    return out;
}

@fragment
fn material_fs( in: VsOutput ) -> @location(0) vec4f {
    //return vec4f(in.texcoord.x,in.texcoord.y,0.0,1.0);
    return textureSample(t_diffuse, s_diffuse, in.texcoord) * vec4f(abs( in.normal ), 1.0);
    //return vec4f( material.diffuse_color, 1.0);
//    return textureSample(t_diffuse, s_diffuse, in.texcoord);
}
