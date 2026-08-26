import { encodeAbiParameters, keccak256 } from "viem";

const weth =
  "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73";

const apxs =
  "0xFE16213961cb4f9B15301f730a5977b9A145add5";

const fee = 3000;
const tickSpacing = 60;
const hooks =
  "0x0000000000000000000000000000000000000000";

const encoded = encodeAbiParameters(
  [
    {
      type: "tuple",
      components: [
        { name: "currency0", type: "address" },
        { name: "currency1", type: "address" },
        { name: "fee", type: "uint24" },
        { name: "tickSpacing", type: "int24" },
        { name: "hooks", type: "address" },
      ],
    },
  ],
  [
    {
      currency0: weth,
      currency1: apxs,
      fee,
      tickSpacing,
      hooks,
    },
  ],
);

const poolId = keccak256(encoded);

console.log("=================================");
console.log("     APXS / WETH POOL ID");
console.log("=================================");
console.log("currency0:", weth);
console.log("currency1:", apxs);
console.log("fee:", fee);
console.log("tickSpacing:", tickSpacing);
console.log("hooks:", hooks);
console.log("---------------------------------");
console.log("Pool ID:", poolId);
console.log("=================================");
