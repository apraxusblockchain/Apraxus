import { network } from "hardhat";
import { keccak256, encodeAbiParameters, parseAbiParameters } from "viem";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();

const poolManager =
  "0xFB3e0C6F74eB1a21CC1Da29aeC80D2Dfe6C9a317";

const weth =
  "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73";

const apxs =
  "0xFE16213961cb4f9B15301f730a5977b9A145add5";

console.log("=================================");
console.log("      APXS / WETH POOL CHECK");
console.log("=================================");
console.log("Network: Arbitrum Sepolia");
console.log("PoolManager:", poolManager);
console.log("WETH:", weth);
console.log("APXS:", apxs);
console.log("---------------------------------");
console.log("currency0 = WETH");
console.log("currency1 = APXS");
console.log("fee = 3000");
console.log("tickSpacing = 60");
console.log("hooks = 0x0000000000000000000000000000000000000000");
console.log("=================================");
