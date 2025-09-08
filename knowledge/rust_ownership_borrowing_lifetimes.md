# Rust Ownership, Borrowing, and Lifetimes

This document summarizes the core concepts of Ownership, Borrowing, and Lifetimes in Rust, as depicted in the provided image.

## Ownership

1.  Every value is 'owned' by a single variable, struct, vector, etc. at a time.
2.  Reassigning the value to another variable, passing it to a function, or putting it into a vector moves the value. The old variable can't be used anymore!

## Borrowing

3.  You can create many read-only references to a value that exist at the same time.
4.  You can't move a value while a reference to the value exists.
5.  You can make a writable (mutable) reference to a value only if there are no read-only references currently in use. One mutable reference to a value can exist at a time.
6.  You can't mutate a value through the owner when any reference (mutable or immutable) to the value exists.
7.  Some types of values are copied instead of moved (numbers, booleans, characters, arrays/tuples with copyable elements).

## Lifetimes

8.  When a variable goes out of scope, the value owned by it is dropped (cleaned up in memory).
9.  Values can't be dropped if there are still active references to it.
10. References to a value can't outlive the value they refer to.

## General Rules

11. These rules will dramatically change how you write code (compared to other languages).
12. When in doubt, remember that Rust wants to minimize unexpected updates to data.

## Visual Representation

```mermaid
graph TD
    A[Value] --> B{Ownership};
    B --> C[Single Owner];
    C --> D{Move Semantics};
    D --> E[Old Variable Invalidated];

    A --> F{Borrowing};
    F --> G[Multiple Read-Only References];
    F --> H{One Mutable Reference <br/> (No Read-Only Refs)};
    G & H --> I[Cannot Move Value <br/> While Ref Exists];
    I --> J[Cannot Mutate Through Owner <br/> While Any Ref Exists];

    A --> K{Lifetimes};
    K --> L[Value Dropped <br/> When Out of Scope];
    L --> M{Cannot Drop Value <br/> With Active References};
    M --> N[References Cannot <br/> Outlive Value];

    style A fill:#f9f,stroke:#333,stroke-width:2px
    style B fill:#bbf,stroke:#333,stroke-width:2px
    style F fill:#bbf,stroke:#333,stroke-width:2px
    style K fill:#bbf,stroke:#333,stroke-width:2px
```


------
Rust want to avoid unexpected updates in bindings.
Rust doesn;t want you to use value that is partially moved.