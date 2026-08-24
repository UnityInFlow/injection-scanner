# Storage layer

<!-- Would-be PI048: `[A-Za-z0-9+\/]{48,}` treats `/` as a base64 character, so
     every path below matched as a "long base64 blob". 3,494 hits across this
     ecosystem's docs; the single largest false-positive source ever measured
     against this scanner. -->

The span writer lives at
`backend/src/main/kotlin/io/github/unityinflow/agenttracer/storage/SpanRepository.kt`
and is wired in by
`backend/src/main/kotlin/io/github/unityinflow/agenttracer/config/StorageConfiguration.kt`.

Batching is handled in
`backend/src/main/kotlin/io/github/unityinflow/agenttracer/ingest/DecodingBatchProcessor.kt`,
which reads from the queue defined in
`backend/src/main/kotlin/io/github/unityinflow/agenttracer/ingest/IngestQueueProperties.kt`.

Tests mirror the layout under
`backend/src/test/kotlin/io/github/unityinflow/agenttracer/storage/SpanRepositoryTest.kt`.

The generated OpenAPI client ends up in
`frontend/src/generated/api/services/AgentTracerSpanQueryService.ts`.
