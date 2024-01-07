# ERC4626 SP Subgraph

This subgraph tracks the ERC4626 token contracts and their transfers. Based on that it calculates the total assets
deposited in each ERC4626 vault.

### ERC4626 standard

```
https://ethereum.org/pl/developers/docs/standards/tokens/erc-4626/
```

### Motivation

DeFi is a fast growing ecosystem. It is important to have a reliable source of data about the assets locked in the
environment. As more and more platform use ERC4626 standard it is a good opportunity to create a subgraph that will
track the assets locked in the vaults.

### TODO

- [x] Index all the ERC4626 compliant contracts
- [ ] Index all the transfers
- [ ] Calculate the total assets locked in the vaults