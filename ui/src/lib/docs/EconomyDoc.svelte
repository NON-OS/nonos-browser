<script lang="ts">
</script>

<article class="doc">
	<header class="doc-header">
		<div class="brand"><span class="mark">N</span><span class="wordmark">NONOS</span></div>
		<h1>The Privacy Work <span class="hl">Economy</span></h1>
		<p class="tagline">A Technical and Economic Specification for NONOS</p>
		<p class="meta">Version 1.0 · March 2026</p>
	</header>

	<section class="section">
		<h2>Abstract</h2>
		<p>This document describes the economic model that governs compensation within the NONOS privacy infrastructure. The system distributes service fees among three classes of participants: infrastructure operators who run privacy-enhancing services, early supporters who funded initial development, and token stakers who provide economic security. All distributions derive from actual service usage rather than token emissions, eliminating inflationary pressure on the token supply.</p>
		<div class="callout"><strong>Current Status:</strong> The model operates on the Sepolia testnet during its validation phase, with production deployment planned for Ethereum mainnet following security review.</div>
	</section>

	<nav class="toc">
		<h3>Table of Contents</h3>
		<ol>
			<li>Introduction</li>
			<li>System Architecture</li>
			<li>The Economic Model</li>
			<li>Participant Classes and Incentives</li>
			<li>Work Measurement and Verification</li>
			<li>Smart Contract Infrastructure</li>
			<li>Testnet Deployment Strategy</li>
			<li>Security Considerations</li>
			<li>Sustainability Analysis</li>
			<li>Technical Appendices</li>
		</ol>
	</nav>

	<section class="section">
		<h2>1. Introduction</h2>
		<h3>1.1 Background</h3>
		<p>Privacy on the internet requires infrastructure. Encrypted routing, anonymous credentials, tracker blocking, fingerprint obfuscation—these services consume computational resources and demand operational expertise. The fundamental challenge lies in compensating those who provide this infrastructure without creating economic distortions that undermine the system's long-term viability.</p>
		<p>Traditional approaches fall into two categories:</p>
		<ul>
			<li><strong>Emission-based systems</strong> mint new tokens on a schedule, paying participants regardless of actual usage. This creates perpetual inflation and disconnects rewards from value creation.</li>
			<li><strong>Fee-based systems</strong> distribute only what users pay, ensuring economic honesty but struggling during bootstrap phases when usage remains low.</li>
		</ul>
		<p>The Privacy Work Economy adopts a strictly fee-based approach. No tokens are minted after initial distribution. All participant compensation derives from service fees paid by users. This design accepts the bootstrap challenge in exchange for sustainable economics once the network achieves sufficient adoption.</p>
		<h3>1.2 Scope</h3>
		<p><strong>This specification covers:</strong></p>
		<ul>
			<li>The three-way fee distribution mechanism</li>
			<li>Work verification methods for privacy services</li>
			<li>Staking requirements and multipliers</li>
			<li>Smart contract addresses and interactions</li>
			<li>Testnet validation strategy</li>
		</ul>
		<p><strong>This specification does not cover:</strong></p>
		<ul>
			<li>The Nym mixnet protocol (NONOS integrates with but does not operate Nym infrastructure)</li>
			<li>Browser implementation details</li>
			<li>Cryptographic primitive specifications</li>
		</ul>
		<h3>1.3 Terminology</h3>
		<table>
			<tr><th>Term</th><th>Definition</th></tr>
			<tr><td>NONOS Daemon</td><td>Background service providing privacy-enhancing capabilities including credential issuance, stealth address scanning, cache mixing, and tracker blocking.</td></tr>
			<tr><td>Node Operator</td><td>An individual or organisation running a NONOS daemon and registered with the smart contract system.</td></tr>
			<tr><td>ZeroState Pass</td><td>Non-fungible tokens issued to early supporters, entitling holders to a share of service fees conditional on token staking.</td></tr>
			<tr><td>NOX</td><td>The ERC-20 token used for fee payment, staking collateral, and reward distribution.</td></tr>
			<tr><td>Epoch</td><td>A 24-hour period during which work is measured and rewards are calculated.</td></tr>
		</table>
	</section>

	<section class="section">
		<h2>2. System Architecture</h2>
		<h3>2.1 Component Overview</h3>
		<h4>NONOS Browser</h4>
		<p>A desktop application providing private web browsing. The browser routes traffic through the Nym mixnet and connects to NONOS daemons for additional privacy services. Users interact exclusively with the browser; daemon selection and service invocation happen transparently.</p>
		<h4>NONOS Daemon</h4>
		<p>A headless service that operators deploy on servers or virtual machines. The daemon provides:</p>
		<ul>
			<li>Zero-knowledge credential issuance and verification</li>
			<li>Stealth address generation and scanning</li>
			<li>Cache mixing and decoy traffic injection</li>
			<li>Browser fingerprint normalisation</li>
			<li>Tracker detection and blocking</li>
			<li>Private information retrieval</li>
			<li>Distributed cookie storage with secret sharing</li>
		</ul>
		<p>Multiple daemons form a peer-to-peer network, coordinating service delivery and participating in work verification.</p>
		<h4>Smart Contract Layer</h4>
		<p>Ethereum contracts that govern:</p>
		<ul>
			<li>Fee collection and distribution</li>
			<li>Operator registration and staking</li>
			<li>ZeroState Pass reward allocation</li>
			<li>Token staking with time-lock multipliers</li>
		</ul>
		<h3>2.2 Traffic Flow</h3>
		<p>When a user browses through NONOS:</p>
		<ol>
			<li>The browser encrypts the request and routes it through Nym's five-hop mixnet</li>
			<li>Traffic exits through a Nym gateway and reaches the destination</li>
			<li>Privacy services (credential verification, cache mixing, etc.) are provided by NONOS daemons</li>
			<li>Fees are collected and distributed through smart contracts</li>
		</ol>
		<p>The Nym mixnet provides network-layer anonymity. NONOS daemons provide application-layer privacy enhancements. These are complementary systems with distinct operators and economics.</p>
		<h3>2.3 Design Principles</h3>
		<div class="principles">
			<div class="principle"><strong>Fee-funded only</strong><p>All rewards derive from service usage. No inflation schedule exists.</p></div>
			<div class="principle"><strong>Verifiable work</strong><p>Operators must demonstrate service provision through measurable outputs and peer attestation.</p></div>
			<div class="principle"><strong>Aligned incentives</strong><p>Each participant class benefits from network growth and quality.</p></div>
			<div class="principle"><strong>Transparent allocation</strong><p>Distribution percentages are fixed in immutable contracts.</p></div>
		</div>
	</section>

	<section class="section">
		<h2>3. The Economic Model</h2>
		<h3>3.1 Fee Collection</h3>
		<p>Users pay for privacy services in ETH or NOX. Fees are collected per-session or per-operation depending on service type.</p>
		<p>When paid in ETH, the FeeSwap contract converts payment to NOX using Uniswap liquidity pools. This conversion happens atomically within the same transaction, ensuring all internal accounting uses a single denomination.</p>
		<h3>3.2 Distribution Ratios</h3>
		<div class="stats-row">
			<div class="stat"><span class="num">50%</span><span class="label">Node Operators</span></div>
			<div class="stat"><span class="num">35%</span><span class="label">ZeroState Holders</span></div>
			<div class="stat"><span class="num">15%</span><span class="label">Token Stakers</span></div>
		</div>
		<table>
			<tr><th>Recipient</th><th>Percentage</th><th>Rationale</th></tr>
			<tr><td>Node Operators</td><td class="hl">50%</td><td>Cover infrastructure costs and incentivise quality service</td></tr>
			<tr><td>ZeroState Pass Holders</td><td class="hl">35%</td><td>Compensate early supporters who funded development</td></tr>
			<tr><td>Token Stakers</td><td class="hl">15%</td><td>Reward liquidity provision and economic commitment</td></tr>
		</table>
		<p>These percentages are encoded in the PrivacyWorkEconomy contract and cannot be modified after deployment.</p>
		<h3>3.3 Distribution Mechanics</h3>
		<p>Fee distribution occurs at epoch boundaries (every 24 hours):</p>
		<ol>
			<li>The contract calculates total fees collected during the epoch</li>
			<li>Fifty percent is allocated to NodeOperatorRewards based on work scores</li>
			<li>Thirty-five percent is allocated to ZeroStateStaking based on qualifying NFT holdings</li>
			<li>Fifteen percent is allocated to NOXStakingPool based on stake-weighted shares</li>
		</ol>
		<p>Participants claim rewards through contract interactions. Unclaimed rewards accumulate without expiration.</p>
	</section>

	<section class="section">
		<h2>4. Participant Classes and Incentives</h2>
		<h3>4.1 Node Operators</h3>
		<p>Node operators run NONOS daemons that provide privacy services to browser users.</p>
		<p><strong>Requirements:</strong></p>
		<ul>
			<li>Stake a minimum of 1,000 NOX as collateral</li>
			<li>Maintain daemon availability above 95% measured uptime</li>
			<li>Respond correctly to peer verification challenges</li>
			<li>Register operator identity with NodeOperatorRewards contract</li>
		</ul>
		<p><strong>Compensation:</strong></p>
		<p>Each operator receives a share of the 50% allocation proportional to their work score relative to all operators:</p>
		<div class="formula">operator_reward = (operator_work_score / total_work_score) × epoch_operator_pool</div>
		<p><strong>Economic Rationale:</strong></p>
		<p>Operators bear real costs: server rental, bandwidth, electricity, and operational time. The 50% allocation ensures that operating a daemon remains profitable under reasonable demand scenarios.</p>

		<h3>4.2 ZeroState Pass Holders</h3>
		<p>ZeroState Passes are 374 non-fungible tokens minted on Ethereum mainnet. No additional passes can be created.</p>
		<p><strong>Background:</strong></p>
		<p>These passes were distributed to early contributors who funded NONOS development before any product existed. Their capital enabled the project to reach its current state.</p>
		<p><strong>Requirements:</strong></p>
		<ul>
			<li>Hold a ZeroState Pass in a registered wallet</li>
			<li>Stake 1,000 NOX per pass held</li>
			<li>Maintain stake throughout the claiming period</li>
		</ul>
		<p><strong>Compensation:</strong></p>
		<div class="formula">pass_reward = (35% of epoch_fees) / number_of_staked_passes</div>
		<p>Passes without staked NOX receive nothing.</p>
		<p><strong>Economic Rationale:</strong></p>
		<p>Early-stage funding carries substantial risk. The 35% allocation compensates this risk while requiring ongoing commitment through the staking requirement. The staking requirement prevents purely passive income extraction and aligns pass holders with network health.</p>
		<p>The fixed supply of 374 passes reflects the actual number of early contributors.</p>

		<h3>4.3 Token Stakers</h3>
		<p>Any holder of NOX tokens can stake them to earn a share of service fees.</p>
		<p><strong>Staking Tiers:</strong></p>
		<table>
			<tr><th>Lock Duration</th><th>Multiplier</th><th>Effective Weight</th></tr>
			<tr><td>14 days</td><td class="hl">1.0×</td><td>Base</td></tr>
			<tr><td>30 days</td><td class="hl">1.25×</td><td>+25%</td></tr>
			<tr><td>90 days</td><td class="hl">1.6×</td><td>+60%</td></tr>
			<tr><td>180 days</td><td class="hl">2.0×</td><td>+100%</td></tr>
			<tr><td>365 days</td><td class="hl">2.5×</td><td>+150%</td></tr>
		</table>
		<p><strong>Compensation:</strong></p>
		<div class="formula">staker_reward = (staker_weighted_stake / total_weighted_stake) × epoch_staker_pool</div>
		<p>Where <code>weighted_stake = staked_amount × tier_multiplier</code>.</p>
		<p><strong>Economic Rationale:</strong></p>
		<p>Stakers provide economic security and reduce token velocity, both of which support network stability. The 15% allocation reflects their lower operational burden compared to node operators. Time-lock multipliers reward long-term commitment.</p>
	</section>

	<section class="section">
		<h2>5. Work Measurement and Verification</h2>
		<h3>5.1 The Verification Challenge</h3>
		<p>Privacy services create an inherent tension: demonstrating useful work without compromising user privacy. An operator claiming to have processed 10,000 credential verifications cannot simply reveal those credentials as proof.</p>
		<p>NONOS addresses this through service-specific metrics, peer attestation, and cryptographic commitments.</p>

		<h3>5.2 Service Metrics</h3>
		<p>Each privacy service generates measurable outputs:</p>
		<h4>Zero-Knowledge Credentials</h4>
		<ul>
			<li>Count of credentials issued (without revealing recipient identity)</li>
			<li>Count of verification proofs processed</li>
			<li>Merkle tree update transactions submitted on-chain</li>
		</ul>
		<h4>Stealth Address Services</h4>
		<ul>
			<li>Blockchain range scanned for stealth payments</li>
			<li>Number of payments indexed and served to clients</li>
			<li>Response latency for address queries</li>
		</ul>
		<h4>Cache Mixing</h4>
		<ul>
			<li>Decoy traffic volume injected</li>
			<li>Cache hit ratio</li>
			<li>Request mixing entropy</li>
		</ul>
		<h4>Fingerprint Normalisation</h4>
		<ul>
			<li>Requests processed through normalisation</li>
			<li>Attribute modification count</li>
		</ul>
		<h4>Tracker Blocking</h4>
		<ul>
			<li>Blocked request count</li>
			<li>Blocklist update frequency</li>
		</ul>
		<h4>Private Information Retrieval</h4>
		<ul>
			<li>Query volume processed</li>
			<li>Response correctness (verifiable through PIR protocol properties)</li>
		</ul>

		<h3>5.3 Peer Attestation</h3>
		<p>Operators periodically challenge each other to verify service availability and correctness.</p>
		<p><strong>Challenge Protocol:</strong></p>
		<ol>
			<li>Operator A selects operator B at random from the registered set</li>
			<li>A issues a test request (credential verification, PIR query, etc.)</li>
			<li>B responds within the timeout window</li>
			<li>A records success or failure with timestamp</li>
			<li>Attestations are aggregated into daily scores</li>
		</ol>
		<p><strong>Collusion Resistance:</strong></p>
		<ul>
			<li>Challenger selection uses verifiable random functions, preventing prediction</li>
			<li>Repeated mutual attestation between the same pair triggers investigation</li>
			<li>Attestation patterns inconsistent with client-reported metrics flag anomalies</li>
		</ul>

		<h3>5.4 Quality Oracle</h3>
		<p>Each daemon runs a quality oracle component that:</p>
		<ul>
			<li>Measures peer response times</li>
			<li>Tracks service availability</li>
			<li>Detects protocol violations</li>
			<li>Reports scores to the peer-to-peer network</li>
		</ul>
		<p>Aggregate oracle scores influence operator work calculations. Consistently poor-quality operators receive reduced reward shares.</p>

		<h3>5.5 Work Score Calculation</h3>
		<table>
			<tr><th>Factor</th><th>Weight</th><th>Measurement</th></tr>
			<tr><td>Service volume</td><td class="hl">40%</td><td>Aggregate metrics across all service types</td></tr>
			<tr><td>Uptime</td><td class="hl">25%</td><td>Successful responses to peer health checks</td></tr>
			<tr><td>Quality rating</td><td class="hl">20%</td><td>Average quality oracle score from peers</td></tr>
			<tr><td>Challenge responses</td><td class="hl">15%</td><td>Correct responses to verification challenges</td></tr>
		</table>
		<p>Scores normalise across all operators to determine reward shares.</p>
	</section>

	<section class="section">
		<h2>6. Smart Contract Infrastructure</h2>
		<h3>6.1 Contract Overview</h3>
		<div class="contracts">
			<div class="contract-card"><strong>NOX Token</strong><p>ERC-20 token for fees, staking, and rewards</p></div>
			<div class="contract-card"><strong>PrivacyWorkEconomy</strong><p>Fee collection and three-way distribution</p></div>
			<div class="contract-card"><strong>NodeOperatorRewards</strong><p>Operator registration, staking, and reward claims</p></div>
			<div class="contract-card"><strong>ZeroStateStaking</strong><p>NFT holder staking and reward distribution</p></div>
			<div class="contract-card"><strong>NOXStakingPool</strong><p>General staking with time-lock tiers</p></div>
			<div class="contract-card"><strong>FeeSwap</strong><p>ETH to NOX conversion via Uniswap</p></div>
		</div>

		<h3>6.2 Deployed Addresses</h3>
		<h4>Sepolia Testnet</h4>
		<div class="contracts">
			<div class="contract"><span>NOX Token</span><code>0xC87799c4517Dcdfc65bfefa3Be64Beb89668c66c</code></div>
			<div class="contract"><span>PrivacyWorkEconomy</span><code>0xAf8018e21Eff6F21BE305941f6d595Bd337c8bCA</code></div>
			<div class="contract"><span>NodeOperatorRewards</span><code>0x8cF7E025DDe69dA239392e54f5D344b434A62ba7</code></div>
			<div class="contract"><span>ZeroStateStaking</span><code>0xe6fD264976bcB896165525a8250908e824Fc9BD8</code></div>
			<div class="contract"><span>NOXStakingPool</span><code>0xb27DAe7EbE628ebe2E9302D7D2C71eF34Dd01705</code></div>
			<div class="contract"><span>FeeSwap</span><code>0x98AE9CBE5D5C2Bee33F6Ba6771C8a6677E102dE6</code></div>
		</div>
		<h4>Ethereum Mainnet</h4>
		<div class="contracts">
			<div class="contract"><span>NOX Token</span><code>0x0a26c80Be4E060e688d7C23aDdB92cBb5D2C9eCA</code></div>
			<div class="contract"><span>ZeroState Pass</span><code>0x7b575DD8e8b111c52Ab1e872924d4Efd4DF403df</code></div>
		</div>
		<p>Economy contracts will deploy to mainnet following testnet validation.</p>

		<h3>6.3 Key Contract Functions</h3>
		<h4>PrivacyWorkEconomy</h4>
		<ul>
			<li><code>collectFee(uint256 amount)</code>: Accept fee payment in NOX</li>
			<li><code>collectFeeETH()</code>: Accept fee payment in ETH (converts to NOX)</li>
			<li><code>distributeEpoch()</code>: Trigger epoch distribution (callable by anyone)</li>
		</ul>
		<h4>NodeOperatorRewards</h4>
		<ul>
			<li><code>registerOperator(bytes32 identity)</code>: Register as node operator</li>
			<li><code>stake(uint256 amount)</code>: Stake NOX as operator collateral</li>
			<li><code>submitWorkProof(bytes proof)</code>: Submit epoch work evidence</li>
			<li><code>claimRewards()</code>: Withdraw accumulated rewards</li>
		</ul>
		<h4>ZeroStateStaking</h4>
		<ul>
			<li><code>stakeForPass(uint256 tokenId, uint256 amount)</code>: Stake NOX for a pass</li>
			<li><code>unstake(uint256 tokenId)</code>: Remove stake and forfeit rewards</li>
			<li><code>claimRewards(uint256 tokenId)</code>: Withdraw accumulated rewards</li>
		</ul>
		<h4>NOXStakingPool</h4>
		<ul>
			<li><code>stake(uint256 amount, uint8 tier)</code>: Stake with selected lock duration</li>
			<li><code>unstake(uint256 positionId)</code>: Withdraw after lock expires</li>
			<li><code>claimRewards()</code>: Withdraw accumulated rewards</li>
		</ul>
	</section>

	<section class="section">
		<h2>7. Testnet Deployment Strategy</h2>
		<h3>7.1 Why Sepolia</h3>
		<p>Production smart contracts handle real funds. Deploying untested code to mainnet risks permanent loss. Sepolia provides:</p>
		<ul>
			<li><strong>Identical execution environment:</strong> Same EVM, same Solidity semantics, same tool compatibility.</li>
			<li><strong>Free test tokens:</strong> Faucets distribute testnet ETH for gas, enabling broad testing participation.</li>
			<li><strong>Realistic conditions:</strong> Network latency, block times, and mempool behaviour approximate mainnet.</li>
			<li><strong>Safe failure:</strong> Bugs lose worthless test tokens, not user funds.</li>
		</ul>

		<h3>7.2 Validation Objectives</h3>
		<p>The testnet phase validates:</p>
		<ul>
			<li><strong>Contract correctness:</strong> Fee distribution matches specified percentages. Staking mechanics function as designed. Edge cases behave correctly.</li>
			<li><strong>Economic dynamics:</strong> Reward distribution across varying operator counts. Tier multiplier calculations. Epoch boundary handling.</li>
			<li><strong>Integration behaviour:</strong> Browser-to-daemon-to-contract flow. Fee collection under load. Concurrent claim transactions.</li>
			<li><strong>Security properties:</strong> Access control enforcement. Reentrancy protection. Integer overflow handling.</li>
		</ul>

		<h3>7.3 Graduation Criteria</h3>
		<p>Mainnet deployment requires:</p>
		<ol>
			<li>Minimum 30 days of continuous testnet operation without critical bugs</li>
			<li>Third-party security audit with all findings addressed</li>
			<li>Load testing demonstrating stability under 10× projected usage</li>
			<li>Community review of contract source code</li>
			<li>Multi-signature deployment with key ceremony</li>
		</ol>
	</section>

	<section class="section">
		<h2>8. Security Considerations</h2>
		<h3>8.1 Attack Scenarios</h3>
		<table>
			<tr><th>Attack</th><th>Mitigation</th></tr>
			<tr><td><strong>Sybil operator attack</strong><br>Adversary registers many operators to capture disproportionate rewards.</td><td>Each operator requires 1,000 NOX stake. Creating 100 fake operators requires 100,000 NOX capital at risk. Fake operators without genuine traffic receive minimal work scores.</td></tr>
			<tr><td><strong>Work inflation</strong><br>Operator falsifies service metrics to claim higher rewards.</td><td>Peer attestation cross-validates claimed work. Metrics inconsistent with client-reported usage trigger investigation. Persistent inflation results in stake slashing.</td></tr>
			<tr><td><strong>ZeroState concentration</strong><br>Single entity acquires all ZeroState Passes.</td><td>Even with all 374 passes, holder captures only 35% of fees and must stake 374,000 NOX. Majority of fees still flow to operators.</td></tr>
			<tr><td><strong>Oracle manipulation</strong><br>Corrupted quality oracle reports false scores.</td><td>Each daemon runs its own oracle. Aggregate scores resist individual corruption. Anomalous scoring patterns result in reports being discounted.</td></tr>
		</table>

		<h3>8.2 Economic Failure Modes</h3>
		<h4>Insufficient demand</h4>
		<p>If users do not pay for services, fees remain zero and no rewards distribute. The system fails gracefully. Operators experiencing losses will cease operations. The network contracts to equilibrium where remaining operators cover costs. No inflation mechanism masks low demand.</p>
		<h4>Token price collapse</h4>
		<p>If NOX loses value, staking requirements become trivially cheap. This reduces economic security but does not break functionality. Governance may propose adjusting stake requirements if validated participants approve.</p>

		<h3>8.3 Contract Security</h3>
		<p>All contracts follow established security practices:</p>
		<ul>
			<li>OpenZeppelin library usage for standard patterns</li>
			<li>Checks-effects-interactions ordering</li>
			<li>Reentrancy guards on external calls</li>
			<li>SafeMath for arithmetic operations</li>
			<li>Access control through role-based permissions</li>
			<li>Event emission for off-chain monitoring</li>
		</ul>
	</section>

	<section class="section">
		<h2>9. Sustainability Analysis</h2>
		<h3>9.1 Operating Cost Model</h3>
		<p>A typical NONOS daemon operator incurs:</p>
		<table>
			<tr><th>Cost Category</th><th>Monthly Estimate</th></tr>
			<tr><td>Server rental (VPS)</td><td>$30-80</td></tr>
			<tr><td>Bandwidth</td><td>$10-50</td></tr>
			<tr><td>Operational time</td><td>Variable</td></tr>
			<tr><td>Stake opportunity cost</td><td>Variable</td></tr>
		</table>
		<p>Total direct costs range from $40-150 monthly depending on scale and location.</p>

		<h3>9.2 Minimum Viable Fee Volume</h3>
		<p>For the network to sustain N operators at average cost C:</p>
		<div class="formula">Required monthly fees = (N × C) / 0.50</div>
		<p>With 100 operators averaging $60 monthly cost:</p>
		<div class="formula">Required fees = (100 × $60) / 0.50 = $12,000/month</div>
		<p>This represents the threshold below which operators begin leaving due to losses.</p>

		<h3>9.3 Equilibrium Dynamics</h3>
		<p><strong>Below minimum viable fees:</strong></p>
		<ul>
			<li>Marginal operators (highest cost, lowest efficiency) exit</li>
			<li>Remaining operators receive larger individual shares</li>
			<li>Exit continues until remaining operators are profitable</li>
		</ul>
		<p><strong>Above minimum viable fees:</strong></p>
		<ul>
			<li>Existing operators profit</li>
			<li>New operators attracted by returns</li>
			<li>Entry continues until marginal operator breaks even</li>
		</ul>
		<p>This self-regulating mechanism maintains network operation across varying demand levels.</p>

		<h3>9.4 Long-term Outlook</h3>
		<p>The system makes no guarantees about returns. Unlike emission-based systems that promise stakers yield regardless of usage, the Privacy Work Economy pays only from actual fees. If privacy services lack demand, participants earn nothing.</p>
		<p>This design prioritises economic honesty over growth incentives. It assumes that genuine utility will generate sufficient demand, and that artificial incentives create unsustainable obligations.</p>
	</section>

	<section class="section">
		<h2>10. Technical Appendices</h2>

		<h3>Appendix A: Contract Source Verification</h3>
		<p>All deployed contracts are verified on Etherscan. Source code is available at:</p>
		<ul>
			<li>Sepolia: <code>https://sepolia.etherscan.io/address/[contract_address]#code</code></li>
			<li>Mainnet: <code>https://etherscan.io/address/[contract_address]#code</code></li>
		</ul>

		<h3>Appendix B: Staking Tier Parameters</h3>
		<table>
			<tr><th>Tier ID</th><th>Duration (days)</th><th>Multiplier</th><th>Early Exit Penalty</th></tr>
			<tr><td>0</td><td>14</td><td>1.0×</td><td>0%</td></tr>
			<tr><td>1</td><td>30</td><td>1.25×</td><td>10%</td></tr>
			<tr><td>2</td><td>90</td><td>1.6×</td><td>20%</td></tr>
			<tr><td>3</td><td>180</td><td>2.0×</td><td>30%</td></tr>
			<tr><td>4</td><td>365</td><td>2.5×</td><td>50%</td></tr>
		</table>
		<p>Early exit before lock expiration incurs the listed penalty on staked principal.</p>

		<h3>Appendix C: Epoch Parameters</h3>
		<table>
			<tr><th>Parameter</th><th>Value</th></tr>
			<tr><td>Epoch duration</td><td>86,400 seconds (24 hours)</td></tr>
			<tr><td>Epoch start reference</td><td>Unix timestamp 0 (midnight UTC, 1 January 1970)</td></tr>
			<tr><td>Distribution delay</td><td>1 hour after epoch end</td></tr>
			<tr><td>Claim expiration</td><td>None (rewards accumulate indefinitely)</td></tr>
		</table>

		<h3>Appendix D: Operator Registration</h3>
		<p>Operators register by calling <code>registerOperator</code> with:</p>
		<ul>
			<li><code>identity</code>: 32-byte commitment to operator public key</li>
			<li>Transaction must include minimum stake (1,000 NOX)</li>
			<li>Registration emits <code>OperatorRegistered</code> event</li>
		</ul>
		<p>Deregistration requires:</p>
		<ul>
			<li>No pending slashing conditions</li>
			<li>7-day cooldown period</li>
			<li>Stake returned after cooldown</li>
		</ul>

		<h3>Appendix E: Integration Points</h3>
		<h4>Daemon to Contract</h4>
		<ul>
			<li>Work proof submission at epoch boundaries</li>
			<li>Reward claim transactions</li>
			<li>Quality oracle score publication</li>
		</ul>
		<h4>Browser to Daemon</h4>
		<ul>
			<li>Service requests via local RPC (port 8420)</li>
			<li>Fee payment authorisation</li>
			<li>Session credential requests</li>
		</ul>
		<h4>Contract to Contract</h4>
		<ul>
			<li>PrivacyWorkEconomy calls distribution functions on child contracts</li>
			<li>FeeSwap interacts with Uniswap router</li>
			<li>ZeroStateStaking queries NFT ownership from pass contract</li>
		</ul>
	</section>

	<section class="section last">
		<div class="callout highlight">
			<strong>Fee-funded only. Verifiable work. Aligned incentives. Transparent allocation.</strong><br><br>
			The Privacy Work Economy ensures sustainable economics through real fees from real users who value privacy.
		</div>
	</section>

	<footer class="doc-footer">
		<p>NONOS Privacy Work Economy · Version 1.0 · March 2026</p>
		<p>This document describes the intended behaviour of the NONOS economic system. Implementation details may differ. Consult deployed contract source code for authoritative specifications.</p>
		<p class="hl">nonos.systems</p>
	</footer>
