# The Privacy Work Economy

A Technical and Economic Specification for NONOS

---

## Abstract

This document describes the economic model that governs compensation within the NONOS privacy infrastructure. The system distributes service fees among three classes of participants: infrastructure operators who run privacy-enhancing services, early supporters who funded initial development, and token stakers who provide economic security. All distributions derive from actual service usage rather than token emissions, eliminating inflationary pressure on the token supply.

The model operates on the Sepolia testnet during its validation phase, with production deployment planned for Ethereum mainnet following security review.

---

## Table of Contents

1. Introduction
2. System Architecture
3. The Economic Model
4. Participant Classes and Incentives
5. Work Measurement and Verification
6. Smart Contract Infrastructure
7. Testnet Deployment Strategy
8. Security Considerations
9. Sustainability Analysis
10. Technical Appendices

---

## 1. Introduction

### 1.1 Background

Privacy on the internet requires infrastructure. Encrypted routing, anonymous credentials, tracker blocking, fingerprint obfuscation—these services consume computational resources and demand operational expertise. The fundamental challenge lies in compensating those who provide this infrastructure without creating economic distortions that undermine the system's long-term viability.

Traditional approaches fall into two categories. Emission-based systems mint new tokens on a schedule, paying participants regardless of actual usage. This creates perpetual inflation and disconnects rewards from value creation. Fee-based systems distribute only what users pay, ensuring economic honesty but struggling during bootstrap phases when usage remains low.

The Privacy Work Economy adopts a strictly fee-based approach. No tokens are minted after initial distribution. All participant compensation derives from service fees paid by users. This design accepts the bootstrap challenge in exchange for sustainable economics once the network achieves sufficient adoption.

### 1.2 Scope

This specification covers:

- The three-way fee distribution mechanism
- Work verification methods for privacy services
- Staking requirements and multipliers
- Smart contract addresses and interactions
- Testnet validation strategy

This specification does not cover:

- The Nym mixnet protocol (NONOS integrates with but does not operate Nym infrastructure)
- Browser implementation details
- Cryptographic primitive specifications

### 1.3 Terminology

**NONOS Daemon**: Background service that provides privacy-enhancing capabilities including credential issuance, stealth address scanning, cache mixing, and tracker blocking.

**Node Operator**: An individual or organisation running a NONOS daemon and registered with the smart contract system.

**ZeroState Pass**: Non-fungible tokens issued to early supporters, entitling holders to a share of service fees conditional on token staking.

**NOX**: The ERC-20 token used for fee payment, staking collateral, and reward distribution.

**Epoch**: A 24-hour period during which work is measured and rewards are calculated.

---

## 2. System Architecture

### 2.1 Component Overview

The NONOS ecosystem comprises three primary components:

**NONOS Browser**
A desktop application providing private web browsing. The browser routes traffic through the Nym mixnet and connects to NONOS daemons for additional privacy services. Users interact exclusively with the browser; daemon selection and service invocation happen transparently.

**NONOS Daemon**
A headless service that operators deploy on servers or virtual machines. The daemon provides:

- Zero-knowledge credential issuance and verification
- Stealth address generation and scanning
- Cache mixing and decoy traffic injection
- Browser fingerprint normalisation
- Tracker detection and blocking
- Private information retrieval
- Distributed cookie storage with secret sharing

Multiple daemons form a peer-to-peer network, coordinating service delivery and participating in work verification.

**Smart Contract Layer**
Ethereum contracts that govern:

- Fee collection and distribution
- Operator registration and staking
- ZeroState Pass reward allocation
- Token staking with time-lock multipliers

### 2.2 Traffic Flow

When a user browses through NONOS:

1. The browser encrypts the request and routes it through Nym's five-hop mixnet
2. Traffic exits through a Nym gateway and reaches the destination
3. Privacy services (credential verification, cache mixing, etc.) are provided by NONOS daemons
4. Fees are collected and distributed through smart contracts

The Nym mixnet provides network-layer anonymity. NONOS daemons provide application-layer privacy enhancements. These are complementary systems with distinct operators and economics.

### 2.3 Design Principles

**Fee-funded only**: All rewards derive from service usage. No inflation schedule exists.

