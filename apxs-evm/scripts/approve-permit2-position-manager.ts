import { network } from "hardhat";
import { parseUnits } from "viem";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();
const [walletClient] = await viem.getWalletClients();

const permit2 =
  "0x000000000022D473030F116dDEE9F6B43aC78BA3";

const positionManager =
  "0xAc631556d3d4019C95769033B5E719dD77124BAc";

const APXS =
  "0xFE16213961cb4f9B15301f730a5977b9A145add5";

const WETH =
  "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73";

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

async function approve(
  name: string,
  token: `0x${string}`,
  amount: bigint,
) {
  console.log("---------------------------------");
  console.log("Token:", name);
  console.log("Amount:", amount.toString());
  console.log("Spender:", positionManager);

  const expiration =
    BigInt(Math.floor(Date.now() / 1000) + 86400);

  const hash = await walletClient.writeContract({
    address: permit2,
    abi,
    functionName: "approve",
    args: [
      token,
      positionManager,
      amount,
      expiration,
    ],
  });

  console.log("Transaction:", hash);

  await publicClient.waitForTransactionReceipt({
    hash,
  });

  console.log("Confirmed!");
}

console.log("=================================");
console.log(" PERMIT2 → POSITION MANAGER");
console.log("=================================");
console.log("Wallet:", walletClient.account.address);

await approve(
  "WETH",
  WETH,
  parseUnits("0.0001", 18),
);

await approve(
  "APXS",
  APXS,
  parseUnits("10000", 8),
);

console.log("=================================");
console.log("PERMIT2 ALLOWANCES SET");
console.log("=================================");
