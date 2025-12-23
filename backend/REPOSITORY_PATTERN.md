# Repository Pattern Guide

## Overview
The application now uses a database pool pattern instead of storing individual repository instances in `AppState`. This makes it easy to add new repositories without modifying the app state.

## How It Works

### AppState Structure
```rust
pub struct AppState {
    pub bitcoin: Arc<BitcoinRpc>,
    pub db_pool: SqlitePool,  // Shared database pool
}
```

### Creating Repositories On-Demand
Instead of storing repository instances, create them from the pool when needed:

```rust
async fn my_handler(State(state): State<Arc<AppState>>) -> Result<...> {
    // Create repository from pool
    let repo = SqliteNodeConfigRepository::new(state.db_pool.clone());

    // Use the repository
    let data = repo.some_method().await?;

    Ok(...)
}
```

## Adding a New Repository

### Step 1: Define the trait in `src/db/traits.rs`
```rust
pub trait MyNewRepository: Send + Sync {
    async fn create(&self, data: MyData) -> Result<MyData, DbError>;
    async fn get(&self, id: &str) -> Result<Option<MyData>, DbError>;
    // ... other methods
}
```

### Step 2: Implement for SQLite in `src/db/sqlite/my_new_repo.rs`
```rust
pub struct SqliteMyNewRepository {
    pool: SqlitePool,
}

impl SqliteMyNewRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MyNewRepository for SqliteMyNewRepository {
    // Implement trait methods using self.pool
}
```

### Step 3: Add table creation in `src/state/app_state.rs`
```rust
async fn setup_database(pool: &SqlitePool) {
    // Existing tables...

    // Add new table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS my_new_table (
            id TEXT PRIMARY KEY,
            -- other columns
        )
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create my_new_table");
}
```

### Step 4: Use in route handlers
```rust
async fn my_handler(State(state): State<Arc<AppState>>) -> Result<...> {
    let repo = SqliteMyNewRepository::new(state.db_pool.clone());
    let data = repo.get("id").await?;
    Ok(...)
}
```

## Benefits

1. **No AppState changes needed** - Just create repos from the pool when needed
2. **Single source of truth** - One database pool for all repositories
3. **Easy to test** - Can mock the pool or create test-specific repos
4. **Flexible** - Can create multiple repo instances if needed
5. **Resource efficient** - SQLite pool manages connections efficiently

## Migration from Old Pattern

### Before (tightly coupled):
```rust
pub struct AppState {
    pub node_repo: Arc<dyn NodeConfigRepository>,
    pub user_repo: Arc<dyn UserRepository>,
    // Need to add new field for each repo!
}
```

### After (loosely coupled):
```rust
pub struct AppState {
    pub db_pool: SqlitePool,
    // No need to change AppState when adding repos!
}

// Create repos as needed
let node_repo = SqliteNodeConfigRepository::new(state.db_pool.clone());
let user_repo = SqliteUserRepository::new(state.db_pool.clone());
```
