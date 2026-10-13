# User guide

This guide walks you through PortalEnv, from a first configuration struct to
real-world setups with multiple environments. Read the chapters in order if you
are new to PortalEnv, or jump to the one that matches what you want to do.

For the complete syntax of the `#[env_portal(...)]` attribute, see the
[`EnvPortal`](crate::EnvPortal) derive macro reference.

## Chapters

1. [Getting started](crate::docs::getting_started) — define a configuration struct and load it from a `.env` file.
2. [Mapping values](crate::docs::mapping) — the kinds of values a `mapping` can contain.
3. [Environments](crate::docs::environments) — switch values per environment such as `local`, `stg` and `prd`.
4. [Composition](crate::docs::composition) — split configuration into parts and combine them.
5. [Value types](crate::docs::value_types) — how strings are converted into Rust types, including your own.
6. [Errors](crate::docs::errors) — what can go wrong and how to fix it.
7. [Recipes](crate::docs::recipes) — practical patterns for testing and deployment.
