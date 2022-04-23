__kernel void update(__global float* input, __global float* output, uint width, uint height) {
    // We use intentionally an 1D array

    // Get the coordinates of the worker:
    int x = get_global_id(0);
    int y = get_global_id(1);

    // Draw 2D-sin() as test:
    output[x+y*width] = sin((float)(2*M_PI*7*x/((float)width) + 2*M_PI*3*y/((float)height))) ;

    // Add input data as test:
    output[x+y*width] += input[x+y*width];
}

