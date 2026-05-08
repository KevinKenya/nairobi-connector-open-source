# Author: Kevin Chege. Location: Nairobi

# nairobi-benchmarks/visualization/plot_scaling.py
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import argparse
import os

def plot_results(csv_path, output_dir):
    df = pd.read_csv(csv_path)
    
    # Set style
    sns.set_theme(style="whitegrid")
    
    # 1. Latency Comparison
    plt.figure(figsize=(10, 6))
    sns.barplot(data=df, x="Workload", y="Mean Latency (ms)", hue="Engine")
    plt.title("Latency Comparison: Nairobi OS vs Competitors")
    plt.yscale("log")
    plt.ylabel("Mean Latency (ms) - Log Scale")
    plt.xticks(rotation=15)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, "latency_comparison.png"))
    
    # 2. Memory Footprint
    plt.figure(figsize=(10, 6))
    sns.barplot(data=df, x="Workload", y="Peak RAM (MB)", hue="Engine")
    plt.title("Memory Footprint (Peak RSS)")
    plt.ylabel("Peak RAM (MB)")
    plt.xticks(rotation=15)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, "memory_footprint.png"))
    
    # 3. CPU Efficiency
    plt.figure(figsize=(10, 6))
    sns.barplot(data=df, x="Workload", y="Peak CPU (%)", hue="Engine")
    plt.title("Peak CPU Utilization (Rayon/SIMD Efficiency)")
    plt.ylabel("Peak CPU (%)")
    plt.xticks(rotation=15)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, "cpu_efficiency.png"))

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", default="results/processed/summary.csv")
    parser.add_argument("--output_dir", default="results/plots")
    args = parser.parse_args()
    
    os.makedirs(args.output_dir, exist_ok=True)
    if os.path.exists(args.input):
        plot_results(args.input, args.output_dir)
    else:
        print(f"Error: {args.input} not found.")
