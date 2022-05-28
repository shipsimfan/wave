
struct Vertex {
    float4 position;
    float4 color;
    float2 uv;
};

RWStructuredBuffer<Vertex> previous_wave : register(u0);
RWStructuredBuffer<Vertex> current_wave : register(u1);
RWStructuredBuffer<Vertex> next_wave : register(u2);

static const float DT = 1.0 / 60.0;
static const float DX = 0.001;
static const float C = 0.05;
static const float R = (C * DT / DX) * (C * DT / DX);

[numthreads(1,1,1)]
void compute_main(uint3 tid : SV_DispatchThreadID) {
    float u_now = current_wave[tid.x].position.y;
    float u_last = previous_wave[tid.x].position.y;
    float lu = current_wave[tid.x - 1].position.y;
    float uu = current_wave[tid.x + 1].position.y;

    float new_u = 2.0 * u_now - u_last + R * (uu - 2.0 * u_now + lu);

    next_wave[tid.x].position.y = new_u;
}


