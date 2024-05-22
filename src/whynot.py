from enum import Enum, auto
from typing import Optional
from dataclasses import dataclass
import sys
from pathlib import Path


class Op(Enum):
    OpEnd = 0
    OpIncDp = 1
    OpDecDp = 2
    OpIncVal = 3
    OpDecVal = 4
    OpOut = 5
    OpIn = 6
    OpJmpFwd = 7
    OpJmpBck = 8
    Unknown = 9


class Statuses(Enum):
    Success = 0
    Failure = 1


PROGRAM_SIZE = 4096
STACK_SIZE = 512
DATA_SIZE = 65535


class StackErrors(Enum):
    OverFlow = 0
    UnderFlow = 1


class Stack:
    def __init__(self) -> None:
        self.ptr: int = 0
        self.arr: list[int] = [0 for _ in range(STACK_SIZE)]

    def push(self, a: int) -> tuple[None, Optional[StackErrors]]:

        if self.ptr >= STACK_SIZE:
            return None, StackErrors.OverFlow

        self.arr[self.ptr] = a
        self.ptr += 1

        return None, None

    def pop(self) -> tuple[Optional[int], Optional[StackErrors]]:
        if self.ptr == 0:
            return None, StackErrors.UnderFlow

        self.ptr -= 1

        return self.arr[self.ptr], None

    def is_empty(self) -> bool:
        match self.ptr:
            case 0:
                return True
            case _:
                return False

    def is_full(self) -> bool:
        match self.ptr:
            case 512:
                return True
            case _:
                return False


@dataclass
class Instruction:
    operator: Op
    operand: int


class Program:
    def __init__(self) -> None:
        self.instructions = [Instruction(Op.Unknown, 0) for _ in range(PROGRAM_SIZE)]
        self.stack = Stack()
        self.print_buffer = ""

    def compile(self, fp: str) -> Statuses:
        pc = 0
        for c in fp.strip():
            if not pc < PROGRAM_SIZE:
                break

            match c:
                case ">":
                    self.instructions[pc].operator = Op.OpIncDp
                case "<":
                    self.instructions[pc].operator = Op.OpDecDp
                case "+":
                    self.instructions[pc].operator = Op.OpIncVal
                case "-":
                    self.instructions[pc].operator = Op.OpDecVal
                case ".":
                    self.instructions[pc].operator = Op.OpOut
                case ",":
                    self.instructions[pc].operator = Op.OpIn
                case "[":
                    self.instructions[pc].operator = Op.OpJmpFwd

                    if self.stack.is_full():
                        return Statuses.Failure

                    self.stack.push(pc)

                case "]":
                    if self.stack.is_empty():
                        return Statuses.Failure

                    result = self.stack.pop()

                    if result[0] is not None:
                        jmp_pc = result[0]
                    else:
                        return Statuses.Failure

                    self.instructions[pc].operator = Op.OpJmpBck
                    self.instructions[pc].operand = jmp_pc
                    self.instructions[jmp_pc].operand = pc

                case _:
                    pc -= 1

            pc += 1

        if not self.stack.is_empty() or (pc == PROGRAM_SIZE):
            return Statuses.Failure

        self.instructions[pc].operator = Op.OpEnd

        print("Compilation successful!")

        return Statuses.Success

    def execute(self) -> Statuses:
        data = [0 for _ in range(DATA_SIZE)]
        pc = 0
        ptr = 0

        while (self.instructions[pc].operator != Op.OpEnd) and (ptr < DATA_SIZE):

            match self.instructions[pc].operator:
                case Op.OpIncDp:
                    ptr += 1
                case Op.OpDecDp:
                    ptr -= 1
                case Op.OpIncVal:
                    data[ptr] += 1
                case Op.OpDecVal:
                    data[ptr] -= 1
                case Op.OpOut:
                    self.print_buffer += chr(data[ptr])
                case Op.OpIn:
                    data[ptr] = int(input())
                case Op.OpJmpFwd:
                    if not data[ptr]:
                        pc = self.instructions[pc].operand
                case Op.OpJmpBck:
                    if data[ptr]:
                        pc = self.instructions[pc].operand
                case _:
                    return Statuses.Failure

            pc += 1

        match ptr != DATA_SIZE:
            case True:
                print(self.print_buffer)
                return Statuses.Success
            case False:
                return Statuses.Failure


class Error(Enum):
    FailedToExecute = auto
    FailedToCompile = auto


def main() -> tuple[None, Optional[Error]]:
    args = sys.argv

    if len(args) != 2 or not Path(args[1]).exists():
        print(f"Usage: {args[0]} filename\n")

    with open(args[1], "r") as f:
        buffer = f.read()
        prog = Program()

        match prog.compile(buffer):
            case Statuses.Success:
                match prog.execute():
                    case Statuses.Success:
                        return (
                            None,
                            None,
                        )
                    case Statuses.Failure:
                        return None, Error.FailedToExecute

            case Statuses.Failure:
                return None, Error.FailedToCompile


if __name__ == "__main__":
    ok, err = main()

    if err is not None:
        print(f"Error: {err}!")
        sys.exit(1)

    print("Program executed successfully!")
    sys.exit(0)
