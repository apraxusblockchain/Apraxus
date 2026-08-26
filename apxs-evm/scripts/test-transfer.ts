import { network } from "hardhat";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();
const [walletClient] = await viem.getWalletClients();

const token = "0xFE16213961cb4f9B15301f730a5977b9A145add5";
const treasury = "0x0488CE4eb88d7146177BF4eC7781eD8182b3cE26";

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

const amount = 1n * 10n ** 8n;

console.log("=================================");
console.log("       APXS TRANSFER TEST");
console.log("=================================");
console.log("Network: Arbitrum Sepolia");
console.log("Token:", token);
console.log("From:", walletClient.account.address);
console.log("To Treasury:", treasury);
console.log("Amount: 1 APXS");
console.log("=================================");

const hash = await walletClient.writeContract({
  address: token,
  abi,
  functionName: "transfer",
  args: [treasury, amount],
});

console.log("Transaction:", hash);

const receipt = await publicClient.waitForTransactionReceipt({
  hash,
});

console.log("Transaction confirmed!");
console.log("Block:", receipt.blockNumber.toString());
console.log("=================================");