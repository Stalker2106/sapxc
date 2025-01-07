# SAP-X Compiler

This is a **SAP-X compiler** written in Rust, designed to compile assembly code for the **SAP-X** architecture. The SAP-X architecture is a simplified X-bit (8 per default) computer model designed by Ben Eater for educational purposes. This compiler allows you to compile assembly programs for this architecture, converting them into binary machine code.

## SAP-X Assembly Language

The SAP-X architecture supports a small set of instructions, each represented by an opcode. The following table describes the opcodes in the SAP-X assembly language:

| Instruction | Binary Representation | Operands | Description                                        |
|-------------|------------------------|----------|----------------------------------------------------|
| `NOP`       | `0000`                 | 0        | No operation (do nothing)                         |
| `LDA`       | `0001`                 | 1        | Load value into the accumulator                   |
| `ADD`       | `0010`                 | 1        | Add value to the accumulator                      |
| `SUB`       | `0011`                 | 1        | Subtract value from the accumulator               |
| `MUL`       | `0100`                 | 1        | Multiply accumulator by value                     |
| `OUT`       | `0101`                 | 0        | Output the value of the accumulator               |
| `HLT`       | `0110`                 | 0        | Halt the program                                  |
| `MI`        | `0111`                 | 1        | Multiply immediate value to accumulator           |
| `RO`        | `1000`                 | 1        | Rotate the accumulator right                      |
| `RI`        | `1001`                 | 1        | Rotate the accumulator left                       |
| `IO`        | `1010`                 | 1        | Input a value into the accumulator                |
| `II`        | `1011`                 | 1        | Input a value and invert the accumulator          |
| `AO`        | `1100`                 | 1        | Add value to the output accumulator               |
| `AI`        | `1101`                 | 1        | Add immediate value to the output accumulator     |
| `EO`        | `1110`                 | 1        | Execute output accumulator                        |
| `SU`        | `1111`                 | 1        | Subtract immediate value from the output          |

### Operand Description
- The **operands** column specifies the number of operands that the instruction takes. Some instructions take no operands (e.g., `NOP`, `HLT`), while others take one operand (e.g., `LDA`, `ADD`).
- The operand values are typically X-bit (Word size - Opcode size) integers (or addresses depending on the operation).

## Configurable Word Size

The SAP-X compiler is highly configurable and can target any word size. By default, it operates with an 8-bit word size, but it can be customized to handle different word sizes, allowing it to be adapted to various processor architectures and use cases.
You can edit its values in `config.rs` file.

## Usage

To compile an assembly file for the SAP-X architecture, Download the latest release and simply run:

    sapxc myasmfile.sapx

Or, you can clone this repository and run the compiler directly from sources with cargo:

    cargo run myasmfile.sapx
    

## Build

To build sapxc, simply use cargo built-in release command

    cargo build --release
