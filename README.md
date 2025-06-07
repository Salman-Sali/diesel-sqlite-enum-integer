Note: This proc macro derive will only work if your enum does not contain any variant which accepts values.

### Your cargo.toml
```toml 
[dependencies]
num_enum = "*"
diesel = { version = "*", features = ["sqlite"] }
```


### Your enum 
```rust 
#[diesel_sqlite_enum_integer::enum_to_diesel_integer]
pub enum MyEnum {
    One,
    Two,
    Three
}
```


### The database entity
```rust
#[diesel(table_name = crate::schema::my_entity)]
#[derive(Debug, Queryable, Selectable, Identifiable, Associations, PartialEq, Insertable)]
pub struct MyEntity {
    pub id: i32,
    pub my_enum: MyEnum
}
```

### The schema
```rust
diesel::table! {
    my_entity (id) {
        id -> Integer,
        my_enum -> Integer
    }
}
```