**Verifiable work**: Operators must demonstrate service provision through measurable outputs and peer attestation.

**Aligned incentives**: Each participant class benefits from network growth and quality.

**Transparent allocation**: Distribution percentages are fixed in immutable contracts.

---

## 3. The Economic Model

### 3.1 Fee Collection

Users pay for privacy services in ETH or NOX. Fees are collected per-session or per-operation depending on service type.

When paid in ETH, the FeeSwap contract converts payment to NOX using Uniswap liquidity pools. This conversion happens atomically within the same transaction, ensuring all internal accounting uses a single denomination.

### 3.2 Distribution Ratios

Collected fees distribute according to fixed percentages:

| Recipient | Percentage | Rationale |
|-----------|------------|-----------|
| Node Operators | 50% | Cover infrastructure costs and incentivise quality service |
| ZeroState Pass Holders | 35% | Compensate early supporters who funded development |
| Token Stakers | 15% | Reward liquidity provision and economic commitment |

These percentages are encoded in the PrivacyWorkEconomy contract and cannot be modified after deployment.

### 3.3 Distribution Mechanics

Fee distribution occurs at epoch boundaries (every 24 hours):

1. The contract calculates total fees collected during the epoch
2. Fifty percent is allocated to NodeOperatorRewards based on work scores
3. Thirty-five percent is allocated to ZeroStateStaking based on qualifying NFT holdings
4. Fifteen percent is allocated to NOXStakingPool based on stake-weighted shares

Participants claim rewards through contract interactions. Unclaimed rewards accumulate without expiration.

---

## 4. Participant Classes and Incentives

### 4.1 Node Operators

Node operators run NONOS daemons that provide privacy services to browser users.

**Requirements**:
- Stake a minimum of 1,000 NOX as collateral
- Maintain daemon availability above 95% measured uptime
- Respond correctly to peer verification challenges
- Register operator identity with NodeOperatorRewards contract

**Compensation**:
Each operator receives a share of the 50% allocation proportional to their work score relative to all operators:

```
operator_reward = (operator_work_score / total_work_score) × epoch_operator_pool
```

Work scores combine multiple factors (detailed in Section 5).

**Economic Rationale**:
Operators bear real costs: server rental, bandwidth, electricity, and operational time. The 50% allocation ensures that operating a daemon remains profitable under reasonable demand scenarios. Higher allocations would leave insufficient funds for ecosystem development; lower allocations would fail to cover operational costs at scale.

### 4.2 ZeroState Pass Holders

ZeroState Passes are 374 non-fungible tokens minted on Ethereum mainnet. No additional passes can be created.

**Background**:
These passes were distributed to early contributors who funded NONOS development before any product existed. Their capital enabled the project to reach its current state.

**Requirements**:
- Hold a ZeroState Pass in a registered wallet
- Stake 1,000 NOX per pass held
- Maintain stake throughout the claiming period

**Compensation**:
Each staked pass receives an equal share of the 35% allocation:

```
pass_reward = (35% of epoch_fees) / number_of_staked_passes
```

Passes without staked NOX receive nothing.

**Economic Rationale**:
Early-stage funding carries substantial risk. Contributors had no guarantee the project would succeed. The 35% allocation compensates this risk while requiring ongoing commitment through the staking requirement.

The staking requirement serves two purposes: it prevents purely passive income extraction, and it aligns pass holders with network health (their staked NOX loses value if the network fails).

The fixed supply of 374 passes reflects the actual number of early contributors. This is a historical fact, not an arbitrary selection.

### 4.3 Token Stakers

Any holder of NOX tokens can stake them to earn a share of service fees.

**Staking Tiers**:

| Lock Duration | Multiplier | Effective Weight |
|---------------|------------|------------------|
| 14 days | 1.0× | Base |
| 30 days | 1.25× | +25% |
| 90 days | 1.6× | +60% |
| 180 days | 2.0× | +100% |
| 365 days | 2.5× | +150% |

**Compensation**:
Each staker receives a share of the 15% allocation proportional to their weighted stake:

```
staker_reward = (staker_weighted_stake / total_weighted_stake) × epoch_staker_pool
```

