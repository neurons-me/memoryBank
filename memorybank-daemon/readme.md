memorybank-daemon/
├── src/
│   ├── main.rs
│   ├── config.rs          <-- manejo de Config y almacenamiento
│   ├── ipc.rs             <-- manejo de comunicación por Unix socket
│   ├── commands.rs        <-- enum DaemonCommand + handlers
│   ├── keywatch.rs        <-- ya existente