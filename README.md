# rustysql
Implement sqlite in rust

### Following are Codd’s Twelve Principles of Relational Databases:

- [ ] Information is represented logically in tables.
- [ ] Data must be logically accessible by table, primary key, and column.
- [ ] Null values must be uniformly treated as _missing information_ not as empty
strings, blanks, or zeros.
- [ ] Metadata (data about the database) must be stored in the database just as regular data is.
- [ ] A single language must be able to define data, views, integrity constraints, authorization, transactions, and data manipulation.
- [ ] Views must show the updates of their base tables and vice versa.
- [ ] A single operation must be available to do each of the following operations:
retrieve data, insert data, update data, or delete data.
- [ ] Batch and end-user operations are logically separate from physical storage and access methods.
- [ ] Batch and end-user operations can change the database schema without having to recreate it or the applications built upon it.
- [ ] Integrity constraints must be available and stored in the metadata, not in an application program.
- [ ] The data manipulation language of the relational system should not care where or how the physical data is distributed and should not require alteration if the physical data is centralized or distributed.
- [ ] Any row processing done in the system must obey the same integrity rules and constraints that set-processing operations do.
