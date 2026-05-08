# Author: Kevin Chege. Location: Nairobi

# nairobi-benchmarks/orchestration/metrics_collector.py
import time
import psutil
import threading
import os
import resource

class MetricsCollector:
    def __init__(self):
        self.stop_event = threading.Event()
        self.peak_cpu = 0.0
        self.peak_ram_rss = 0.0
        self.thread = None

    def _collect_background(self, pid):
        process = psutil.Process(pid)
        while not self.stop_event.is_set():
            try:
                # Capture Peak CPU (interval=0.1s for precision)
                cpu_percent = process.cpu_percent(interval=0.1)
                if cpu_percent > self.peak_cpu:
                    self.peak_cpu = cpu_percent
                
                # Capture RAM
                mem_info = process.memory_info()
                if mem_info.rss > self.peak_ram_rss:
                    self.peak_ram_rss = mem_info.rss
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                break

    def start(self):
        self.stop_event.clear()
        self.peak_cpu = 0.0
        self.peak_ram_rss = 0.0
        # Start background monitoring thread
        self.thread = threading.Thread(target=self._collect_background, args=(os.getpid(),))
        self.thread.daemon = True
        self.thread.start()

    def stop(self):
        self.stop_event.set()
        if self.thread:
            self.thread.join()
        
        # Cross-verify RAM with resource.getrusage (Peak RSS in bytes on Linux)
        rusage_peak = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss * 1024
        if rusage_peak > self.peak_ram_rss:
            self.peak_ram_rss = rusage_peak

    def get_metrics(self):
        return {
            "peak_cpu_percent": self.peak_cpu,
            "peak_ram_bytes": self.peak_ram_rss,
            "peak_ram_mb": self.peak_ram_rss / (1024 * 1024)
        }

def measure_wall_time_ns(func, *args, **kwargs):
    t0 = time.perf_counter_ns()
    result = func(*args, **kwargs)
    t1 = time.perf_counter_ns()
    return result, t1 - t0