Where `weighted_stake = staked_amount × tier_multiplier`.

**Economic Rationale**:
Stakers provide economic security and reduce token velocity, both of which support network stability. The 15% allocation reflects their lower operational burden compared to node operators. Time-lock multipliers reward long-term commitment, reducing speculative behaviour.

---

## 5. Work Measurement and Verification

### 5.1 The Verification Challenge

Privacy services create an inherent tension: demonstrating useful work without compromising user privacy. An operator claiming to have processed 10,000 credential verifications cannot simply reveal those credentials as proof.

NONOS addresses this through service-specific metrics, peer attestation, and cryptographic commitments.

### 5.2 Service Metrics

Each privacy service generates measurable outputs:

**Zero-Knowledge Credentials**
- Count of credentials issued (without revealing recipient identity)
- Count of verification proofs processed
- Merkle tree update transactions submitted on-chain

**Stealth Address Services**
- Blockchain range scanned for stealth payments
- Number of payments indexed and served to clients
- Response latency for address queries

**Cache Mixing**
- Decoy traffic volume injected
- Cache hit ratio
- Request mixing entropy

**Fingerprint Normalisation**
- Requests processed through normalisation
- Attribute modification count

**Tracker Blocking**
- Blocked request count
- Blocklist update frequency

**Private Information Retrieval**
- Query volume processed
- Response correctness (verifiable through PIR protocol properties)

### 5.3 Peer Attestation

Operators periodically challenge each other to verify service availability and correctness.

**Challenge Protocol**:
1. Operator A selects operator B at random from the registered set
2. A issues a test request (credential verification, PIR query, etc.)
3. B responds within the timeout window
4. A records success or failure with timestamp
5. Attestations are aggregated into daily scores

**Collusion Resistance**:
- Challenger selection uses verifiable random functions, preventing prediction
- Repeated mutual attestation between the same pair triggers investigation
- Attestation patterns inconsistent with client-reported metrics flag anomalies

### 5.4 Quality Oracle

Each daemon runs a quality oracle component that:

- Measures peer response times
- Tracks service availability
- Detects protocol violations
- Reports scores to the peer-to-peer network

Aggregate oracle scores influence operator work calculations. Consistently poor-quality operators receive reduced reward shares.

### 5.5 Work Score Calculation

An operator's epoch work score combines:

| Factor | Weight | Measurement |
|--------|--------|-------------|
| Service volume | 40% | Aggregate metrics across all service types |
| Uptime | 25% | Successful responses to peer health checks |
| Quality rating | 20% | Average quality oracle score from peers |
| Challenge responses | 15% | Correct responses to verification challenges |

Scores normalise across all operators to determine reward shares.

---

## 6. Smart Contract Infrastructure

### 6.1 Contract Overview

| Contract | Purpose |
|----------|---------|
| NOX Token | ERC-20 token for fees, staking, and rewards |
| PrivacyWorkEconomy | Fee collection and three-way distribution |
| NodeOperatorRewards | Operator registration, staking, and reward claims |
| ZeroStateStaking | NFT holder staking and reward distribution |
| NOXStakingPool | General staking with time-lock tiers |
| FeeSwap | ETH to NOX conversion via Uniswap |

### 6.2 Deployed Addresses

**Sepolia Testnet**:

| Contract | Address |
|----------|---------|
| NOX Token | `0xC87799c4517Dcdfc65bfefa3Be64Beb89668c66c` |
| PrivacyWorkEconomy | `0xAf8018e21Eff6F21BE305941f6d595Bd337c8bCA` |
| NodeOperatorRewards | `0x8cF7E025DDe69dA239392e54f5D344b434A62ba7` |
| ZeroStateStaking | `0xe6fD264976bcB896165525a8250908e824Fc9BD8` |
| NOXStakingPool | `0xb27DAe7EbE628ebe2E9302D7D2C71eF34Dd01705` |
| FeeSwap | `0x98AE9CBE5D5C2Bee33F6Ba6771C8a6677E102dE6` |

**Ethereum Mainnet**:

| Contract | Address |
|----------|---------|
| NOX Token | `0x0a26c80Be4E060e688d7C23aDdB92cBb5D2C9eCA` |
| ZeroState Pass | `0x7b575DD8e8b111c52Ab1e872924d4Efd4DF403df` |

