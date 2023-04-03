# DeWiCats Program Library

A collection of solana programs used for Helium's Solana integration

# [DeWiCats Program Library](https://dewicats.xyz/)

Frontend for DewiCats DAO

## Local Setup

1. Install dependencies

```
$: yarn
```

2. Build anchor dependencies

```
$: anchor run build-deps
```

3. Start localnet

```
$: TESTING=true anchor localnet
```

## DeWi-Hydra

## 👏🏽 Contributing Guidelines

We keep an updated list of bugs/feature requests in [Github Issues](https://github.com/DeWiCats/dewicats-program-library/issues).

![GitHub issues](https://github.com/DeWiCats/dewicats-program-library/issues?style=flat-square)

Filter by ["good first issue"](https://github.com/DeWiCats/dewicats-program-library/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) to get your feet wet!
Once you submit a PR, our CI will generate a temporary testing URL where you can validate your changes. Tag any of the gatekeepers on the review to merge them into master.

_**NOTE**_: For big changes associated with feature releases/milestones, they will be merged onto the `develop` branch for more thorough QA before a final merge to `main`
