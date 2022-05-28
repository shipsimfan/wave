
struct Vertex {
    float4 position;
    float4 color;
    float2 uv;
};

cbuffer Settings {
    float R;
    float DX;
    float DY;
    float DT;

    float COLOR_MOD;
    uint NUM_POINTS_X;
    uint NUM_POINTS_Y;
    float reserved;
}

RWStructuredBuffer<Vertex> previous_wave : register(u0);
RWStructuredBuffer<Vertex> current_wave : register(u1);
RWStructuredBuffer<Vertex> next_wave : register(u2);

float4 speed_to_color(float speed) {
    float value = saturate(3.0 * abs(speed));
    return float4(value, 0, 1.0 - value, 1.0);
}

uint index(uint x, uint y) {
    return x + y * NUM_POINTS_X;
}

[numthreads(16,16,1)]
void compute_main(uint3 tid : SV_DispatchThreadID) {
    uint idx = index(tid.x, tid.y);
    uint x_u_index = index(tid.x + 1, tid.y);
    uint x_l_index = index(tid.x - 1, tid.y);
    uint y_u_index = index(tid.x, tid.y + 1);
    uint y_l_index = index(tid.x, tid.y - 1);

    // Gather values
    float f_now = current_wave[idx].position.y;
    float f_prev = previous_wave[idx].position.y;
    float f_x_u = current_wave[x_u_index].position.y;
    float f_x_l = current_wave[x_l_index].position.y;
    float f_y_u = current_wave[y_u_index].position.y;
    float f_y_l = current_wave[y_l_index].position.y;

    // Compute new value
    float f_now_2 = 2.0 * f_now;

    float f_x = (f_x_u - f_now_2 + f_x_l) / (DX * DX);
    float f_y = (f_y_u - f_now_2 + f_y_l) / (DY * DY);

    float f_new = f_now_2 - f_prev + R * (f_x + f_y);

    // Set new value
    next_wave[idx].color = speed_to_color((f_now - f_new) / DT);
    next_wave[idx].position.y = f_new;
}