Economy contracts will deploy to mainnet following testnet validation.

### 6.3 Key Contract Functions

**PrivacyWorkEconomy**:
- `collectFee(uint256 amount)`: Accept fee payment in NOX
- `collectFeeETH()`: Accept fee payment in ETH (converts to NOX)
- `distributeEpoch()`: Trigger epoch distribution (callable by anyone)

**NodeOperatorRewards**:
- `registerOperator(bytes32 identity)`: Register as node operator
- `stake(uint256 amount)`: Stake NOX as operator collateral
- `submitWorkProof(bytes proof)`: Submit epoch work evidence
- `claimRewards()`: Withdraw accumulated rewards

**ZeroStateStaking**:
- `stakeForPass(uint256 tokenId, uint256 amount)`: Stake NOX for a pass
- `unstake(uint256 tokenId)`: Remove stake and forfeit rewards
- `claimRewards(uint256 tokenId)`: Withdraw accumulated rewards

**NOXStakingPool**:
- `stake(uint256 amount, uint8 tier)`: Stake with selected lock duration
- `unstake(uint256 positionId)`: Withdraw after lock expires
- `claimRewards()`: Withdraw accumulated rewards

---

## 7. Testnet Deployment Strategy

### 7.1 Why Sepolia

Production smart contracts handle real funds. Deploying untested code to mainnet risks permanent loss. Sepolia provides:

**Identical execution environment**: Same EVM, same Solidity semantics, same tool compatibility.

**Free test tokens**: Faucets distribute testnet ETH for gas, enabling broad testing participation.

**Realistic conditions**: Network latency, block times, and mempool behaviour approximate mainnet.

**Safe failure**: Bugs lose worthless test tokens, not user funds.

### 7.2 Validation Objectives

The testnet phase validates:

**Contract correctness**: Fee distribution matches specified percentages. Staking mechanics function as designed. Edge cases (zero fees, single operator, maximum stake) behave correctly.

**Economic dynamics**: Reward distribution across varying operator counts. Tier multiplier calculations. Epoch boundary handling.

**Integration behaviour**: Browser-to-daemon-to-contract flow. Fee collection under load. Concurrent claim transactions.

**Security properties**: Access control enforcement. Reentrancy protection. Integer overflow handling.

### 7.3 Graduation Criteria

Mainnet deployment requires:

1. Minimum 30 days of continuous testnet operation without critical bugs
2. Third-party security audit with all findings addressed
3. Load testing demonstrating stability under 10× projected usage
4. Community review of contract source code
5. Multi-signature deployment with key ceremony

---

## 8. Security Considerations

### 8.1 Attack Scenarios

**Sybil operator attack**: An adversary registers many operators to capture disproportionate rewards.

*Mitigation*: Each operator requires 1,000 NOX stake. Creating 100 fake operators requires 100,000 NOX capital at risk. Additionally, fake operators lacking genuine service traffic receive minimal work scores regardless of count.

**Work inflation**: An operator falsifies service metrics to claim higher rewards.

*Mitigation*: Peer attestation cross-validates claimed work. Metrics inconsistent with client-reported usage or peer observations trigger investigation. Persistent inflation results in stake slashing.

**ZeroState concentration**: A single entity acquires all ZeroState Passes.

*Mitigation*: Even with all 374 passes, the holder captures only 35% of fees and must stake 374,000 NOX. The majority of fees still flow to operators. Additionally, this attack requires the holder to want the network to succeed, since their capital is at risk.

**Oracle manipulation**: Corrupted quality oracle reports false scores.

*Mitigation*: Each daemon runs its own oracle. Aggregate scores resist individual corruption. Anomalous scoring patterns from specific peers result in their reports being discounted.

### 8.2 Economic Failure Modes

**Insufficient demand**: If users do not pay for services, fees remain zero and no rewards distribute.

*Response*: The system fails gracefully. Operators experiencing losses will cease operations. The network contracts to equilibrium where remaining operators cover costs. No inflation mechanism masks low demand.

**Token price collapse**: If NOX loses value, staking requirements become trivially cheap.

