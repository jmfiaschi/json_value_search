window.BENCHMARK_DATA = {
  "lastUpdate": 1639778238099,
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
      },
      {
        "commit": {
          "author": {
            "email": "jm.fiaschi@gmail.com",
            "name": "jm.fiaschi",
            "username": "jmfiaschi"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "80ced300c91550d640f9e3aa25fb911281100a49",
          "message": "fix(cicd): automatize cargo version update (#1)",
          "timestamp": "2021-12-17T22:53:50+01:00",
          "tree_id": "f46bfbfa3b9cd6d2bb527cea445cf4fdb2d6cc2b",
          "url": "https://github.com/jmfiaschi/json_value_search/commit/80ced300c91550d640f9e3aa25fb911281100a49"
        },
        "date": 1639778237116,
        "tool": "cargo",
        "benches": [
          {
            "name": "search//field/other_field",
            "value": 29577,
            "range": "± 1742",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/*/other_field).",
            "value": 58012,
            "range": "± 3563",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/1/other_field).",
            "value": 30835,
            "range": "± 1522",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/1",
            "value": 18740,
            "range": "± 783",
            "unit": "ns/iter"
          },
          {
            "name": "search//field/*/regex",
            "value": 95857,
            "range": "± 3962",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}