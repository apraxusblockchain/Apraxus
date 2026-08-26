import { network } from "hardhat";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();

const POSITION_MANAGER =
  "0xAc631556d3d4019C95769033B5E719dD77124BAc";

const TX =
  "0x2d84ff43437cbb317f95e903444c0830252d31fe4520746cf994ad002ce7ae50";

const receipt = await publicClient.getTransactionReceipt({
  hash: TX,
});

console.log("=================================");
console.log("    MINT RECEIPT VERIFICATION");
console.log("=================================");
console.log("Transaction:", TX);
console.log("Status:", receipt.status);
console.log("Block:", receipt.blockNumber.toString());
console.log("Logs:", receipt.logs.length);
console.log("=================================");

for (let i = 0; i < receipt.logs.length; i++) {
  const log = receipt.logs[i];

  console.log(`LOG ${i}`);
  console.log("Address:", log.address);
  console.log("Topics:", log.topics);
  console.log("---------------------------------");
}
