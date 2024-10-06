// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {AxelarExecutable} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutable.sol";
import {IAxelarGateway} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol";
import {IAxelarGasService} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol";
import {StringToAddress, AddressToString} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/libs/AddressString.sol";
import "./Lock.sol"; // Import the DataConsumerV3 contract

contract Axelar is AxelarExecutable {
    using StringToAddress for string;
    using AddressToString for address;

    IAxelarGasService public immutable gasService;
    DataConsumerV3 public dataConsumer; // Instance of the DataConsumerV3 contract
    string public chainName; // name of the chain this contract is deployed to

    struct Message {
        string sender;
        string message;
    }

    Message public storedMessage; // message received from _execute

    constructor(
        address gateway_,
        address gasReceiver_,
        string memory chainName_,
        address dataConsumerAddress // Pass the address of the DataConsumerV3 contract
    ) AxelarExecutable(gateway_) {
        gasService = IAxelarGasService(gasReceiver_);
        chainName = chainName_;
        dataConsumer = DataConsumerV3(dataConsumerAddress); // Initialize the DataConsumerV3 contract
    }

    // Function to send BTC/USD price to another chain
    function sendBTCtoUSD(
        string calldata destinationChain,
        string calldata destinationAddress
    ) external payable {
        require(msg.value > 0, "Gas payment is required");

        // 1. Retrieve BTC/USD price from DataConsumerV3 contract
        int btcPrice = dataConsumer.getBTCtoUSD();

        // 2. Generate GMP payload
        string memory priceMessage = string(abi.encodePacked("BTC/USD Price: ", _intToString(btcPrice)));
        bytes memory executeMsgPayload = abi.encode(msg.sender.toString(), priceMessage);
        bytes memory payload = _encodePayloadToCosmWasm(executeMsgPayload);

        // 3. Pay for gas
        gasService.payNativeGasForContractCall{value: msg.value}(
            address(this),
            destinationChain,
            destinationAddress,
            payload,
            msg.sender
        );

        // 4. Make GMP call
        gateway.callContract(destinationChain, destinationAddress, payload);
    }

    // Function to send ETH/USD price to another chain
    function sendETHtoUSD(
        string calldata destinationChain,
        string calldata destinationAddress
    ) external payable {
        require(msg.value > 0, "Gas payment is required");

        // 1. Retrieve ETH/USD price from DataConsumerV3 contract
        int ethPrice = dataConsumer.getETHtoUSD();

        // 2. Generate GMP payload
        string memory priceMessage = string(abi.encodePacked("ETH/USD Price: ", _intToString(ethPrice)));
        bytes memory executeMsgPayload = abi.encode(msg.sender.toString(), priceMessage);
        bytes memory payload = _encodePayloadToCosmWasm(executeMsgPayload);

        // 3. Pay for gas
        gasService.payNativeGasForContractCall{value: msg.value}(
            address(this),
            destinationChain,
            destinationAddress,
            payload,
            msg.sender
        );

        // 4. Make GMP call
        gateway.callContract(destinationChain, destinationAddress, payload);
    }

    function _encodePayloadToCosmWasm(bytes memory executeMsgPayload) internal view returns (bytes memory) {
        // Schema for encoding payload to CosmWasm
        bytes memory argValues = abi.encode(
            chainName,
            address(this).toString(),
            executeMsgPayload
        );

    
        string[] memory argumentNameArray = new string[](3);
        argumentNameArray[0] = "source_chain";
        argumentNameArray[1] = "source_address";
        argumentNameArray[2] = "payload";

        
        string[] memory abiTypeArray = new string[](3);
        // string[] memory argumentNameArray = new string[](3);
        abiTypeArray[0] = "string";
        abiTypeArray[1] = "string";
        abiTypeArray[2] = "bytes";

        bytes memory gmpPayload = abi.encode(
            "receive_message_evm",
            argumentNameArray,
            abiTypeArray,
            argValues
        );

        return abi.encodePacked(
            bytes4(0x00000001),
            gmpPayload
        );
    }

    function _execute(
        string calldata /*sourceChain*/,
        string calldata /*sourceAddress*/,
        bytes calldata payload
    ) internal override {
        (string memory sender, string memory message) = abi.decode(payload, (string, string));
        storedMessage = Message(sender, message);
    }

    // Helper function to convert int to string
    function _intToString(int256 value) internal pure returns (string memory) {
        if (value == 0) {
            return "0";
        }
        int256 temp = value;
        uint256 digits;
        bool negative = false;
        if (value < 0) {
            negative = true;
            temp = -temp;
        }
        while (temp != 0) {
            digits++;
            temp /= 10;
        }
        bytes memory buffer;
        if (negative) {
            buffer = new bytes(digits + 1);
            buffer[0] = '-';
        } else {
            buffer = new bytes(digits);
        }
        temp = value < 0 ? -value : value;
        while (temp != 0) {
            digits -= 1;
            buffer[digits + (negative ? 1 : 0)] = bytes1(uint8(48 + uint256(temp % 10)));
            temp /= 10;
        }
        return string(buffer);
    }
}
