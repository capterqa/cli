<p align="center">
  <a href="https://docs.capter.io">
    <img src="media/icon.png" alt="Capter logo" width="128" height="128">
  </a>
</p>

# Capter CLI

![test-build](https://github.com/capterqa/cli/workflows/test-build/badge.svg)
[![codecov](https://codecov.io/gh/capterqa/cli/branch/alpha/graph/badge.svg?token=DAUCAH1MWW)](https://codecov.io/gh/capterqa/cli)

Capter is a lightweight **end-to-end** testing tool for APIs. It's language agnostic and can test APIs written in any language (Node.js, Go etc).

- 🧑‍💻 Write tests in YAML
- 🔎 Run the same tests locally, in CI, or as a cron job to monitor your live APIs
- 🏃‍♂️ Takes **less than a minute** to get started

## How it works:

Create your workflows in a folder called `.capter/`:

```yaml
# .capter/products.yml

name: products
steps:
  - name: fetch all products
    id: products
    # use the URL environment variable to decide where to run the test
    url: ${{ env.URL }}/api/products
    assertions:
      - !assert status equal 200
      - !assert body isArray

  - name: fetch first product
    # use the previous response to fetch the first product
    url: ${{ env.URL }}/api/posts/${{ products.response.body.0.id }}
    assertions:
      - !assert body.id equal ${{ products.response.body.0.id }}
```

Then run the CLI:

![CLI](media/demo.gif)

## Getting started

Follow the instructions in the documentation to get started:

- [Installation](https://docs.capter.io/docs/installation)
- [Gettings started](https://docs.capter.io/docs/getting-started)
