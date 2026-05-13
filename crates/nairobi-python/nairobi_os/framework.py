# File: /home/chege/nairobi-connector-open-source/crates/nairobi-python/nairobi_os/framework.py
# Author: Kevin Chege. Location: Nairobi
# Date: 2026-05-15

import json
from . import _core
from . import lagos

data = _core.data

class SovereignFrame:
    """
    A high-level, fluent interface for Nairobi OS data handles.
    Hides the complexity of D-Bus handles and daemon management.
    """
    def __init__(self, handle_id):
        self.handle_id = handle_id

    def crunch(self, column):
        """
        Returns the Axiom Crunch statistics for the specified column.
        """
        result_json = data.crunch(self.handle_id, column)
        return json.loads(result_json)

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
        new_handle_id = data.sql_query(self.handle_id, sql)
        return SovereignFrame(new_handle_id)

    def plot(self, width=1000, height=400):
        """
        Spawns the Lagos Vision widget for the current frame.
        """
        return lagos.plot_inline(self.handle_id, width=width, height=height)
