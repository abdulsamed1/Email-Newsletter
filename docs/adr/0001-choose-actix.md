# ADR 0001 — Select Actix-web as the HTTP framework

Date: 2025-09-07  
Status: Accepted  
Authors: Engineering Team

## Context
The project requires a production-ready HTTP framework to implement RESTful APIs for the Email Newsletter service. Primary requirements are: high performance, stable async/await support, broad ecosystem (middleware, routing, testing), clear upgrade path, and active community support. Candidate frameworks considered: Actix-web, Rocket, Warp, Axum.

## Decision
Actix-web is selected as the project's HTTP framework.

## Rationale
- Mature and widely adopted in production Rust backends.
- Strong ecosystem of middleware, integrations, and community-contributed crates.
- Proven performance characteristics for high-concurrency workloads.
- Stable async support and interoperability with common runtime patterns.
- Good fit for current team constraints and delivery schedule.

## Considered alternatives
- Rocket — ergonomic API; historically slower to adopt async/stable features and had more boundary on ecosystem choices.
- Warp — composable and type-safe; smaller ecosystem and different ergonomics.
- Axum — modern, tokio-native design; strong candidate but assessed as less mature in ecosystem breadth at decision time.

## Consequences
- Positive:
    - Faster delivery due to available libraries and examples.
    - Suitable for high-throughput endpoints.
- Negative:
    - Team members may incur a short-term learning curve for Actix idioms.
    - Some coupling to Actix ecosystem components; migration to a different framework later will require non-trivial changes.
- Mitigations:
    - Provide internal onboarding materials and examples.
    - Encapsulate framework-specific logic behind well-defined interfaces to reduce coupling.

## Next steps
- Create a minimal project template and example endpoints.
- Define coding patterns, error-handling conventions, and testing strategy for Actix-web.
- Add CI checks and dependency update policy for framework-related crates.
- Revisit this ADR if project requirements or ecosystem maturity materially change.
