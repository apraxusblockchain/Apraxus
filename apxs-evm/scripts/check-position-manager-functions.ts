import {
  getAddress,
  toFunctionSelector,
} from "viem";

const PM = getAddress(
  "0xAc631556d3d4019C95769033B5E719dD77124BAc"
);

const abi = [
  {
    type: "function",
    name: "modifyLiquidities",
    stateMutability: "payable",
    inputs: [
      { name: "unlockData", type: "bytes" },
      { name: "deadline", type: "uint256" },
    ],
    outputs: [],
  },
  {
    type: "function",
    name: "modifyLiquiditiesWithoutUnlock",
    stateMutability: "payable",
    inputs: [
      { name: "actions", type: "bytes" },
      { name: "params", type: "bytes[]" },
    ],
    outputs: [],
  },
] as const;

console.log("=================================");
console.log(" POSITION MANAGER FUNCTIONS");
console.log("=================================");
console.log("PositionManager:", PM);
console.log("---------------------------------");

for (const item of abi) {
  console.log(
    item.name,
    "selector:",
    toFunctionSelector(item)
  );
}

console.log("=================================");
