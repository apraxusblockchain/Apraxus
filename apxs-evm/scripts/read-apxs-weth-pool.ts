import { network } from "hardhat";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();

const stateView =
  "0x9d467fa9062b6e9b1a46e26007ad82db116c67cb";

const poolId =
  "0x40c82be5ba64731e3396bdaab91434a64b89f3cdf80ec493d0a5fafa28f1ae24";

const abi = [
  {
    type: "function",
    name: "getSlot0",
    stateMutability: "view",
    inputs: [
      { name: "poolId", type: "bytes32" },
    ],
    outputs: [
      { name: "sqrtPriceX96", type: "uint160" },
      { name: "tick", type: "int24" },
      { name: "protocolFee", type: "uint24" },
      { name: "lpFee", type: "uint24" },
    ],
  },
  {
    type: "function",
    name: "getLiquidity",
    stateMutability: "view",
    inputs: [
      { name: "poolId", type: "bytes32" },
    ],
    outputs: [
      { name: "liquidity", type: "uint128" },
    ],
  },
] as const;

console.log("=================================");
console.log("     APXS / WETH POOL STATE");
console.log("=================================");
console.log("Network: Arbitrum Sepolia");
console.log("StateView:", stateView);
console.log("Pool ID:", poolId);
console.log("---------------------------------");

try {
  const slot0 = await publicClient.readContract({
    address: stateView,
    abi,
    functionName: "getSlot0",
    args: [poolId],
  });

  const liquidity = await publicClient.readContract({
    address: stateView,
    abi,
    functionName: "getLiquidity",
    args: [poolId],
  });

  console.log("sqrtPriceX96:", slot0[0].toString());
  console.log("tick:", slot0[1].toString());
  console.log("protocolFee:", slot0[2].toString());
  console.log("lpFee:", slot0[3].toString());
  console.log("liquidity:", liquidity.toString());

  console.log("---------------------------------");

  if (slot0[0] === 0n && liquidity === 0n) {
    console.log("STATUS: POOL NOT INITIALIZED");
  } else {
    console.log("STATUS: POOL HAS STATE");
  }
} catch (error) {
  console.log("STATUS: POOL STATE READ FAILED");
  console.log(error);
}

console.log("=================================");