*Response*: This reduces economic security but does not break functionality. Governance may propose adjusting stake requirements if validated participants approve.

### 8.3 Contract Security

All contracts follow established security practices:

- OpenZeppelin library usage for standard patterns
- Checks-effects-interactions ordering
- Reentrancy guards on external calls
- SafeMath for arithmetic operations
- Access control through role-based permissions
- Event emission for off-chain monitoring

---

## 9. Sustainability Analysis

### 9.1 Operating Cost Model

A typical NONOS daemon operator incurs:

| Cost Category | Monthly Estimate |
|---------------|------------------|
| Server rental (VPS) | $30-80 |
| Bandwidth | $10-50 |
| Operational time | Variable |
| Stake opportunity cost | Variable |

Total direct costs range from $40-150 monthly depending on scale and location.

### 9.2 Minimum Viable Fee Volume

For the network to sustain N operators at average cost C:

```
Required monthly fees = (N × C) / 0.50
```

With 100 operators averaging $60 monthly cost:
```
Required fees = (100 × $60) / 0.50 = $12,000/month
```

This represents the threshold below which operators begin leaving due to losses.

### 9.3 Equilibrium Dynamics

Below minimum viable fees:
- Marginal operators (highest cost, lowest efficiency) exit
- Remaining operators receive larger individual shares
- Exit continues until remaining operators are profitable

Above minimum viable fees:
- Existing operators profit
- New operators attracted by returns
- Entry continues until marginal operator breaks even

This self-regulating mechanism maintains network operation across varying demand levels, though at correspondingly varying scale.

### 9.4 Long-term Outlook

The system makes no guarantees about returns. Unlike emission-based systems that promise stakers yield regardless of usage, the Privacy Work Economy pays only from actual fees. If privacy services lack demand, participants earn nothing.

This design prioritises economic honesty over growth incentives. It assumes that genuine utility will generate sufficient demand, and that artificial incentives create unsustainable obligations.

---

## 10. Technical Appendices

### Appendix A: Contract Source Verification

All deployed contracts are verified on Etherscan. Source code is available at:

- Sepolia: https://sepolia.etherscan.io/address/[contract_address]#code
- Mainnet: https://etherscan.io/address/[contract_address]#code

### Appendix B: Staking Tier Parameters

| Tier ID | Duration (days) | Multiplier | Early Exit Penalty |
|---------|-----------------|------------|-------------------|
| 0 | 14 | 1.0× | 0% |
| 1 | 30 | 1.25× | 10% |
| 2 | 90 | 1.6× | 20% |
| 3 | 180 | 2.0× | 30% |
| 4 | 365 | 2.5× | 50% |

Early exit before lock expiration incurs the listed penalty on staked principal.

### Appendix C: Epoch Parameters

| Parameter | Value |
|-----------|-------|
| Epoch duration | 86,400 seconds (24 hours) |
| Epoch start reference | Unix timestamp 0 (midnight UTC, 1 January 1970) |
| Distribution delay | 1 hour after epoch end |
| Claim expiration | None (rewards accumulate indefinitely) |

### Appendix D: Operator Registration

Operators register by calling `registerOperator` with:

- `identity`: 32-byte commitment to operator public key
- Transaction must include minimum stake (1,000 NOX)
- Registration emits `OperatorRegistered` event

Deregistration requires:
- No pending slashing conditions
- 7-day cooldown period
- Stake returned after cooldown

### Appendix E: Integration Points

**Daemon to Contract**:
- Work proof submission at epoch boundaries
- Reward claim transactions
- Quality oracle score publication

**Browser to Daemon**:
- Service requests via local RPC (port 8420)
- Fee payment authorisation
- Session credential requests

**Contract to Contract**:
- PrivacyWorkEconomy calls distribution functions on child contracts
- FeeSwap interacts with Uniswap router
- ZeroStateStaking queries NFT ownership from pass contract

---

## Document Information

**Version**: 1.0
**Status**: Draft for Review
**Last Updated**: March 2026
**Authors**: NONOS Technical Team

---

*This document describes the intended behaviour of the NONOS economic system. Implementation details may differ. Consult deployed contract source code for authoritative specifications.*