</article>

<style>
	.doc { max-width: 850px; line-height: 1.7; }
	.doc-header { margin-bottom: 3rem; padding-bottom: 2rem; border-bottom: 1px solid #1a1a1a; }
	.brand { display: flex; align-items: center; gap: 10px; margin-bottom: 1rem; }
	.mark { width: 40px; height: 40px; border: 2px solid #00ffcc; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 18px; font-weight: 700; color: #00ffcc; }
	.wordmark { font-size: 20px; font-weight: 600; color: #e0e0e0; letter-spacing: 4px; }
	.doc-header h1 { font-size: 2.5rem; font-weight: 300; margin-bottom: 0.5rem; }
	.tagline { color: #888; font-size: 1.1rem; margin-bottom: 0.5rem; }
	.meta { color: #666; font-size: 0.85rem; }
	.toc { background: #111; border: 1px solid #222; border-radius: 8px; padding: 1.5rem 2rem; margin-bottom: 3rem; }
	.toc h3 { font-size: 0.85rem; text-transform: uppercase; letter-spacing: 2px; color: #00ffcc; margin-bottom: 1rem; }
	.toc ol { margin: 0; padding-left: 1.5rem; color: #e0e0e0; }
	.toc li { margin-bottom: 0.5rem; }
	.section { margin-bottom: 3rem; padding-bottom: 2rem; border-bottom: 1px solid #222; }
	.section.last { border-bottom: none; }
	.section h2 { font-size: 13px; font-weight: 600; letter-spacing: 3px; text-transform: uppercase; color: #888; margin-bottom: 2rem; padding-bottom: 1rem; border-bottom: 1px solid #222; }
	.section h3 { font-size: 1.5rem; font-weight: 600; color: #e0e0e0; margin: 2rem 0 1rem; }
	.section h4 { font-size: 1.1rem; font-weight: 600; color: #00ffcc; margin: 1.5rem 0 0.75rem; }
	.section p { color: #e0e0e0; margin-bottom: 1rem; }
	.hl { color: #00ffcc; font-weight: 500; }
	ul, ol { margin: 1rem 0 1rem 0; padding-left: 1.5rem; color: #e0e0e0; }
	li { margin-bottom: 0.5rem; padding-left: 0.5rem; }
	li::marker { color: #00ffcc; }
	.callout { background: #111; border-left: 3px solid #00ffcc; padding: 1.25rem 1.5rem; margin: 1.5rem 0; color: #e0e0e0; }
	.callout.highlight { background: #111; text-align: center; font-size: 1.05rem; }
	.callout strong { color: #00ffcc; }
	.formula { background: #111; border: 1px solid #222; border-radius: 8px; padding: 1.25rem 2rem; margin: 1.5rem 0; font-family: 'JetBrains Mono', monospace; font-size: 0.9rem; text-align: center; color: #00ffcc; }
	table { width: 100%; border-collapse: collapse; margin: 1.5rem 0; font-size: 0.9rem; }
	th { text-align: left; padding: 1rem; background: #111; color: #00ffcc; font-weight: 600; font-size: 0.75rem; letter-spacing: 1px; text-transform: uppercase; border-bottom: 1px solid #222; }
	td { padding: 1rem; border-bottom: 1px solid #222; color: #e0e0e0; vertical-align: top; }
	tr:last-child td { border-bottom: none; }
	code { font-family: 'JetBrains Mono', monospace; background: #111; padding: 3px 8px; border-radius: 4px; font-size: 0.85rem; color: #00ffcc; }
	.stats-row { display: flex; justify-content: center; gap: 3rem; margin: 2rem 0; }
	.stat { text-align: center; background: #111; border: 1px solid #222; border-radius: 8px; padding: 1.5rem 2rem; }
	.num { display: block; font-size: 2.25rem; font-weight: 600; color: #00ffcc; margin-bottom: 0.5rem; }
	.label { font-size: 0.75rem; color: #888; text-transform: uppercase; letter-spacing: 1px; }
	.contracts { display: flex; flex-direction: column; gap: 0.75rem; margin: 1rem 0; }
	.contract { display: flex; justify-content: space-between; align-items: center; padding: 1rem 1.25rem; background: #111; border: 1px solid #222; border-radius: 8px; }
	.contract span { color: #e0e0e0; font-size: 0.9rem; }
	.contract code { font-size: 0.75rem; background: #0a0a0a; }
	.contract-card { background: #111; border: 1px solid #222; border-radius: 8px; padding: 1.25rem; margin-bottom: 0.75rem; }
	.contract-card strong { color: #00ffcc; font-family: 'JetBrains Mono', monospace; font-size: 0.95rem; display: block; margin-bottom: 0.5rem; }
	.contract-card p { margin: 0; font-size: 0.85rem; color: #888; }
	.principles { display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem; margin: 1.5rem 0; }
	.principle { background: #111; border: 1px solid #222; border-radius: 8px; padding: 1.25rem; }
	.principle strong { color: #00ffcc; display: block; margin-bottom: 0.5rem; }
	.principle p { margin: 0; font-size: 0.85rem; color: #888; }
	.doc-footer { text-align: center; padding-top: 2rem; margin-top: 1rem; border-top: 1px solid #222; }
	.doc-footer p { color: #666; font-size: 0.85rem; margin-bottom: 0.5rem; }
	@media (max-width: 700px) {
		.stats-row { flex-direction: column; gap: 1rem; }
		.contract { flex-direction: column; align-items: flex-start; gap: 0.5rem; }
		.principles { grid-template-columns: 1fr; }
	}
</style>
