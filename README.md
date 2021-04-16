# Clean Architecture

A clean architecture is a software structure that allows developer to keep his code analysable, testable, verifiable and
maintainable.

Robert C. Martin propose that architecture as a layered organization with well-defined responsibilities.

We can compare it to an onion.

More information at :

* [Clean Architecture: A Craftsman's Guide to Software Structure and Design (Robert C. Martin book)](https://www.amazon.fr/Clean-Architecture-Craftsmans-Software-Structure/dp/0134494164)
* <https://whatis.techtarget.com/definition/clean-architecture> and others

## Explanation of my clean architecture

My application is composed of 4 parts :

1. Domain : Contains all entities, errors, ...
2. Business : Contains use cases
3. Infrastructure : Contains repositories (that allows to use dependencies)
4. Application : Contains presenters, view model
