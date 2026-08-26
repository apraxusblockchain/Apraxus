import { useEffect, useState } from 'react'
import { createPublicClient, http } from 'viem'
import { arbitrumSepolia } from 'viem/chains'
import './App.css'

const client = createPublicClient({
  chain: arbitrumSepolia,
  transport: http('https://sepolia-rollup.arbitrum.io/rpc'),
})

const STATE_VIEW =
  '0x9d467fa9062b6e9b1a46e26007ad82db116c67cb'

const POSITION_MANAGER =
  '0xAc631556d3d4019C95769033B5E719dD77124BAc'

const POOL_ID =
  '0x40c82be5ba64731e3396bdaab91434a64b89f3cdf80ec493d0a5fafa28f1ae24'

const TOKEN_ID = 502n

const stateViewAbi = [
  {
    type: 'function',
    name: 'getSlot0',
    stateMutability: 'view',
    inputs: [{ name: 'poolId', type: 'bytes32' }],
    outputs: [
      { name: 'sqrtPriceX96', type: 'uint160' },
      { name: 'tick', type: 'int24' },
      { name: 'protocolFee', type: 'uint24' },
      { name: 'lpFee', type: 'uint24' },
    ],
  },
  {
    type: 'function',
    name: 'getLiquidity',
    stateMutability: 'view',
    inputs: [{ name: 'poolId', type: 'bytes32' }],
    outputs: [{ name: 'liquidity', type: 'uint128' }],
  },
] as const

const positionAbi = [
  {
    type: 'function',
    name: 'ownerOf',
    stateMutability: 'view',
    inputs: [{ name: 'tokenId', type: 'uint256' }],
    outputs: [{ name: '', type: 'address' }],
  },
  {
    type: 'function',
    name: 'getPositionLiquidity',
    stateMutability: 'view',
    inputs: [{ name: 'tokenId', type: 'uint256' }],
    outputs: [{ name: 'liquidity', type: 'uint128' }],
  },
] as const

declare global {
  interface Window {
    ethereum?: {
      request: (args: {
        method: string
        params?: unknown[]
      }) => Promise<unknown>
    }
  }
}

function shortenAddress(address: string) {
  return `${address.slice(0, 6)}...${address.slice(-4)}`
}

