module add intel/19.1.3.304/6pv46so
module add intel-mkl/2020.4.304/vg6aq26

icpx \
  -O3 \
  -mkl=parallel \
  -o cpp_fft_bench \
  cpp_fft_bench.cpp
icpx \
  -O3 \
  -mkl=parallel \
  -o cpp_fft_bench \
  cpp_fft_bench.cpp
