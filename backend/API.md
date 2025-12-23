# API Documentation

## Base URL
```
http://localhost:3000/api
```

## Health Check

### GET /health
Check if the server is running.

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "ok"
  },
  "message": "Service is healthy"
}
```

---

## Node Information

### GET /node/info
Get information about the connected Bitcoin node.

**Response:**
```json
{
  "success": true,
  "data": {
    "network": "regtest",
    "block_height": 101,
    "best_block_hash": "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206",
    "sync": {
      "is_synced": true,
      "progress": 1.0
    },
    "pruned": false,
    "difficulty": 4.656542373906925e-10,
    "headers": 101,
    "verification_progress": 1.0,
    "backend": {
      "version": "Bitcoin Core",
      "node_type": "bitcoind"
    }
  },
  "message": "Node information retrieved successfully"
}
```

### GET /node/block-count
Get the current block count from the Bitcoin node.

**Response:**
```json
{
  "success": true,
  "data": {
    "block_count": 101
  },
  "message": "Block count retrieved successfully"
}
```

---

## Node Configuration Management

### POST /config/nodes
Create a new node configuration.

**Request Body:**
```json
{
  "name": "My Local Regtest",
  "rpc_url": "http://localhost:18443",
  "rpc_user": "bitcoin",
  "rpc_password": "password",
  "network": "regtest"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "My Local Regtest",
    "rpc_url": "http://localhost:18443",
    "rpc_user": "bitcoin",
    "rpc_password": "password",
    "network": "regtest",
    "is_active": true
  },
  "message": "Node configuration created successfully"
}
```

### GET /config/nodes
List all node configurations.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "My Local Regtest",
      "rpc_url": "http://localhost:18443",
      "rpc_user": "bitcoin",
      "rpc_password": "password",
      "network": "regtest",
      "is_active": true
    }
  ],
  "message": "Node configurations retrieved successfully"
}
```

### GET /config/nodes/:id
Get a specific node configuration by ID.

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "My Local Regtest",
    "rpc_url": "http://localhost:18443",
    "rpc_user": "bitcoin",
    "rpc_password": "password",
    "network": "regtest",
    "is_active": true
  },
  "message": "Node configuration retrieved successfully"
}
```

### GET /config/nodes/active
Get the currently active node configuration.

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "My Local Regtest",
    "rpc_url": "http://localhost:18443",
    "rpc_user": "bitcoin",
    "rpc_password": "password",
    "network": "regtest",
    "is_active": true
  },
  "message": "Active node configuration retrieved successfully"
}
```

### PUT /config/nodes/:id/activate
Set a node configuration as active (deactivates all others).

**Response:**
```json
{
  "success": true,
  "data": null,
  "message": "Node configuration activated successfully"
}
```

### DELETE /config/nodes/:id
Delete a node configuration.

**Response:**
```json
{
  "success": true,
  "data": null,
  "message": "Node configuration deleted successfully"
}
```

### POST /config/nodes/test
Test a node connection without saving the configuration.

**Request Body:**
```json
{
  "rpc_url": "http://localhost:18443",
  "rpc_user": "bitcoin",
  "rpc_password": "password"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "success": true,
    "message": "Connection successful"
  },
  "message": "Connection test completed"
}
```

---

## Error Responses

All error responses follow this format:

```json
{
  "success": false,
  "error": {
    "type": "not_found",
    "message": "Node configuration with id abc123 not found"
  }
}
```

**Error Types:**
- `bitcoin_rpc_connection_error` - Failed to connect to Bitcoin node
- `bitcoin_rpc_error` - Bitcoin RPC returned an error
- `bitcoin_rpc_parse_error` - Failed to parse Bitcoin RPC response
- `bitcoin_rpc_no_result` - Bitcoin RPC returned no result
- `config_error` - Configuration error
- `internal_server_error` - Internal server error
- `not_found` - Resource not found
- `database_error` - Database operation failed

---

## Testing with cURL

### Create a node configuration:
```bash
curl -X POST http://localhost:3000/api/config/nodes \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Local Regtest",
    "rpc_url": "http://localhost:18443",
    "rpc_user": "bitcoin",
    "rpc_password": "password",
    "network": "regtest"
  }'
```

### List all configurations:
```bash
curl http://localhost:3000/api/config/nodes
```

### Get active configuration:
```bash
curl http://localhost:3000/api/config/nodes/active
```

### Test connection:
```bash
curl -X POST http://localhost:3000/api/config/nodes/test \
  -H "Content-Type: application/json" \
  -d '{
    "rpc_url": "http://localhost:18443",
    "rpc_user": "bitcoin",
    "rpc_password": "password"
  }'
```

### Get node info:
```bash
curl http://localhost:3000/api/node/info
```

### Get block count:
```bash
curl http://localhost:3000/api/node/block-count
```
