import nairobi_os
import time

print("==========================================")
print("Nairobi OS v0.5.0: Visual Canvas Live-Fire")
print("==========================================")

# 1. Ignite the OS (Spawns Axum Refinery and Nairobi Hub)
nairobi_os.ignite()

# 2. Launch the Visual Canvas (egui-snarl)
print("🎨 Launching Nairobi Canvas... Draw your DAG and click 'Compile & Close'.")
dag_bytes = nairobi_os.canvas.open()

if dag_bytes:
    print(f"\n✅ Visual Compiler Output: {len(dag_bytes)} bytes of GVariant DAG")
    
    # 3. Execute the DAG (Hub deserializes and orchestrates)
    print("🚀 Passing GVariant DAG to Nairobi Hub for bare-metal execution...")
    t0 = time.time()
    nairobi_os.canvas.execute(dag_bytes)
    latency = (time.time() - t0) * 1000
    print(f"🎉 Pipeline Execution Complete in {latency:.2f} ms")
else:
    print("❌ Canvas closed without compilation.")