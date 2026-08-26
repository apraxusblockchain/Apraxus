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
      { name: "spender", type: "address" },
      { name: "amount", type: "uint256" },
    ],
    outputs: [{ type: "bool" }],
  },
] as const;

async function approve(
  name: string,
  token: `0x${string}`,
  amount: bigint,
) {
  console.log("---------------------------------");
  console.log("Approving:", name);
  console.log("Token:", token);
  console.log("Spender:", permit2);
  console.log("Amount:", amount.toString());

  const hash = await walletClient.writeContract({
    address: token,
    abi,
    functionName: "approve",
    args: [permit2, amount],
  });

  console.log("Transaction:", hash);

  await publicClient.waitForTransactionReceipt({
    hash,
  });

  console.log("Confirmed!");
}

console.log("=================================");
console.log("     APXS / WETH PERMIT2 APPROVAL");
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
console.log("ERC20 -> Permit2 approvals DONE");
console.log("=================================");
