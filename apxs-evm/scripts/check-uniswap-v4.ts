import { network } from "hardhat";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();

const poolManager =
  "0xFB3e0C6F74eB1a21CC1Da29aeC80D2Dfe6C9a317";

const positionManager =
  "0xAc631556d3d4019C95769033B5E719dD77124BAc";

const quoter =
  "0xAc631556d3d4019C95769033B5E719dD77124BAc";

console.log("=================================");
console.log("     UNISWAP V4 CHECK");
console.log("=================================");
console.log("Network: Arbitrum Sepolia");
console.log("PoolManager:", poolManager);
console.log("PositionManager:", positionManager);
console.log("Quoter:", quoter);

const poolCode = await publicClient.getCode({
  address: poolManager,
});

const positionCode = await publicClient.getCode({
  address: positionManager,
});

const quoterCode = await publicClient.getCode({
  address: quoter,
});

console.log("---------------------------------");
console.log("PoolManager code:", poolCode ? "FOUND" : "NOT FOUND");
console.log(
  "PositionManager code:",
  positionCode ? "FOUND" : "NOT FOUND",
);
console.log("Quoter code:", quoterCode ? "FOUND" : "NOT FOUND");
console.log("=================================");
