window.BENCHMARK_DATA = {
  "lastUpdate": 1639776749463,
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
        "date": 1639776748847,
        "tool": "cargo",
        "benches": [
          {
            "name": "search//field/other_field",
            "value": 30389,
            "range": "± 1584",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/*/other_field).",
            "value": 60791,
            "range": "± 3417",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/1/other_field).",
            "value": 32313,
            "range": "± 4218",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/1",
            "value": 18414,
            "range": "± 461",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/*/regex",
            "value": 99127,
            "range": "± 7933",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}