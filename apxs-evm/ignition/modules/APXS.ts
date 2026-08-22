import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const APXSModule = buildModule("APXSModule", (m) => {
  const initialOwner = m.getAccount(0);

  const apxs = m.contract("APXS", [initialOwner]);

  return { apxs };
});

export default APXSModule;