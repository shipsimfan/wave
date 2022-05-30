cbuffer CameraBuffer {
    matrix projection;
}

cbuffer ObjectBuffer {
    matrix object;
    float4 tint;
}

struct VertexInputType {
    float4 position: POSITION;
    float4 color: COLOR;
    float2 uv: TEXCOORD;
};

struct PixelInputType {
    float4 position: SV_POSITION;
    float4 color: COLOR;
    float2 uv: TEXCOORD;
};

Texture2D<float> wave : register(t0);
SamplerState sampler_type;

float4 f_to_color(float f) {
    float value = saturate(f + 0.5);
    return float4(value, 0, 1.0 - value, 1.0);
}

PixelInputType vertex_main(VertexInputType input) {
    PixelInputType output;

    float y = wave.GatherRed(sampler_type, input.uv);

    output.position = float4(input.position.x, y, input.position.zw);

    output.position = mul(output.position, object);
    output.position = mul(output.position, projection);
    output.color = f_to_color(y);
    output.uv = input.uv;

    return output;
}

float4 pixel_main(PixelInputType input) : SV_TARGET {
    return input.color;
}