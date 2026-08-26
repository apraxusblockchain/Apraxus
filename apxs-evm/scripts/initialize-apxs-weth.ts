import { network } from "hardhat";
import {
  encodeAbiParameters,
  keccak256,
  parseAbiParameters,
} from "viem";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();
const [walletClient] = await viem.getWalletClients();

const poolManager =
  "0xFB3e0C6F74eB1a21CC1Da29aeC80D2Dfe6C9a317";

const weth =
  "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73";

const apxs =
  "0xFE16213961cb4f9B15301f730a5977b9A145add5";

const hooks =
  "0x0000000000000000000000000000000000000000";

const sqrtPriceX96 =
  7922816251426433759354395033n;

const fee = 3000;
const tickSpacing = 60;

const poolKey = {
  currency0: weth,
  currency1: apxs,
  fee,
  tickSpacing,
  hooks,
};

const poolId = keccak256(
  encodeAbiParameters(
    [
      {
        type: "tuple",
        components: [
          { name: "currency0", type: "address" },
          { name: "currency1", type: "address" },
          { name: "fee", type: "uint24" },
          { name: "tickSpacing", type: "int24" },
          { name: "hooks", type: "address" },
        ],
      },
    ],
    [poolKey],
  ),
);

const abi = [
  {
    type: "function",
    name: "initialize",
    stateMutability: "nonpayable",
    inputs: [
      {
        name: "key",
        type: "tuple",
        components: [
          { name: "currency0", type: "address" },
          { name: "currency1", type: "address" },
          { name: "fee", type: "uint24" },
          { name: "tickSpacing", type: "int24" },
          { name: "hooks", type: "address" },
        ],
      },
      {
        name: "sqrtPriceX96",
        type: "uint160",
      },
    ],
    outputs: [
      { name: "tick", type: "int24" },
    ],
  },
] as const;

console.log("=================================");
console.log("    APXS / WETH POOL INIT");
console.log("=================================");
console.log("Network: Arbitrum Sepolia");
console.log("PoolManager:", poolManager);
console.log("Pool ID:", poolId);
console.log("currency0:", weth);
console.log("currency1:", apxs);
console.log("fee:", fee);
console.log("tickSpacing:", tickSpacing);
console.log("hooks:", hooks);
console.log("sqrtPriceX96:", sqrtPriceX96.toString());
console.log("---------------------------------");
console.log("Initializing pool...");
console.log("=================================");

const hash = await walletClient.writeContract({
  address: poolManager,
  abi,
  functionName: "initialize",
  args: [poolKey, sqrtPriceX96],
});

console.log("Transaction:", hash);

const receipt = await publicClient.waitForTransactionReceipt({
  hash,
});

console.log("Transaction confirmed!");
console.log("Block:", receipt.blockNumber.toString());
console.log("=================================");
