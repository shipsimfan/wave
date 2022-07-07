cbuffer Settings {
    float2 DTH_2MI; // dt * h_bar / (2 * m * i)
    float DX2; // dx ^ 2
    float DY2; // dy ^ 2
    uint NUM_POINTS_X;
    uint NUM_POINTS_Y;
    float2 reserved;
}

RWStructuredBuffer<float2> previous_wave : register(u0);
RWStructuredBuffer<float2> current_wave : register(u1);
RWStructuredBuffer<float2> next_wave : register(u2);

RWTexture2D<float> output: register(u3);

uint index(uint x, uint y) {
    return x + y * NUM_POINTS_X;
}

float2 c_mul(float2 c1, float2 c2) {
	return float2(c1.x  *c2.x - c1.y * c2.y, c1.y * c2.x + c1.x * c2.y);
}

[numthreads(16,16,1)]
void compute_main(uint3 tid : SV_DispatchThreadID) {
    uint idx = index(tid.x, tid.y);
    uint x_u_index = tid.x == NUM_POINTS_X - 1 ? idx : index(tid.x + 1, tid.y);
    uint x_l_index = tid.x == 0 ? idx : index(tid.x - 1, tid.y);
    uint y_u_index = tid.y == NUM_POINTS_Y - 1 ? idx : index(tid.x, tid.y + 1);
    uint y_l_index = tid.y == 0 ? idx : index(tid.x, tid.y - 1);

    // Gather values
    float2 psi_now = current_wave[idx];
    float2 psi_prev = previous_wave[idx];
    float2 psi_x_u = current_wave[x_u_index];
    float2 psi_x_l = current_wave[x_l_index];
    float2 psi_y_u = current_wave[y_u_index];
    float2 psi_y_l = current_wave[y_l_index];

    // Compute new value
    float2 psi_now_2 = 2.0 * psi_now;
    
    float2 new_psi_x = (psi_x_u - psi_now_2 + psi_x_l) / DX2;
    float2 new_psi_y = (psi_y_u - psi_now_2 + psi_y_l) / DY2;
    float2 psi_grad = c_mul(DTH_2MI, new_psi_x + new_psi_y);

    float2 psi_new = psi_now - psi_grad;

    // Set new value
    next_wave[idx] = psi_new;
    output[tid.xy] = psi_new.x * psi_new.x + psi_new.y * psi_new.y;
}


