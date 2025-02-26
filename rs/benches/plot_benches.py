#!/usr/bin/env python3
import os
import glob
import json
import matplotlib.pyplot as plt
from collections import defaultdict

def find_estimates_files(root_dir="target/criterion"):
    """
    Search for all 'estimates.json' files under the given directory (recursively).
    """
    pattern = os.path.join(root_dir, "**", "estimates.json")
    return glob.glob(pattern, recursive=True)

def extract_benchmark_name_from_path(filepath):
    """
    Determine the full benchmark name from the file path.
    Example: 'target/criterion/my_function_cat/new/estimates.json' -> 'my_function_cat'
    """
    # Parent directory of 'estimates.json'
    dirpath = os.path.dirname(filepath)
    # One level higher, to extract the benchmark name
    # (because 'estimates.json' is usually located in something/new/estimates.json)
    upper_dir = os.path.dirname(dirpath)
    benchmark_name = os.path.basename(upper_dir)
    return benchmark_name

def main():
    # 1) Find all estimates.json files
    files = find_estimates_files()

    # Nested dictionary: {function: {category: (mean, stddev)}}
    data_by_function = defaultdict(dict)

    # To keep track of all encountered categories, for ordering and coloring
    all_categories = set()

    for filepath in files:
        try:
            with open(filepath, 'r') as f:
                data = json.load(f)
                # Retrieve point_estimate (mean) and std_dev (standard deviation)
                mean_val = data["mean"]["point_estimate"]
                std_dev_val = data["std_dev"]["point_estimate"]

                # 2) Extract the full benchmark name from the path
                full_bench_name = extract_benchmark_name_from_path(filepath)

                # 3) Split into <FUNCTION>-<CATEGORY>
                #    If the function name itself contains underscores, use rsplit(-, 1)
                #    to split from the end.
                if "-" in full_bench_name:
                    function_name, category_name = full_bench_name.rsplit("-", 1)
                else:
                    # If there is no underscore, assign the entire string to function_name
                    function_name = full_bench_name
                    category_name = "default"

                # Store the values
                data_by_function[function_name][category_name] = (mean_val, std_dev_val)

                # Add the category to the global list
                all_categories.add(category_name)

        except Exception as e:
            print(f"Unable to read {filepath}: {e}")

    # 4) Determine a fixed order for the categories, e.g., alphabetical order
    all_categories = sorted(all_categories)

    # 5) Assign a fixed color per category (and keep it consistent)
    #    Choose a colormap in matplotlib with enough distinct colors.
    cmap = plt.get_cmap("tab10")  # tab10, tab20, Set1, etc.
    color_map = {}
    for i, cat in enumerate(all_categories):
        color_map[cat] = cmap(i % 10)  # If more than 10 categories, loop over colors

    # 6) Create a chart per function
    output_dir = "criterion_plots_by_function"
    os.makedirs(output_dir, exist_ok=True)

    for function_name, cat_data in data_by_function.items():
        # cat_data is a dict: {category: (mean, std_dev)}

        # Retrieve data in the order of the global categories
        means = []
        std_devs = []
        colors = []
        valid_categories = []

        for cat in all_categories:
            if cat in cat_data:
                m, s = cat_data[cat]
                means.append(m)
                std_devs.append(s)
                colors.append(color_map[cat])
                valid_categories.append(cat)
            else:
                # This function has no test for this category => ignore it
                pass

        # Create the figure
        plt.figure(figsize=(8, 5))

        x_positions = range(len(valid_categories))
        plt.bar(
            x_positions,
            means,
            yerr=std_devs,
            capsize=5,
            color=colors,
        )

        plt.xticks(x_positions, valid_categories, rotation=45, ha='right')
        plt.title(f"Benchmarks for function '{function_name}'")
        # plt.xlabel("Category")
        plt.ylabel("Average time (ns)")
        plt.tight_layout()

        # Save the chart
        output_path = os.path.join(output_dir, f"{function_name}.png")
        plt.savefig(output_path)
        plt.close()  # Close the figure to avoid memory accumulation

        print(f"Chart saved: {output_path}")

    print("Done.")

if __name__ == "__main__":
    main()
