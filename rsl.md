# RSL: Relational Schema Language

Define a schema as easily and portably as possible. Trying to reinvent the SQL wheel, with modern considerations.
Takes heavy inspiration from `SQL` and `Zod` (the `js` library).

## Schemas

analogous to `CREATE TABLE` in sql

```
schema User {
  id: u64,
  first_name: utf8,
}
```

### datatype modifiers

optionnal: `datatype?`
: prepends 1 bit to repr to show if initialized or not

vector:
: notation: `datatype[N]`
: - where `N` is the allocated length.
: if inner datatype is optionnal, the repr is prepended by a `N`-long array of bits to determine the presence or not
: of the elements in the array.

array:
: notation: `datatype[]`
: necessarily unsized

they can be chained: `datatype?[][3]` means optionnally-initialized array of 3 values vector

### Validators

the idea of specifying the possible values for a field, structurally.

```rsl
...
  ip: ipv4,
  age: min(5) max(20),
  age2: inrange(5..=20),
  name: utf8,
...
```

### Constraints

the idea of constraining a field with the database

```

```

## a fully fledged exemple

```rsl

```
