import nairobi_os as nb

# Open the visual canvas for DAG compilation
dag_bytes = nb.canvas.open()

# Execute the compiled pipeline
if dag_bytes:
    nb.canvas.execute(dag_bytes)
