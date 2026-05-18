# Copyright 2026 Kevin Chege
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Author: Kevin Chege. Location: Nairobi

# nairobi-benchmarks/orchestration/result_validator.py
import math
import json

class ResultValidator:
    # Statistical fields that should be validated for mathematical accuracy
    STATISTICAL_FIELDS = {"mean", "std_dev", "skewness", "kurtosis", "pearson"}
    
    # Latency fields that should NOT be validated (they vary by engine)
    LATENCY_FIELDS = {"ingest_ms", "crunch_ms", "corr_ms", "total_ms"}
    
    def __init__(self, tolerance=1e-5):
        self.tolerance = tolerance

    def validate(self, engine_name, results, reference_results):
        """
        Validates that the results from an engine match the reference results.
        Only validates statistical/mathematical fields, not latency fields.
        Results is a dict: {"mean": float, "std_dev": float, "skewness": float, "kurtosis": float, "pearson": float, 
                            "ingest_ms": float, "crunch_ms": float, "corr_ms": float, "total_ms": float}
        """
        if reference_results is None:
            # First engine sets the baseline (usually Pandas or DuckDB)
            return True, "Baseline established."

        errors = []
        for key in reference_results:
            # Skip latency fields - they are expected to differ between engines
            if key in self.LATENCY_FIELDS:
                continue
            
            if key not in results:
                errors.append(f"Missing key: {key}")
                continue
            
            val = results[key]
            ref = reference_results[key]
            
            if val is None or ref is None:
                continue

            if not math.isclose(val, ref, rel_tol=self.tolerance, abs_tol=self.tolerance):
                errors.append(f"{key}: {val} != {ref} (tol={self.tolerance})")

        if errors:
            return False, f"Validation Failed for {engine_name}: " + "; ".join(errors)
        
        return True, f"Validation Passed for {engine_name}."

def print_validation_report(success, message):
    if success:
        print(f"✅ {message}")
    else:
        print(f"❌ {message}")