function App() {
  const [account, setAccount] = useState<string | null>(null)
  const [connecting, setConnecting] = useState(false)
  const [tick, setTick] = useState<number | null>(null)
  const [poolLiquidity, setPoolLiquidity] = useState<bigint | null>(null)
  const [positionLiquidity, setPositionLiquidity] = useState<bigint | null>(null)
  const [positionOwner, setPositionOwner] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [manageOpen, setManageOpen] = useState(false)

  async function connectWallet() {
    if (!window.ethereum) {
      alert('MetaMask is not installed.')
      return
    }

    try {
      setConnecting(true)

      const accounts = (await window.ethereum.request({
        method: 'eth_requestAccounts',
      })) as string[]

      if (accounts.length > 0) {
        setAccount(accounts[0])
      }
    } catch (error) {
      console.error('Wallet connection failed:', error)
    } finally {
      setConnecting(false)
    }
  }

  async function loadOnChainData() {
    try {
      setLoading(true)
      setError(null)

      const [slot0, liquidity, owner, positionLiq] =
        await Promise.all([
          client.readContract({
            address: STATE_VIEW,
            abi: stateViewAbi,
            functionName: 'getSlot0',
            args: [POOL_ID],
          }),

          client.readContract({
            address: STATE_VIEW,
            abi: stateViewAbi,
            functionName: 'getLiquidity',
            args: [POOL_ID],
          }),

          client.readContract({
            address: POSITION_MANAGER,
            abi: positionAbi,
            functionName: 'ownerOf',
            args: [TOKEN_ID],
          }),

          client.readContract({
            address: POSITION_MANAGER,
            abi: positionAbi,
            functionName: 'getPositionLiquidity',
            args: [TOKEN_ID],
          }),
        ])

      setTick(Number(slot0[1]))
      setPoolLiquidity(liquidity)
      setPositionOwner(owner)
      setPositionLiquidity(positionLiq)
    } catch (err) {
      console.error('On-chain data failed:', err)
      setError('Unable to read Arbitrum Sepolia data.')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadOnChainData()

    const interval = setInterval(loadOnChainData, 15000)

    return () => clearInterval(interval)
  }, [])

  return (
    <main className="app">
      <header className="nav">
        <div className="brand">
          <div className="logo">A</div>

          <div>
            <strong>APRAXUS</strong>
            <span>APXS LIQUIDITY</span>
          </div>
        </div>

        <button
          className="connect"
          onClick={connectWallet}
          disabled={connecting}
        >
          {connecting
            ? 'Connecting...'
            : account
              ? shortenAddress(account)
              : 'Connect Wallet'}
        </button>
      </header>

      <section className="hero">
        <div className="eyebrow">
          ARBITRUM SEPOLIA · UNISWAP V4
        </div>

        <h1>APXS / WETH</h1>

        <p>
          Decentralized liquidity infrastructure for Apraxus.
          <br />
          Verified on-chain deployment.
        </p>
      </section>

      <section className="grid">
        <div className="card">
          <span>POOL</span>
          <h2>APXS / WETH</h2>
          <p>Uniswap v4 · 0.30% fee</p>
        </div>

        <div className="card">
          <span>CURRENT TICK</span>

          <h2>
            {loading ? 'Loading...' : tick ?? '—'}
          </h2>

          <p>Tick spacing: 60</p>
        </div>

        <div className="card">
          <span>POOL LIQUIDITY</span>

          <h2>
            {loading
              ? 'Loading...'
              : poolLiquidity?.toLocaleString() ?? '—'}
          </h2>

          <p>
            {error ? error : 'Live on-chain data'}
          </p>
        </div>

        <div className="card">
          <span>LP POSITION</span>

          <h2>#502</h2>

          <p>
            {positionLiquidity !== null
              ? `Liquidity ${positionLiquidity.toLocaleString()}`
              : 'Loading position...'}
          </p>
        </div>
      </section>

      <section className="position">
        <div>
          <span>YOUR POSITION</span>

          <h2>APXS / WETH</h2>

          <p>
            {account
              ? `Connected wallet: ${shortenAddress(account)}`
              : positionOwner
                ? `Position owner: ${shortenAddress(positionOwner)}`
                : 'Connect your wallet to manage liquidity'}
          </p>
        </div>

        <button
          className="primary"
          onClick={() => {
            if (account) {
              setManageOpen(true)
            } else {
              connectWallet()
            }
          }}
        >
          {account ? 'Manage Liquidity' : 'Connect Wallet'}
        </button>
      </section>

      {manageOpen && (
        <section className="manage-panel">
          <div className="manage-header">
            <div>
              <span>POSITION MANAGEMENT</span>
              <h2>APXS / WETH · #502</h2>
            </div>

            <button
              className="close-button"
              onClick={() => setManageOpen(false)}
            >
              Close
            </button>
          </div>

          <div className="manage-grid">
            <div>
              <span>CURRENT TICK</span>
              <strong>{tick ?? '—'}</strong>
            </div>

            <div>
              <span>POSITION RANGE</span>
              <strong>-46200 → -45960</strong>
            </div>

            <div>
              <span>LIQUIDITY</span>
              <strong>
                {positionLiquidity?.toLocaleString() ?? '—'}
              </strong>
            </div>

            <div>
              <span>POSITION OWNER</span>
              <strong>
                {positionOwner
                  ? shortenAddress(positionOwner)
                  : '—'}
              </strong>
            </div>

            <div>
              <span>WETH REQUIRED</span>
              <strong>0.00006447218057508</strong>
            </div>

            <div>
              <span>APXS REQUIRED</span>
              <strong>9,999.99999999</strong>
            </div>
          </div>

          <div className="manage-status">
            <span>STATUS</span>
            <strong>✓ VERIFIED ON-CHAIN</strong>
            <p>
              Position #502 is currently active in the
              APXS/WETH Uniswap v4 pool.
            </p>
          </div>
        </section>
      )}

      <section className="contracts">
        <h2>Verified Contracts</h2>

        <div className="contract">
          <span>APXS</span>
          <code>0xFE16...add5</code>
        </div>

        <div className="contract">
          <span>WETH</span>
          <code>0x980B...7c73</code>
        </div>

        <div className="contract">
          <span>PoolManager</span>
          <code>0xFB3e...a317</code>
        </div>

        <div className="contract">
          <span>PositionManager</span>
          <code>0xAc63...4BAc</code>
        </div>
      </section>
    </main>
  )
}

export default App
