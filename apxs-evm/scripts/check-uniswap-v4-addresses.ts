import { network } from "hardhat";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();

const contracts = {
  PoolManager: "0xFB3e0C6F74eB1a21CC1Da29aeC80D2Dfe6C9a317",
  PositionManager: "0xAc631556d3d4019C95769033B5E719dD77124BAc",
  Quoter: "0xAc631556d3d4019C95769033B5E719dD77124BAc",
  UniversalRouter: "0xefd1d4bd4cf1e86da286bb4cb1b8bced9c10ba47",
  Permit2: "0x000000000022D473030F116dDEE9F6B43aC78BA3",
};

console.log("=================================");
console.log(" UNISWAP V4 OFFICIAL ADDRESS CHECK");
console.log("=================================");
console.log("Network: Arbitrum Sepolia");

for (const [name, address] of Object.entries(contracts)) {
  const code = await publicClient.getCode({
    address: address as `0x${string}`,
  });

  console.log(`${name}: ${code && code !== "0x" ? "FOUND" : "NOT FOUND"}`);
  console.log(address);
  console.log("---------------------------------");
}

console.log("=================================");
