import { createPublicClient, http, getAddress } from "viem";
import { arbitrumSepolia } from "viem/chains";

const client = createPublicClient({
  chain: arbitrumSepolia,
  transport: http(),
});

const PM = getAddress("0xAc631556d3d4019C95769033B5E719dD77124BAc");

const abi = [
  {
    type: "function",
    name: "poolManager",
    stateMutability: "view",
    inputs: [],
    outputs: [{ type: "address" }],
  },
  {
    type: "function",
    name: "permit2",
    stateMutability: "view",
    inputs: [],
    outputs: [{ type: "address" }],
  },
] as const;

console.log("=================================");
console.log(" POSITION MANAGER DETAILS");
console.log("=================================");
console.log("PositionManager:", PM);

try {
  const poolManager = await client.readContract({
    address: PM,
    abi,
    functionName: "poolManager",
  });
  console.log("poolManager():", poolManager);
} catch (e) {
  console.log("poolManager(): NOT AVAILABLE");
}

try {
  const permit2 = await client.readContract({
    address: PM,
    abi,
    functionName: "permit2",
  });
  console.log("permit2():", permit2);
} catch (e) {
  console.log("permit2(): NOT AVAILABLE");
}

console.log("=================================");
