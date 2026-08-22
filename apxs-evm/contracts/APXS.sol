// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract APXS is ERC20 {
    uint256 public constant MAX_SUPPLY = 1_000_000_000 * 10 ** 8;

    constructor(address initialHolder) ERC20("Apraxus", "APXS") {
        require(initialHolder != address(0), "APXS: zero holder");

        _mint(initialHolder, MAX_SUPPLY);
    }

    function decimals() public pure override returns (uint8) {
        return 8;
    }

    function burn(uint256 amount) external {
        _burn(msg.sender, amount);
    }
}
