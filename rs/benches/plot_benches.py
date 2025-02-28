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

def extract_benchmark_name_from_path(filepath):
    """
    Determines the complete benchmark name from the path.
    Example: 'target/criterion/myfunction-category/new/estimates.json' -> 'myfunction-category'
    """
    dirpath = os.path.dirname(filepath)   # e.g. 'target/criterion/myfunction-category/new'
    upper_dir = os.path.dirname(dirpath)  # e.g. 'target/criterion/myfunction-category'
    benchmark_name = os.path.basename(upper_dir)  # e.g. 'myfunction-category'
    return benchmark_name

def main():
    # 1) Find all estimates.json files
    files = find_estimates_files()

    # We'll store data in a nested dictionary: {function_name: {category_name: (mean, std_dev)}}
    data_by_function = defaultdict(dict)

    # We'll collect all categories across all benchmarks so we can enforce a fixed order
    all_categories = set()

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

                # 3) Split into <FUNCTION>-<CATEGORY>
                # We do a rsplit('-', 1) to separate from the right
                if "-" in full_bench_name:
                    function_name, category_name = full_bench_name.rsplit("-", 1)
                else:
                    function_name = full_bench_name
                    category_name = "default"

                # Store the mean and std_dev in our data structure
                data_by_function[function_name][category_name] = (mean_val, std_dev_val)

                # Keep track of the category
                all_categories.add(category_name)

        except Exception as e:
            print(f"Could not process file {filepath} : {e}")

    # 4) Determine a fixed order for categories (alphabetical)
    all_categories = sorted(all_categories)

    # 5) Assign a fixed color per category
    cmap = plt.get_cmap("tab10")
    color_map = {}
    for i, cat in enumerate(all_categories):
        color_map[cat] = cmap(i % 10)  # if more than 10 categories, it wraps around

    # Create an output directory for the charts
    output_dir = "criterion_plots_by_function"
    os.makedirs(output_dir, exist_ok=True)

    # 6) Generate one bar chart per function
    for function_name, cat_data in data_by_function.items():

        # cat_data is a dict: {category_name: (mean, std_dev)}
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
                # If this function does not have a benchmark for that category, skip it
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
        plt.title(f"Benchmarks for function '{function_name}'")
        # plt.xlabel("Category")
        plt.ylabel("Mean time (ns)")
        plt.tight_layout()

        # Save the figure
        output_path = os.path.join(output_dir, f"{function_name}.png")
        plt.savefig(output_path)
        plt.close()

        print(f"Saved chart to: {output_path}")

    print("Done.")

if __name__ == "__main__":
    main()
