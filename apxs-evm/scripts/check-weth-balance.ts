import { network } from "hardhat";
import { formatUnits } from "viem";

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
    name: "balanceOf",
    stateMutability: "view",
    inputs: [{ name: "account", type: "address" }],
    outputs: [{ type: "uint256" }],
  },
  {
    type: "function",
    name: "decimals",
    stateMutability: "view",
    inputs: [],
    outputs: [{ type: "uint8" }],
  },
] as const;

const decimals = await publicClient.readContract({
  address: weth,
  abi,
  functionName: "decimals",
});

const balance = await publicClient.readContract({
  address: weth,
  abi,
  functionName: "balanceOf",
  args: [walletClient.account.address],
});

console.log("=================================");
console.log("   ARBITRUM SEPOLIA WETH CHECK");
console.log("=================================");
console.log("Wallet:", walletClient.account.address);
console.log("WETH:", weth);
console.log("Balance:", formatUnits(balance, decimals), "WETH");
console.log("=================================");
