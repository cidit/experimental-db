# RSL: Relational Schema Language

Define a schema as easily and portably as possible.

## Schemas

analogous to `CREATE TABLE` in sql

```
schema User {
  id: u64,
  first_name: utf8,
}
```

### datatype modifiers

array: `datatype[]`
`N` sized vector: `datatype[N]`
optionnal: `datatype?`

they can be chained: `datatype?[][3]` means optionnally-initialized array of 3 values vector

### Validators

the idea of creating a specified type

```
utf8 = u8
```

### Constraints

the idea of constraining a field with the database

```

```
