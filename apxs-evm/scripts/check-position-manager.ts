import { network } from "hardhat";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();

const positionManager =
  "0xAc631556d3d4019C95769033B5E719dD77124BAc";

const poolManager =
  "0xFB3e0C6F74eB1a21CC1Da29aeC80D2Dfe6C9a317";

const abi = [
  {
    type: "function",
    name: "poolManager",
    stateMutability: "view",
    inputs: [],
    outputs: [{ type: "address" }],
  },
] as const;

console.log("=================================");
console.log(" POSITION MANAGER CHECK");
console.log("=================================");
console.log("PositionManager:", positionManager);

const code = await publicClient.getCode({
  address: positionManager,
});

console.log("Code:", code ? "FOUND" : "NOT FOUND");
console.log("Code length:", code?.length ?? 0);

try {
  const pm = await publicClient.readContract({
    address: positionManager,
    abi,
    functionName: "poolManager",
  });

  console.log("PositionManager.poolManager():", pm);
  console.log("Expected PoolManager:", poolManager);
  console.log(
    "MATCH:",
    pm.toLowerCase() === poolManager.toLowerCase()
      ? "YES"
      : "NO",
  );
} catch (e: any) {
  console.log("poolManager() read failed:");
  console.log(e.shortMessage ?? e.message);
}

console.log("=================================");
