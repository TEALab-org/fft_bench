#!/bin/bash
#SBATCH --job-name="expanse_4"
#SBATCH --partition=compute
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=1
#SBATCH --mem=200G
#SBATCH --account=sun127
#SBATCH --export=ALL
#SBATCH -t 06:30:00

python /home/rbentley/fft_bench/python_scripts/test_rust_expanse_4.py
