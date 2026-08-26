import { network } from "hardhat";

const { viem } = await network.create({
  network: "sepolia",
  chainType: "l1",
});

const [walletClient] = await viem.getWalletClients();

console.log("Wallet address:", walletClient.account.address);