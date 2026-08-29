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

# File: crates/nairobi-python/nairobi_os/framework.py
# Author: Kevin Chege. Location: Nairobi
# Date: 2026-05-21

import json
from . import _core
from . import lagos

data = _core.data

class ColumnAccessor:
    """
    Helper class to allow fluent column access like df.PTS.mean()
    """
    def __init__(self, frame, column):
        self._frame = frame
        self._column = column
        
    def crunch(self):
        return self._frame.crunch(self._column)
        
    def mean(self):
        return self._frame.mean(self._column)
        
    def max(self):
        return self._frame.max(self._column)
        
    def min(self):
        return self._frame.min(self._column)
        
    def std_dev(self):
        return self._frame.std_dev(self._column)
        
    def variance(self):
        return self._frame.variance(self._column)
        
    def skewness(self):
        return self._frame.skewness(self._column)
        
    def kurtosis(self):
        return self._frame.kurtosis(self._column)
        
    def p95(self):
        return self._frame.p95(self._column)
        
    def p99(self):
        return self._frame.p99(self._column)
        
    def calculate(self):
        return self._frame.calculate(self._column)


class SovereignFrame:
    """
    A high-level, fluent interface for Nairobi OS data handles.
    Hides the complexity of D-Bus handles and daemon management.
    """
    def __init__(self, handle_id):
        self.handle_id = handle_id
        self._crunch_cache = {}  # Cache for crunch results by column

    def __getattr__(self, name):
        """Allow column access via attribute: df.PTS"""
        # Exclude internal/private attributes from column accessor
        if name.startswith('_'):
            raise AttributeError(f"'{self.__class__.__name__}' object has no attribute '{name}'")
        return ColumnAccessor(self, name)

    def __getitem__(self, name):
        """Allow column access via dictionary syntax: df['PTS']"""
        return ColumnAccessor(self, name)

    def free(self):
        """Frees the underlying D-Bus memfd handle."""
        data.free(self.handle_id)

    def _get_crunch_result(self, column):
        """Get crunch result for column, computing and caching if necessary."""
        if column not in self._crunch_cache:
            result_json = data.crunch(self.handle_id, column)
            self._crunch_cache[column] = json.loads(result_json)
        return self._crunch_cache[column]

    def crunch(self, column):
        """
        Returns the Axiom Crunch statistics for the specified column.
        """
        return self._get_crunch_result(column)

    def mean(self, column):
        """
        Returns the mean value for the specified column.
        """
        result = self._get_crunch_result(column)
        return result['mean']

    def max(self, column):
        """
        Returns the maximum value for the specified column.
        """
        result = self._get_crunch_result(column)
        return result['max']

    def min(self, column):
        """
        Returns the minimum value for the specified column.
        """
        result = self._get_crunch_result(column)
        return result['min']

    def std_dev(self, column):
        """
        Returns the standard deviation for the specified column.
        """
        result = self._get_crunch_result(column)
        return result['std_dev']

    def variance(self, column):
        """
        Returns the variance for the specified column.
        """
        result = self._get_crunch_result(column)
        return result['variance']

    def skewness(self, column):
        """
        Returns the skewness for the specified column.
        """
        result = self._get_crunch_result(column)
        return result['skewness']

    def kurtosis(self, column):
        """
        Returns the kurtosis for the specified column.
        """
        result = self._get_crunch_result(column)
        return result['kurtosis']

    def p95(self, column):
        """
        Returns the 95th percentile for the specified column.
        """
        result = self._get_crunch_result(column)
        return result['p95']

    def p99(self, column):
        """
        Returns the 99th percentile for the specified column.
        """
        result = self._get_crunch_result(column)
        return result['p99']

    def calculate(self, column=None):
        """
        Returns statistical math for the specified column, or all numeric columns if None.
        Alias for crunch() for Pandas-like familiarity.
        """
        if column is None:
            # Calculate for all columns - this would need a different approach
            # For now, we'll require a column specification to match the existing API
            raise ValueError("calculate() requires a column parameter. Use crunch(column) for specific column stats.")
        return self.crunch(column)

    def correlate(self, cols):
        """
        Returns the Relational Strike correlation matrix for the specified columns.
        """
        result_json = data.correlate(self.handle_id, cols)
        return json.loads(result_json)

    def query(self, sql):
        """
        Executes a SQL query on the frame and returns a new SovereignFrame
        containing the distilled result.
        """
        import re
        rewritten_sql = re.sub(r'\bdf\b', 'dataset', sql, flags=re.IGNORECASE)
        new_handle_id = data.sql_query(self.handle_id, rewritten_sql)
        return SovereignFrame(new_handle_id)

    def plot(self, width=1000, height=400):
        """
        Spawns the Lagos Vision rendering pipeline for the current frame.
        Supports both interactive inline display in Jupyter notebooks and
        headless offscreen rendering via lagos-vision-daemon.
        """
        return lagos.plot_inline(self.handle_id, width=width, height=height)

