{
  "servers": {
    "legacy-metrics": {
      "type": "http",
      "url": "http://metrics.internal.example.com/mcp",
      "headers": {
        "Authorization": "Bearer ${METRICS_TOKEN}"
      }
    }
  }
}
