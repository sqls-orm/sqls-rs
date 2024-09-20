### TODO:
- sqlx::query_as integration
- raw str support for all queries
- separation to traits, to restrict all chain methods access for a query
- ```rust
    /// is problematic, since scalar version implements .attr() method
    Optional::default()
        .id(<value>)
        .attr(<value>)
  ```