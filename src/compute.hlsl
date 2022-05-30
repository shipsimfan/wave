
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

RWStructuredBuffer<float> previous_wave : register(u0);
RWStructuredBuffer<float> current_wave : register(u1);
RWStructuredBuffer<float> next_wave : register(u2);

RWTexture2D<float> output: register(u3);

uint index(uint x, uint y) {
    return x + y * NUM_POINTS_X;
}

[numthreads(16,16,1)]
void compute_main(uint3 tid : SV_DispatchThreadID) {
    uint idx = index(tid.x, tid.y);
    uint x_u_index = tid.x == NUM_POINTS_X - 1 ? 0.0 : index(tid.x + 1, tid.y);
    uint x_l_index = tid.x == 0 ? 0.0 : index(tid.x - 1, tid.y);
    uint y_u_index = index(tid.x, tid.y + 1);
    uint y_l_index = index(tid.x, tid.y - 1);

    // Gather values
    float f_now = current_wave[idx];
    float f_prev = previous_wave[idx];
    float f_x_u = current_wave[x_u_index];
    float f_x_l = current_wave[x_l_index];
    float f_y_u = current_wave[y_u_index];
    float f_y_l = current_wave[y_l_index];

    // Compute new value
    float f_now_2 = 2.0 * f_now;

    float f_x = (f_x_u - f_now_2 + f_x_l) / (DX * DX);
    float f_y = (f_y_u - f_now_2 + f_y_l) / (DY * DY);

    float f_new = f_now_2 - f_prev + R * (f_x + f_y);

    // Set new value
    next_wave[idx] = f_new;
    output[tid.xy] = f_new;
}


