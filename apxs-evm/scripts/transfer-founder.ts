import { network } from "hardhat";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();
const [walletClient] = await viem.getWalletClients();

const token = "0xFE16213961cb4f9B15301f730a5977b9A145add5";

const founder = "0x90f771DD3FCF441cE33Ad481Ade618Ed33C601b3";

const abi = [
  {
    type: "function",
    name: "transfer",
    stateMutability: "nonpayable",
    inputs: [
      { name: "to", type: "address" },
      { name: "value", type: "uint256" },
    ],
    outputs: [{ type: "bool" }],
  },
] as const;

const amount = 100_000_000n * 10n ** 8n;

console.log("=================================");
console.log("       APXS FOUNDER TRANSFER");
console.log("=================================");
console.log("Network: Arbitrum Sepolia");
console.log("Token:", token);
console.log("From:", walletClient.account.address);
console.log("To:", founder);
console.log("Amount: 100,000,000 APXS");
console.log("=================================");

const hash = await walletClient.writeContract({
  address: token,
  abi,
  functionName: "transfer",
  args: [founder, amount],
});

console.log("Transaction:", hash);

const receipt = await publicClient.waitForTransactionReceipt({
  hash,
});

console.log("Transaction confirmed!");
console.log("Block:", receipt.blockNumber.toString());
console.log("=================================");