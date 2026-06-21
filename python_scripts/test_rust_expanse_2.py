import param_sweep
import os

exec_path = "/home/rbentley/fft_bench/rust_fft_bench/target/release/rust_fft_bench"
results_dir = "/home/rbentley/fft_bench/results/expanse_2"
sweep_config = {
    "cli": {
        "threads": [1, 2, 16, 32, 64, 120, 122, 126, 127, 128],
        "plan_size": [8, 16, 27, 32, 64, 81, 125, 128, 243, 256, 343, 512, 625, 729, 1024, 2048, 2187, 2401, 3125, 4096, 6561, 8192, 15625, 16384, 16807, 19683, 32768, 59049, 65536, 78125, 117649, 131072, 177147, 262144, 390625, 524288, 531441, 823543, 1048576, 1594323, 1953125, 2097152, 4194304, 4782969, 5764801, 8388608, 9765625],
        "plan_type": ["estimate", "measure", "patient"],
        "test_count": [12],
    }
}

def run_params(params_outer):
    params = params_outer["cli"]
    plan_size = params["plan_size"]
    plan_type = params["plan_type"]
    test_count = params["test_count"]
    threads = params["threads"]
    wisdom_path = f"{results_dir}/wisdom_{plan_type}"
    output_path = f"{results_dir}/rust_{plan_size}_{threads}_{plan_type}.json"

    if os.path.exists(output_path):
        print(f"LOG: Already exists: {output_path}")
        return;

    command = f"{exec_path}"
    command += f" --plan-size {plan_size}"
    command += f" --plan-type {plan_type}"
    command += f" --threads {threads}"
    command += f" --test-count {test_count}"
    command += f" --wisdom-path {wisdom_path}"
    command += f" --output {output_path}"
    os.system(command);

def main():
    sweep_params = param_sweep.generate(sweep_config)
    print(f"LOG: |sweep_params| = {len(sweep_params)}")
    for param in sweep_params:
        run_params(param)

main()
