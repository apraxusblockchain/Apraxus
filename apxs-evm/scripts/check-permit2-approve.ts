import { network } from "hardhat";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();

const permit2 =
  "0x000000000022D473030F116dDEE9F6B43aC78BA3";

const abi = [
  {
    type: "function",
    name: "approve",
    stateMutability: "nonpayable",
    inputs: [
      { name: "token", type: "address" },
      { name: "spender", type: "address" },
      { name: "amount", type: "uint160" },
      { name: "expiration", type: "uint48" },
    ],
    outputs: [],
  },
] as const;

const selector = await publicClient.readContract({
  address: permit2,
  abi,
  functionName: "approve",
  args: [
    "0xFE16213961cb4f9B15301f730a5977b9A145add5",
    "0xAc631556d3d4019C95769033B5E719dD77124BAc",
    1000000000000n,
    1787734440,
  ],
}).catch((error) => {
  console.log("READ RESULT: function exists / transaction simulation may require state change");
  console.log("Error:", error.shortMessage ?? error.message);
});

console.log("=================================");
console.log("      PERMIT2 APPROVE CHECK");
console.log("=================================");
console.log("Permit2:", permit2);
console.log("Function: approve(token, spender, amount, expiration)");
console.log("=================================");
