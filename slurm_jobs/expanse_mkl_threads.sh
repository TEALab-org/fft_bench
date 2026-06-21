#!/bin/bash
#SBATCH --job-name="mkl_threads"
#SBATCH --partition=compute
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=1
#SBATCH --mem=200G
#SBATCH --account=sun127
#SBATCH --export=ALL
#SBATCH -t 06:30:00

module add intel/19.1.3.304/6pv46so
module add intel-mkl/2020.4.304/vg6aq26

python /home/rbentley/fft_bench/python_scripts/test_rust_expanse_mkl_threads.py
