
struct Vertex {
    float4 position;
    float4 color;
    float2 uv;
};

cbuffer Settings {
    float R;
    float color_mod;
    float DT;
    float DX;
}

RWStructuredBuffer<Vertex> previous_wave : register(u0);
RWStructuredBuffer<Vertex> current_wave : register(u1);
RWStructuredBuffer<Vertex> next_wave : register(u2);

float4 speed_to_color(float speed) {
    float value = saturate(color_mod * abs(speed));
    return float4(value, 0, 1.0 - value, 1.0);
}

[numthreads(64,1,1)]
void compute_main(uint3 tid : SV_DispatchThreadID) {
    float u_now = current_wave[tid.x].position.y;
    float u_last = previous_wave[tid.x].position.y;
    float lu = current_wave[tid.x - 1].position.y;
    float uu = current_wave[tid.x + 1].position.y;

    float new_u = 2.0 * u_now - u_last + R * (uu - 2.0 * u_now + lu);
    float4 new_color = speed_to_color((u_now - new_u) / DT);
    
    next_wave[tid.x].color = new_color;
    next_wave[tid.x].position.y = new_u;
}


