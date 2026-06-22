import param_sweep
import os

rust_exec_path = "/home/rbentley/fft_bench/rust_fft_bench/target/release/rust_fft_bench"
cpp_exec_path = "/home/rbentley/fft_bench/cpp_fft_bench/cpp_fft_bench"
results_dir = "/home/rbentley/fft_bench/results/expanse_3"
sweep_config = {
    "cli": {
        "threads": [1],
        "plan_size": [i for i in range(1000, 14001)] + [16384],
        "plan_type": ["estimate", "measure"],
        "test_count": [12],
    },
    "tool": {
        "tool": ["cpp", "rust"]
    }
}

def run_cpp_params(params):
    plan_size = params["plan_size"]
    test_count = params["test_count"]
    threads = params["threads"]
    output_path = f"{results_dir}/cpp_{plan_size}_{threads}.json"

    if os.path.exists(output_path):
        print(f"LOG: Already exists: {output_path}")
        return;

    command = f"{cpp_exec_path}"
    command += f" {plan_size}"
    command += f" {threads}"
    command += f" {test_count}"
    command += f" {output_path}"
    os.system(command);

def run_rust_params(params):
    plan_size = params["plan_size"]
    plan_type = params["plan_type"]
    test_count = params["test_count"]
    threads = params["threads"]
    wisdom_path = f"{results_dir}/wisdom_{plan_type}"
    output_path = f"{results_dir}/rust_{plan_size}_{threads}_{plan_type}.json"

    if os.path.exists(output_path):
        print(f"LOG: Already exists: {output_path}")
        return;

    command = f"{rust_exec_path}"
    command += f" --plan-size {plan_size}"
    command += f" --plan-type {plan_type}"
    command += f" --threads {threads}"
    command += f" --test-count {test_count}"
    command += f" --wisdom-path {wisdom_path}"
    command += f" --output {output_path}"
    os.system(command);

def run_params(params_outer):
    params = params_outer["cli"]
    tool = params_outer["tool"]["tool"]
    if tool == "rust":
        run_rust_params(params)
    elif tool == "cpp":
        run_cpp_params(params)
    
def main():
    sweep_params = param_sweep.generate(sweep_config)
    print(f"LOG: |sweep_params| = {len(sweep_params)}")
    for param in sweep_params:
        run_params(param)

main()
