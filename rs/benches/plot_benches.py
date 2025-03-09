#!/usr/bin/env python3
import os
import glob
import json
import matplotlib.pyplot as plt
from collections import defaultdict


def find_estimates_files(root_dir="target/criterion"):
    """
    Recursively searches for all 'estimates.json' files under the given directory.
    """
    pattern = os.path.join(root_dir, "**", "estimates.json")
    return glob.glob(pattern, recursive=True)


def common_prefix_and_remainder(strings):
    """
    This function takes a list of strings and returns a tuple:
    - the longest common prefix among all the strings
    - a list of the original strings without this prefix
    """
    if not strings:
        return "", []

    # Initialize the prefix with the first string
    prefix = strings[0]

    # Adjust the common prefix based on the other strings
    for s in strings[1:]:
        while not s.startswith(prefix):
            prefix = prefix[:-1]  # Remove one character from the right
            if prefix == "":
                break
        if prefix == "":
            break

    # Create a list of strings with the common prefix removed
    remainders = [s[len(prefix):] for s in strings]
    return prefix, remainders


def extract_benchmark_name_from_path(filepath):
    """
    Determines the complete benchmark name from the path.
    Example: 'target/criterion/mycase-variant/new/estimates.json' -> 'mycase-variant'
    """
    dirpath = os.path.dirname(filepath)  # e.g. 'target/criterion/mycase-variant/new'
    upper_dir = os.path.dirname(dirpath)  # e.g. 'target/criterion/mycase-variant'
    benchmark_name = os.path.basename(upper_dir)  # e.g. 'mycase-variant'
    return benchmark_name


def main():
    # 1) Find all estimates.json files
    files = find_estimates_files()

    # We'll store data in a nested dictionary: {case_name: {variant_name: (mean, std_dev)}}
    data_by_case = defaultdict(dict)

    # We'll collect all categories across all benchmarks so we can enforce a fixed order
    all_categories = set()

    # prefix, remainders = common_prefix_and_remainder(files)
    # print(f"Use {prefix} as common prefix")
    #
    # files_to_names = defaultdict(dict)
    #
    # for index, filepath in enumerate(files):
    #     files_to_names[filepath] = remainders[index]

    # 2) Read each JSON file and extract the measurements
    for filepath in files:
        try:
            with open(filepath, 'r') as f:
                data = json.load(f)
                # Retrieve mean point estimate and std_dev from Criterion
                mean_val = data["mean"]["point_estimate"]
                std_dev_val = data["std_dev"]["point_estimate"]

                # Extract the benchmark name from the path
                full_bench_name = extract_benchmark_name_from_path(filepath)

                # 3) Split into <CASE>-<VARIANT>
                # We do a rsplit('-', 1) to separate from the right
                if "-" in full_bench_name:
                    case_name, variant_name = full_bench_name.rsplit("-", 1)
                    if case_name == "":
                        case_name = "default"
                else:
                    case_name = "default"
                    variant_name = full_bench_name

                # case_name = "default"
                # variant_name = files_to_names[filepath]

                # Store the mean and std_dev in our data structure
                data_by_case[case_name][variant_name] = (mean_val, std_dev_val)

                # Keep track of the variant
                all_categories.add(variant_name)

        except Exception as e:
            print(f"Could not process file {filepath} : {e}")

    # 4) Determine a fixed order for categories (alphabetical)
    all_categories = sorted(all_categories)

    # 5) Assign a fixed color per variant
    cmap = plt.get_cmap("tab10")
    color_map = {}
    for i, cat in enumerate(all_categories):
        color_map[cat] = cmap(i % 10)  # if more than 10 categories, it wraps around

    # Create an output directory for the charts
    output_dir = "criterion_plots_by_case"
    os.makedirs(output_dir, exist_ok=True)

    # 6) Generate one bar chart per case
    for case_name, cat_data in data_by_case.items():

        # cat_data is a dict: {variant_name: (mean, std_dev)}
        means = []
        std_devs = []
        colors = []
        valid_categories = []

        # We'll go through all categories in the global sorted list
        # to ensure the order is consistent and the colors match
        for cat in all_categories:
            if cat in cat_data:
                m, s = cat_data[cat]
                means.append(m)
                std_devs.append(s)
                colors.append(color_map[cat])
                valid_categories.append(cat)
            else:
                # If this case does not have a benchmark for that variant, skip it
                pass

        # Plot the chart
        plt.figure(figsize=(8, 5))
        x_positions = range(len(valid_categories))

        # We use yerr to display error bars (standard deviation in this case)
        plt.bar(
            x_positions,
            means,
            yerr=std_devs,
            capsize=5,
            color=colors,
        )

        plt.xticks(x_positions, valid_categories, rotation=45, ha='right')
        if case_name != "default":
            plt.title(f"Benchmarks for case '{case_name}'")
        # plt.xlabel("Variant")
        plt.ylabel("Mean time (ns)")
        plt.tight_layout()

        # Save the figure
        output_path = os.path.join(output_dir, f"{case_name}.png")
        plt.savefig(output_path)
        plt.close()

        print(f"Saved chart to: {output_path}")

    print("Done.")


if __name__ == "__main__":
    main()
