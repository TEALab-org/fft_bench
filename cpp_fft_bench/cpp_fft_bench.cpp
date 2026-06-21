// Written by Gemini Pro, based on rust_fft_bench
// 06 / 21 / 2026

#include <iostream>
#include <vector>
#include <chrono>
#include <random>
#include <fstream>
#include <string>
#include <complex>
#include <numeric>
#include <mkl.h>

// Custom allocator to ensure 64-byte alignment, matching your Rust AlignedVec behavior
template <typename T, std::size_t Align = 64>
struct AlignedAllocator {
    using value_type = T;
    
    AlignedAllocator() noexcept = default;
    template <typename U> AlignedAllocator(const AlignedAllocator<U, Align>&) noexcept {}

    // Required by older standard libraries (like GCC 8's libstdc++)
    template <typename U>
    struct rebind {
        using other = AlignedAllocator<U, Align>;
    };

    T* allocate(std::size_t n) {
        if (n == 0) return nullptr;
        void* ptr = mkl_malloc(n * sizeof(T), Align);
        if (!ptr) throw std::bad_alloc();
        return static_cast<T*>(ptr);
    }

    void deallocate(T* p, std::size_t) noexcept {
        mkl_free(p);
    }

    // Required for standard allocator completeness
    bool operator==(const AlignedAllocator&) const noexcept { return true; }
    bool operator!=(const AlignedAllocator&) const noexcept { return false; }
};

int main(int argc, char* argv[]) {
    if (argc != 5) {
        std::cerr << "Usage: cpp_fft_bench <plan_size> <threads> <iterations> <output_json_path>\n";
        return 1;
    }

    // Parse arguments
    size_t plan_size = std::stoull(argv[1]);
    int threads = std::stoi(argv[2]);
    size_t test_count = std::stoull(argv[3]);
    std::string output_path = argv[4];

    std::cout << "FFT_BENCH: Start\n";
    std::cout << "Plan Size: " << plan_size << "\nThreads: " << threads 
              << "\nIterations: " << test_count << "\nOutput: " << output_path << "\n";

    // Set MKL threads
    mkl_set_num_threads(threads);

    // Create Buffers (Aligned)
    using RealVec = std::vector<double, AlignedAllocator<double>>;
    using ComplexVec = std::vector<std::complex<double>, AlignedAllocator<std::complex<double>>>;

    RealVec real_buffer(plan_size);
    ComplexVec complex_buffer(plan_size / 2 + 1);

    std::cout << "FFT_BENCH: Create Initial Conditions\n";
    std::mt19937_64 rng(12345); // Fixed seed for reproducibility, or use std::random_device{}()
    std::uniform_real_distribution<double> dist(-1.0, 1.0);
    
    double initial_sum = 0.0;
    for (size_t i = 0; i < plan_size; ++i) {
        double value = dist(rng);
        initial_sum += value;
        real_buffer[i] = value;
    }
    std::cout << "FFT_BENCH: Initial Sum " << initial_sum << "\n";

    std::cout << "FFT_BENCH: Create FFT Plans\n";
    DFTI_DESCRIPTOR_HANDLE descriptor = nullptr;
    
    // Create 1D Real-to-Complex descriptor
    MKL_LONG status = DftiCreateDescriptor(&descriptor, DFTI_DOUBLE, DFTI_REAL, 1, plan_size);
    if (status && !DftiErrorClass(status, DFTI_NO_ERROR)) {
        std::cerr << "Error creating MKL descriptor: " << DftiErrorMessage(status) << "\n";
        return 1;
    }

    // Configure for out-of-place transforms (matching the Rust r2c/c2r)
    DftiSetValue(descriptor, DFTI_PLACEMENT, DFTI_NOT_INPLACE);
    // Tell MKL to use standard complex arrays rather than packed CCS format
    DftiSetValue(descriptor, DFTI_CONJUGATE_EVEN_STORAGE, DFTI_COMPLEX_COMPLEX);
    
    status = DftiCommitDescriptor(descriptor);
    if (status && !DftiErrorClass(status, DFTI_NO_ERROR)) {
        std::cerr << "Error committing MKL descriptor: " << DftiErrorMessage(status) << "\n";
        return 1;
    }

    std::vector<uint64_t> timings;
    timings.reserve(test_count);
    double n_r = static_cast<double>(plan_size);

    for (size_t test = 0; test < test_count; ++test) {
        auto start_time = std::chrono::high_resolution_clock::now();

        // Forward Transform (Real to Complex)
        DftiComputeForward(descriptor, real_buffer.data(), complex_buffer.data());
        
        // Backward Transform (Complex to Real)
        DftiComputeBackward(descriptor, complex_buffer.data(), real_buffer.data());

        auto end_time = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::nanoseconds>(end_time - start_time).count();
        timings.push_back(duration);

        std::cout << "FFT_BENCH: test " << test << ", duration " << duration << " ns\n";

        // Normalize data (Matching Rust iter_mut / n_r)
        for (size_t i = 0; i < plan_size; ++i) {
            real_buffer[i] /= n_r;
        }
    }

    // Check drift
    double final_sum = 0.0;
    for(const auto& val : real_buffer) {
        final_sum += val;
    }
    
    std::cout << "FFT_BENCH: Final Sum " << final_sum << "\n";
    std::cout << "FFT_BENCH: Sum Drift: " << (final_sum - initial_sum) << "\n";

    // Write output to JSON
    std::cout << "FFT_BENCH: Writing Output\n";
    std::ofstream out_file(output_path);
    if (out_file.is_open()) {
        out_file << "{\n";
        out_file << "  \"name\": \"cpp_mkl_fft_bench\",\n";
        out_file << "  \"threads\": " << threads << ",\n";
        out_file << "  \"plan_type\": \"MKL_OUT_OF_PLACE\",\n";
        out_file << "  \"plan_size\": " << plan_size << ",\n";
        out_file << "  \"timings\": [\n    ";
        for (size_t i = 0; i < timings.size(); ++i) {
            out_file << timings[i];
            if (i < timings.size() - 1) {
                out_file << ",\n    ";
            }
        }
        out_file << "\n  ]\n";
        out_file << "}\n";
        out_file.close();
    } else {
        std::cerr << "ERROR: Could not open output file " << output_path << "\n";
    }

    // Cleanup
    DftiFreeDescriptor(&descriptor);

    std::cout << "FFT_BENCH: End\n";
    return 0;
}
