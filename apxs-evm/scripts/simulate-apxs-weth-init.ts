import { network } from "hardhat";
import { encodeFunctionData } from "viem";

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

const key = {
  currency0: weth,
  currency1: apxs,
  fee: 3000,
  tickSpacing: 60,
  hooks,
};

console.log("=================================");
console.log("   APXS / WETH INIT SIMULATION");
console.log("=================================");
console.log("PoolManager:", poolManager);
console.log("currency0:", weth);
console.log("currency1:", apxs);
console.log("fee:", key.fee);
console.log("tickSpacing:", key.tickSpacing);
console.log("hooks:", hooks);
console.log("sqrtPriceX96:", sqrtPriceX96.toString());
console.log("---------------------------------");
console.log("SIMULATING ONLY — NO TRANSACTION");
console.log("---------------------------------");

try {
  const result = await publicClient.simulateContract({
    address: poolManager,
    abi,
    functionName: "initialize",
    args: [key, sqrtPriceX96],
    account: walletClient.account,
  });

  console.log("SIMULATION: SUCCESS");
  console.log("Returned tick:", result.result.toString());
} catch (error) {
  console.log("SIMULATION: FAILED");
  console.log(error);
}

console.log("=================================");
