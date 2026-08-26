import { network } from "hardhat";
import { formatEther } from "viem";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();
const [walletClient] = await viem.getWalletClients();

const weth = "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73";

const abi = [
  {
    type: "function",
    name: "deposit",
    stateMutability: "payable",
    inputs: [],
    outputs: [],
  },
] as const;

console.log("=================================");
console.log("       APXS WETH TEST");
console.log("=================================");
console.log("Network: Arbitrum Sepolia");
console.log("Wallet:", walletClient.account.address);
console.log("WETH:", weth);
console.log("Wrapping: 0.01 ETH");
console.log("=================================");

const hash = await walletClient.writeContract({
  address: weth,
  abi,
  functionName: "deposit",
  value: 10_000_000_000_000_000n,
});

console.log("Transaction:", hash);

const receipt = await publicClient.waitForTransactionReceipt({
  hash,
});

console.log("Transaction confirmed!");
console.log("Block:", receipt.blockNumber.toString());
console.log("=================================");
