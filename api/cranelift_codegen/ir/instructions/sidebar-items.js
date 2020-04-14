initSidebarItems({"enum":[["BranchInfo","Information about branch and jump instructions."],["CallInfo","Information about call instructions."],["InstructionData",""],["InstructionFormat","An instruction format"],["Opcode","An instruction opcode."],["ResolvedConstraint","The type constraint on a value argument once the controlling type variable is known."]],"struct":[["OpcodeConstraints","Value type constraints for a given opcode."],["ValueTypeSet","A value type set describes the permitted set of types for a type variable."],["VariableArgs","A variable list of `Value` operands used for function call arguments and passing arguments to basic blocks."]],"type":[["ValueList","Some instructions use an external list of argument values because there is not enough space in the 16-byte `InstructionData` struct. These value lists are stored in a memory pool in `dfg.value_lists`."],["ValueListPool","Memory pool for holding value lists. See `ValueList`."]]});