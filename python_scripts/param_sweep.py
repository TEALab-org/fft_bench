import copy

# generate_param_sweep(sweep_params)
#
# For a specification of all values to try for a given key,
# genate a list of all combination of key -> value combinations.
def generate(sweep_params):
    # We use double buffer,
    # for each new parameter, 
    # add all new variations based on previous variations
    # Populate initial, empty, config with top level keys
    initial_object = {}
    for top_key in sweep_params:
        initial_object[top_key] = {}

    old_buffer = [initial_object]
    new_buffer = []

    for top_key in sweep_params:
        for key in sweep_params[top_key]:
            for v in sweep_params[top_key][key]:
                for param in old_buffer:
                    new_param = copy.deepcopy(param)
                    new_param[top_key][key] = v
                    new_buffer.append(new_param)
            old_buffer = new_buffer
            new_buffer = []

    print("Found %d param instances" % len(old_buffer))
    return old_buffer
