import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const APXSModule = buildModule("APXSModule", (m) => {
  const initialHolder = m.getAccount(0);

  const apxs = m.contract("APXS", [initialHolder]);

  return { apxs };
});

export default APXSModule;
