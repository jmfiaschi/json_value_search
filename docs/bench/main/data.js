window.BENCHMARK_DATA = {
  "lastUpdate": 1639776785646,
  "repoUrl": "https://github.com/jmfiaschi/json_value_search",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "jm.fiaschi@gmail.com",
            "name": "jmfiaschi",
            "username": "jmfiaschi"
          },
          "committer": {
            "email": "jm.fiaschi@gmail.com",
            "name": "jmfiaschi",
            "username": "jmfiaschi"
          },
          "distinct": true,
          "id": "fa57d5c86d5720e0a8cc6ab803d9d5e27ebf2a28",
          "message": "fix(cicd): automatize cargo version update",
          "timestamp": "2021-12-17T22:25:03+01:00",
          "tree_id": "c064123dbcc2ea054d4d40d986c063824ec6523e",
          "url": "https://github.com/jmfiaschi/json_value_search/commit/fa57d5c86d5720e0a8cc6ab803d9d5e27ebf2a28"
        },
        "date": 1639776784855,
        "tool": "cargo",
        "benches": [
          {
            "name": "search//field/other_field",
            "value": 23943,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/*/other_field).",
            "value": 48584,
            "range": "± 453",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/1/other_field).",
            "value": 25736,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/1",
            "value": 15019,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/*/regex",
            "value": 76921,
            "range": "± 584",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}