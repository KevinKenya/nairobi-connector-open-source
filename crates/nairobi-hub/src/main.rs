// Copyright 2026 Kevin Chege
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use nairobi_protocol::ImperialError;
use tracing::info;
use zbus::{Connection, dbus_interface, fdo::Error};
use tokio::sync::Mutex;

struct HubService {
    executor: Mutex<nairobi_hub::DagExecutor>,
}

#[dbus_interface(name = "org.nairobi.NairobiHub1")]
impl HubService {
    async fn execute_dag(&self, dag_bytes: Vec<u8>) -> Result<String, Error> {
        let mut executor = self.executor.lock().await;
        match executor.execute(dag_bytes).await {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::Failed(format!("Execution failed: {}", e))),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ImperialError> {
    tracing_subscriber::fmt::init();

    let connection = Connection::session().await.map_err(|e| {
        ImperialError::SystemicSeizure(format!("Failed to connect to D-Bus: {}", e))
    })?;

    let executor = nairobi_hub::DagExecutor::new()
        .await
        .map_err(|e| ImperialError::SystemicSeizure(format!("Failed to initialize executor: {}", e)))?;

    let service = HubService {
        executor: Mutex::new(executor),
    };

    let _ = connection
        .object_server()
        .at("/org/nairobi/NairobiHub1", service)
        .await
        .map_err(|e| ImperialError::SystemicSeizure(format!("Failed to register service: {}", e)))?;

    info!("Nairobi Hub is live on D-Bus at org.nairobi.NairobiHub1");

    tokio::signal::ctrl_c().await.map_err(|e| {
        ImperialError::SystemicSeizure(format!("Failed to wait for shutdown: {}", e))
    })?;

    Ok(())
}