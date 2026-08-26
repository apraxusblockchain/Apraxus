import { network } from "hardhat";
import { formatEther } from "viem";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();
const [walletClient] = await viem.getWalletClients();

const address = walletClient.account.address;

const balance = await publicClient.getBalance({
  address,
});

console.log("=================================");
console.log("   ARBITRUM SEPOLIA ETH CHECK");
console.log("=================================");
console.log("Wallet:", address);
console.log("ETH:", formatEther(balance));
console.log("=================================");
